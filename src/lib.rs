#![feature(associated_consts)]
#![feature(rustc_private)]
#[cfg_attr(rsnek_debug, any(test, feature="rsnek-debug"))]

/// # Notes and TODOS
///
///  - TODO: Consider having objects implement an `pub fn alloc(self, rt: Runtime) -> ObjectRef`
///
extern crate num;
extern crate arena;

#[macro_use]
pub mod log;
pub mod typedef;
pub mod opcode;
pub mod instruction;
pub mod result;
pub mod heap;
pub mod runtime;
pub mod error;



#[cfg(test)]
mod tests {

    use std;
    use std::ops::Deref;
    use super::typedef::object::{self, ObjectMethods, ObjectRef};

    use super::runtime::Runtime;
    use super::typedef::integer;
    use super::typedef::builtin::Builtin;
    use super::typedef::integer::IntegerObject;
    use super::typedef::float::FloatObject;
    use super::typedef::string::StringObject;
    use super::typedef::tuple::TupleObject;

    use std::borrow::Borrow;

//    macro_rules! builtin {
//    ($objref:expr) => ($objref.0.borrow())
//}

    // Just try to init the runtime
    #[test]
    fn test_runtime_constructors() {
        Runtime::new(None);
        Runtime::new(Some(2048));
    }

    // Create integer object on the stack and try to allocate it
    // in the runtime.
    #[test]
    fn test_alloc_integer() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);
        println!("{:?}", runtime);
    }

    #[test]
    fn test_int_add_int_equals_int() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one2 : ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref : &std::cell::RefCell<Builtin> = one.0.borrow();
        let two = one_ref.borrow().deref().add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    #[test]
    fn test_int_add_float_equals_float() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let one_float= FloatObject::new(1.0).as_builtin().as_object_ref();
        let onef : ObjectRef = runtime.alloc(one_float.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let mut two_float: ObjectRef;
        {
            let one_ref: &std::cell::RefCell<Builtin> = one.0.borrow();
            // Alloc will happen here because a new float will be created during the addition
            two_float = one_ref.borrow().deref().add(&mut runtime, &onef).unwrap();
            assert_eq!(runtime.heap_size(), 3);
        }

        let two_ref : &std::cell::RefCell<Builtin>  = two_float.0.borrow();
        two_ref.borrow().deref().float().unwrap();

        println!("{:?}", runtime);
    }

    #[test]
    fn test_add_string_string_equals_string() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = StringObject::from_str("1").as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one = StringObject::from_str("2").as_builtin().as_object_ref();
        let one2 : ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref : &std::cell::RefCell<Builtin> = one.0.borrow();
        let two = one_ref.borrow().deref().add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    #[test]
    fn test_tuple() {
        let mut runtime = Runtime::new(None);
        let mut t1 = vec![
            IntegerObject::new_i64(0).as_builtin().as_object_ref(),
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
            FloatObject::new(5.0).as_builtin().as_object_ref(),
        ];

        t1 = t1.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
        assert_eq!(runtime.heap_size(), t1.len());

        let tuple_obj = TupleObject::new(&t1).as_builtin().as_object_ref();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        assert_eq!(runtime.heap_size(), t1.len() + 1);

        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let mut t2 = vec![
                StringObject::from_str("Hello").as_objref(),
                StringObject::from_str("World").as_objref()
            ];
            t2 = t2.iter().map(|objref| runtime.alloc(objref.clone()).unwrap()).collect();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 1);

            let tuple2 = runtime.alloc(TupleObject::new(&t2).as_builtin().as_object_ref()).unwrap();
            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 2);

            let x: &std::cell::RefCell<Builtin> = tuple2.0.borrow();
            tuple_3 = x.borrow().deref().add(&mut runtime, &tuple).unwrap();

            assert_eq!(runtime.heap_size(), t1.len() + t2.len() + 3);
        }

        println!("{}", tuple_3);
        println!("{:?}", runtime);
    }
}