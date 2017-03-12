extern crate rattlesnake;

use rattlesnake::opcode::OpCode;
use rattlesnake::instruction::*;
use rattlesnake::runtime::Runtime;
use rattlesnake::snektype::{BuiltinType, SnekInteger};
use rattlesnake::integer::IntegerObject;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use std::fs::File;
use std::borrow::Borrow;
use rattlesnake::object::ObjectMethods;

static FILEPATH: &'static str = "tests/python/e0002_add_x_plus_y.pyc";


fn main() {

    let f = File::open(FILEPATH).unwrap();

    for byte in f.bytes() {
        let b = byte.unwrap();
        println!("{} = {:?}", b, OpCode::from(b));
    }

    //println!("{:?}", PYFUNC_ADD);

    {
        let runtime = Runtime::new(None);
        let mut rt = RefCell::new(runtime.clone());


        let zero = rt.borrow_mut().reserve(Rc::new(BuiltinType::Integer(IntegerObject::new(
            RefCell::new(runtime), 10
        ))));
        let value = zero.unwrap();
        println!("{}", value);
        println!("{}", value.as_integer_object_ref());
        let val2 = value.as_integer_object_ref();
        let val4 = value.as_integer_object_ref().clone();
        let val3 = val2.add(Rc::new(val4)).unwrap();

        println!("{}", val3);

        for x in 1..10 {
            println!("{}", x);


        }
    }
}
