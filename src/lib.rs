#![feature(associated_consts)]
#![feature(rustc_private)]
#![feature(const_fn)]
/// # Notes and TODOS
///
///  - TODO: Consider having objects implement an `pub fn alloc(self, rt: Runtime) -> ObjectRef`
///         - Generally cleanup the `as_builtin()` and `as_object_ref()` shit
///
/// - TODO: Determine if there is a generic way to increment rc's for adds to collections
///
///  - TODO: Some types need a weak ref back to their own RC in order to return back the proper
///          runtime result. We may also need an id or something else in order to look up
///          the in place modifyables down the chain.
///
///  - TODO: Consider a lighter weight NativeBuiltin union/enum for polymorphic native type cases
///
extern crate num;

#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod macros;

pub mod typedef;
pub mod result;
pub mod heap;
pub mod runtime;
pub mod error;
pub mod object;


#[allow(unused_variables,non_snake_case,unused_imports,unused_mut)]
#[cfg(test)]
mod tests {
    use std;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};

    use typedef::objectref::ToRtWrapperType;
    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::integer::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use object::model::PyBehavior;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
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

        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let one_float: ObjectRef = FloatObject::new(1.0).to();
        let onef: ObjectRef = runtime.alloc(one_float.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let mut two_float: ObjectRef;
        {
            let one_ref: &Box<Builtin> = one.0.borrow();
            // Alloc will happen here because a new float will be created during the addition
            two_float = one_ref.op_add(&mut runtime, &onef).unwrap();
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

        let one_object: ObjectRef = StringObject::from_str("1").to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one: ObjectRef = StringObject::from_str("2").to();
        let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref: &Box<Builtin> = one.0.borrow();
        let two = one_ref.op_add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    #[test]
    fn test_tuple_add_tuple_equals_tuple() {
        let mut runtime = Runtime::new(None);
        let mut t1: Vec<ObjectRef> = vec![IntegerObject::new_i64(0).to(),
                                          IntegerObject::new_i64(1).to(),
                                          IntegerObject::new_i64(2).to(),
                                          IntegerObject::new_i64(3).to(),
                                          IntegerObject::new_i64(4).to(),
                                          IntegerObject::new_i64(5).to(),
                                          FloatObject::new(0.0).to(),
                                          FloatObject::new(1.0).to(),
                                          FloatObject::new(2.0).to(),
                                          FloatObject::new(3.0).to(),
                                          FloatObject::new(4.0).to(),
                                          FloatObject::new(5.0).to()];

        t1 = t1.iter().map(|objref: &ObjectRef| runtime.alloc(objref.clone()).unwrap()).collect();
        assert_eq!(runtime.heap_size(), t1.len());

        let tuple_obj: ObjectRef = TupleObject::new(&t1).to();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        assert_eq!(runtime.heap_size(), t1.len() + 1);

        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let mut t2: Vec<ObjectRef> = vec![StringObject::from_str("Hello").to(),
                                              StringObject::from_str("World").to()];

            t2 = t2.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 1);

            let tuple2 = runtime.alloc(TupleObject::new(&t2).to()).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 2);

            let x: &Box<Builtin> = tuple2.0.borrow();
            tuple_3 = x.op_add(&mut runtime, &tuple).unwrap();
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
        let mut t1 = vec![IntegerObject::new_i64(0).to(),
                          IntegerObject::new_i64(1).to(),
                          IntegerObject::new_i64(2).to(),
                          IntegerObject::new_i64(3).to(),
                          IntegerObject::new_i64(4).to(),
                          IntegerObject::new_i64(5).to(),
                          FloatObject::new(0.0).to(),
                          FloatObject::new(1.0).to(),
                          FloatObject::new(2.0).to(),
                          FloatObject::new(3.0).to(),
                          FloatObject::new(4.0).to(),
                          FloatObject::new(5.0).to()];

        t1 = t1.iter().map(|objref: &ObjectRef| runtime.alloc(objref.clone()).unwrap()).collect();
        assert_eq!(runtime.heap_size(), t1.len());

        let tuple_obj: ObjectRef = ListObject::new(&t1).to();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        assert_eq!(runtime.heap_size(), t1.len() + 1);

        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let mut t2: Vec<ObjectRef> = vec![StringObject::from_str("Hello").to(),
                                              StringObject::from_str("World").to()];
            t2 = t2.iter()
                .map(|objref: &ObjectRef| runtime.alloc(objref.clone()).unwrap())
                .collect();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 1);

            let tuple2 = runtime.alloc(ListObject::new(&t2).to()).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 2);

            let x: &Box<Builtin> = tuple2.0.borrow();
            tuple_3 = x.deref().op_add(&mut runtime, &tuple).unwrap();

            assert_eq!(runtime.heap_size(),
                       t1.len() + t2.len() + 2,
                       "list+list unexpectedly allocated extra heap");
        }

        println!("{}", tuple_3);
        println!("{:?}", runtime);
    }
}
