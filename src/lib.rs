#![feature(associated_consts)]
#![feature(rustc_private)]
#![feature(const_fn)]
#[cfg_attr(rsnek_debug, any(test, feature = "rsnek-debug"))]
/// # Notes and TODOS
///
///  - TODO: Consider having objects implement an `pub fn alloc(self, rt: Runtime) -> ObjectRef`
///         - Generally cleanup the `as_builtin()` and `as_object_ref()` shit
///  - TODO: Determine if there is a generic way to increment rc's for adds to collections
///  - TODO: Some types need a weak ref back to their own RC in order to return back the proper
///          runtime result. We may also need an id or something else in order to look up
///          the in place modifyables down the chain.
extern crate num;
extern crate arena;

#[macro_use]
pub mod log;
pub mod typedef;
pub mod result;
pub mod heap;
pub mod runtime;
pub mod error;
pub mod object;


#[cfg(test)]
mod tests {
    use std;
    use std::ops::Deref;
    use super::typedef::objectref::{self, ObjectBinaryOperations, ObjectRef};

    use super::runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use super::typedef::integer;
    use super::typedef::builtin::Builtin;
    use super::typedef::integer::{Integer, IntegerObject};
    use super::typedef::float::FloatObject;
    use super::typedef::string::StringObject;
    use super::typedef::tuple::TupleObject;
    use super::typedef::list::ListObject;
    use super::typedef::boolean::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use object::api::Identifiable;
    use std::borrow::Borrow;


    // Just try to init the runtime
    #[test]
    fn test_runtime_constructors() {
        {
            let rt = Runtime::new(None);
            assert_eq!(rt.heap_size(), 0);
            assert_eq!(rt.heap_capacity(), DEFAULT_HEAP_CAPACITY);
        }
        {
            let rt = Runtime::new(Some(2048));
            assert_eq!(rt.heap_size(), 0);
            assert_eq!(rt.heap_capacity(), 2048);
        }
    }


    #[test]
    fn test_int_add_float_equals_float() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let one_float = FloatObject::new(1.0).as_builtin().as_object_ref();
        let onef: ObjectRef = runtime.alloc(one_float.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let mut two_float: ObjectRef;
        {
            let one_ref: &Box<Builtin> = one.0.borrow();
            // Alloc will happen here because a new float will be created during the addition
            two_float = one_ref.add(&mut runtime, &onef).unwrap();
            assert_eq!(runtime.heap_size(), 3);
        }

        let two_ref: &Box<Builtin> = two_float.0.borrow();
        two_ref.float().unwrap();

        println!("{:?}", runtime);
    }

    #[test]
    fn test_add_string_string_equals_string() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = StringObject::from_str("1").as_builtin().as_object_ref();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one = StringObject::from_str("2").as_builtin().as_object_ref();
        let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref: &Box<Builtin> = one.0.borrow();
        let two = one_ref.add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    #[test]
    fn test_tuple_add_tuple_equals_tuple() {
        let mut runtime = Runtime::new(None);
        let mut t1 = vec![IntegerObject::new_i64(0).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(1).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(2).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(3).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(4).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(5).as_builtin().as_object_ref(),
                          FloatObject::new(0.0).as_builtin().as_object_ref(),
                          FloatObject::new(1.0).as_builtin().as_object_ref(),
                          FloatObject::new(2.0).as_builtin().as_object_ref(),
                          FloatObject::new(3.0).as_builtin().as_object_ref(),
                          FloatObject::new(4.0).as_builtin().as_object_ref(),
                          FloatObject::new(5.0).as_builtin().as_object_ref()];

        t1 = t1.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
        assert_eq!(runtime.heap_size(), t1.len());

        let tuple_obj = TupleObject::new(&t1).as_builtin().as_object_ref();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        assert_eq!(runtime.heap_size(), t1.len() + 1);

        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let mut t2 = vec![StringObject::from_str("Hello").as_objref(),
                              StringObject::from_str("World").as_objref()];

            t2 = t2.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 1);

            let tuple2 = runtime.alloc(TupleObject::new(&t2).as_builtin().as_object_ref()).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 2);

            let x: &Box<Builtin> = tuple2.0.borrow();
            tuple_3 = x.add(&mut runtime, &tuple).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 3);
        }

        println!("{}", tuple_3);
        println!("{:?}", runtime);
    }

    ///
    ///  List Tests
    ///

    /// list + list => list (and does not allocate new heap objects)
    #[test]
    fn test_list_add_list_equals_list_no_alloc() {
        let mut runtime = Runtime::new(None);
        let mut t1 = vec![IntegerObject::new_i64(0).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(1).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(2).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(3).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(4).as_builtin().as_object_ref(),
                          IntegerObject::new_i64(5).as_builtin().as_object_ref(),
                          FloatObject::new(0.0).as_builtin().as_object_ref(),
                          FloatObject::new(1.0).as_builtin().as_object_ref(),
                          FloatObject::new(2.0).as_builtin().as_object_ref(),
                          FloatObject::new(3.0).as_builtin().as_object_ref(),
                          FloatObject::new(4.0).as_builtin().as_object_ref(),
                          FloatObject::new(5.0).as_builtin().as_object_ref()];

        t1 = t1.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
        assert_eq!(runtime.heap_size(), t1.len());

        let tuple_obj = ListObject::new(&t1).as_builtin().as_object_ref();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        assert_eq!(runtime.heap_size(), t1.len() + 1);

        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let mut t2 = vec![StringObject::from_str("Hello").as_objref(),
                              StringObject::from_str("World").as_objref()];
            t2 = t2.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 1);

            let tuple2 = runtime.alloc(ListObject::new(&t2).as_builtin().as_object_ref()).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 2);

            let x: &Box<Builtin> = tuple2.0.borrow();
            tuple_3 = x.deref().add(&mut runtime, &tuple).unwrap();

            assert_eq!(runtime.heap_size(),
                       t1.len() + t2.len() + 2,
                       "list+list unexpectedly allocated extra heap");
        }

        println!("{}", tuple_3);
        println!("{:?}", runtime);
    }
}
