//#[macro_use]
//extern crate rattlesnake;
//
//use rattlesnake::opcode::OpCode;
//use rattlesnake::instruction::*;
//use rattlesnake::runtime::Runtime;
//use rattlesnake::typedef::builtin::{Builtin};
//use rattlesnake::typedef::integer::IntegerObject;
//use rattlesnake::typedef::float::FloatObject;
//use rattlesnake::typedef::string::StringObject;
//use std::rc::Rc;
//use std::cell::RefCell;
//use std::io::prelude::*;
//use std::fs::File;
//use std::borrow::Borrow;
//use rattlesnake::typedef::object::{ObjectRef,ObjectMethods};
//use std::ops::Deref;
//use std::string::String;
//
//static FILEPATH: &'static str = "tests/python/e0002_add_x_plus_y.pyc";
//

fn main() {

//    let f = File::open(FILEPATH).unwrap();
//    debug!("Potato!");
//
//    for byte in f.bytes() {
//        let b = byte.unwrap();
//        println!("{} = {:?}", b, OpCode::from(b));
////    }
//
//    let mut runtime = Runtime::new(None);
//
//    // int + int => int
//    let one = IntegerObject::new_i64(1).as_builtin().as_object_ref();
//    runtime.push_object(one.clone());
//
//    for x in 1..10 {
//        println!("{}", x);
//        let integer = IntegerObject::new_i64(x).as_builtin().as_object_ref();
//        let value: ObjectRef = runtime.push_object(integer.clone()).unwrap();
//        let b: &RefCell<Builtin> = value.0.borrow();
//        let int_plus_one = b.borrow_mut().deref().add(&mut runtime, &one).unwrap();
//
//        println!("{}", integer);
//        println!("{}", int_plus_one);
//        //runtime.debug_references();
//        println!();
//    }
//    //runtime.debug_references();
//
//    // float + float => float
//    let fone = FloatObject::new(1.0).as_builtin().as_object_ref();
//    runtime.push_object(fone.clone());
//
////    for x in 1..10 {
////        println!("{}", x);
////        let float = FloatObject::new(x as f64).as_builtin().as_object_ref();
////        let value: ObjectRef = runtime.push_object(float.clone()).unwrap();
////        let b: &RefCell<Builtin> = value.0.borrow();
////        let float_plus_one = b.borrow_mut().deref().add(&mut runtime, &fone).unwrap();
////
////        println!("{}", float);
////        println!("{}", float_plus_one);
////        runtime.debug_references();
////        println!();
////    }
////    runtime.debug_references();
////
////    // float + int => float
////    for x in 30..40 {
////        println!("{}", x);
////        let float = FloatObject::new(x as f64).as_builtin().as_object_ref();
////        let value: ObjectRef = runtime.push_object(float.clone()).unwrap();
////        let b: &RefCell<Builtin> = value.0.borrow();
////        let float_plus_one = b.borrow_mut().deref().add(&mut runtime, &one).unwrap();
////
////        println!("{}", float);
////        println!("{}", float_plus_one);
////        runtime.debug_references();
////        println!();
////
////    }
////
////    // int + float => float
////    for x in 45..103 {
////        println!("{}", x);
////        let float = IntegerObject::new_i64(x).as_builtin().as_object_ref();
////        let value: ObjectRef = runtime.push_object(float.clone()).unwrap();
////        let b: &RefCell<Builtin> = value.0.borrow();
////        let int_plus_fone = b.borrow_mut().deref().add(&mut runtime, &fone).unwrap();
////
////        println!("{}", float);
////        println!("{}", int_plus_fone);
////        println!();
////    }
//
//    for x in 1..1000000 {
//        let float = IntegerObject::new_i64(x).as_builtin().as_object_ref();
//        let value: ObjectRef = match runtime.push_object(float.clone()) {
//            Ok(obj) => obj,
//            Err(e) => {
//                //println!("Error happened! {:?}", e);
//                //println!("Forcing Garbage Collection...");
//                runtime.gc_object_refs();
//                continue
//            }
//        };
//
//        let b: &RefCell<Builtin> = value.0.borrow();
//        let int_plus_fone = b.borrow_mut().deref().add(&mut runtime, &fone).unwrap();
//        //println!("{}", int_plus_fone);
//    }
//
//    let h = StringObject::new("Hello".to_string()).as_builtin().as_object_ref();
//    let value = runtime.push_object(h.clone()).unwrap();
//
//    let w = StringObject::new(" World".to_string()).as_builtin().as_object_ref();
//    runtime.push_object(w.clone()).unwrap();
//
//    let b: &RefCell<Builtin> = value.0.borrow();
//    let h_plus_w = b.borrow_mut().deref().add(&mut runtime, &w).unwrap();
//    println!("{}", h_plus_w);
//    println!("{} {}", h, w);
    //    //b.borrow_mut().deref().add(&mut runtime, &fone)?
    println!("things")
}


