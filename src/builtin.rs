extern crate num;

use runtime::Runtime;
use object;
use object::{Object, ObjectRef};
use result::RuntimeResult;
use integer::IntegerObject;
use float::FloatObject;
use std::rc::Rc;
use std::fmt;
use error::{Error, ErrorType};

pub type CastResult<T: Object> = Result<T, Error>;


#[derive(Clone,Debug)]
pub enum Builtin {
    Integer(IntegerObject),
    Float(FloatObject),
    Object,
    Type,
    Meta,
    None,
}

impl object::ObjectMethods for Builtin {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        match self {
            &Builtin::Integer(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Float(ref lhs) => lhs.add(runtime, rhs),
            ref other => Err(Error(ErrorType::Type, "Add not implemented for type"))
        }
    }
}

impl object::TypeInfo for Builtin {

}

impl object::Object for Builtin {

}


impl Builtin {
    pub fn int(&self) -> CastResult<&IntegerObject> {
        match *self {
            Builtin::Integer(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not an integer object"))
        }
    }

    pub fn as_object_ref(self) -> ObjectRef {
        ObjectRef::new(self)
    }
}


impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Builtin::Integer(ref obj) => write!(f, "{}", obj),
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


