extern crate num;

use object;
use result::{Result, InterpreterError};
use integer::IntegerObject;
use std::rc::Rc;
use std::fmt;

pub type SnekInteger = num::BigInt;
pub type SnekFloat = Box<f64>;
pub type SnekComplex = num::Complex<f64>;


#[derive(Clone)]
pub enum BuiltinType {
    Integer(IntegerObject),
    Float,
    Object,
    Type,
    Meta,
    None,
}

impl<T: object::Object> object::ObjectMethods<T> for BuiltinType {
    fn add(&self, _: Rc<T>) -> Result<Rc<BuiltinType>> {
        Err(InterpreterError{message: "Unimplemented!"})
    }
}

impl object::TypeInfo for BuiltinType {
    fn snek_type(&self) -> BuiltinType {
        BuiltinType::Meta
    }
}

impl object::Object for BuiltinType {

}

impl BuiltinType {
    pub fn as_integer_object_ref(&self) -> &IntegerObject {
        if let BuiltinType::Integer(ref obj) = *self { &obj } else { panic!("Not an IntegerObject") }
    }
}


impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BuiltinType::Integer(ref obj) => write!(f, "{}", obj),
            _ => write!(f, "BuiltinType")
        }
    }
}

#[derive(Debug,Clone)]
pub enum FunctionType {
    //    const Inquery: fn(Object) -> isize;
    //    const Unary: fn(Object) -> Object;
    //    const Binary: fn(Object, Object) -> Object;
    //    const Ternary: fn(Object, Object, Object) -> Object;
    //    Inquery(fn(Object) -> Result),
    //    Unary(fn(Object) -> Result),
    //    Binary(fn(Object, Object) -> Result),
    //    Ternary(fn(Object, Object, Object) -> Result)
}


