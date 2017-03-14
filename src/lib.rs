#![feature(associated_consts)]
#![feature(rustc_private)]

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
    fn test_init_runtime() {
        Runtime::new(None);
    }

    // Create integer object on the stack and try to allocate it
    // in the runtime.
    #[test]
    fn test_alloc_integer() {
        let mut runtime = Runtime::new(None);
        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();
    }

    #[test]
    fn test_int_add_int_equals_int() {
        let mut runtime = Runtime::new(None);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        let another_one = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one2 : ObjectRef = runtime.alloc(another_one.clone()).unwrap();

        let one_ref : &std::cell::RefCell<Builtin> = one.0.borrow();
        let two = one_ref.borrow().deref().add(&mut runtime, &another_one).unwrap();
    }

    #[test]
    fn test_int_add_float_equals_float() {
        let mut runtime = Runtime::new(None);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one : ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        let one_float= FloatObject::new(1.0).as_builtin().as_object_ref();
        let onef : ObjectRef = runtime.alloc(one_float.clone()).unwrap();

        let mut two_float: ObjectRef;
        {
            let one_ref: &std::cell::RefCell<Builtin> = one.0.borrow();
            two_float = one_ref.borrow().deref().add(&mut runtime, &onef).unwrap();
        }

        let two_ref : &std::cell::RefCell<Builtin>  = two_float.0.borrow();
        two_ref.borrow().deref().float().unwrap();
    }

    #[test]
    fn test_tuple() {

        let mut runtime = Runtime::new(None);
        let t1 = vec![
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

        let tuple_obj = TupleObject::new(&t1).as_builtin().as_object_ref();
        let tuple: ObjectRef = runtime.alloc(tuple_obj.clone()).unwrap();
        println!("{}", tuple);

        let mut tuple_3: ObjectRef;
        {
            let t2 = vec![
                StringObject::from_str("Hello").as_objref(),
                StringObject::from_str("World").as_objref()
            ];

            let tuple2 = runtime.alloc(TupleObject::new(&t2).as_builtin().as_object_ref()).unwrap();

            let x: &std::cell::RefCell<Builtin> = tuple2.0.borrow();
            tuple_3 = x.borrow().deref().add(&mut runtime, &tuple).unwrap();
        }

        println!("{}", tuple_3);

        //
//        let mut runtime = Runtime::new(None);
//
//        // int + int => int
//        let one = IntegerObject::new_i64(1).as_builtin().as_object_ref();
//        runtime.push_object(one.clone());
//
//        for x in 1..10 {
//            println!("{}", x);
//            let integer = IntegerObject::new_i64(x).as_builtin().as_object_ref();
//            let value: ObjectRef = runtime.push_object(integer.clone()).unwrap();
//            let b: &RefCell<Builtin> = value.0.borrow();
//            let int_plus_one = b.borrow_mut().deref().add(&mut runtime, &one).unwrap();
//
//            println!("{}", integer);
//            println!("{}", int_plus_one);
//            //runtime.debug_references();
//            println!();
//        }
//        //runtime.debug_references();
//
//        // float + float => float
//        let fone = FloatObject::new(1.0).as_builtin().as_object_ref();
//        runtime.push_object(fone.clone());
    }
}