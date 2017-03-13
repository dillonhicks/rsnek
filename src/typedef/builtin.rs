use std::rc::Rc;
use std::fmt;

use runtime::Runtime;
use result::RuntimeResult;
use error::{Error, ErrorType};

use super::object;
use super::object::{Object, ObjectRef};
use super::integer::IntegerObject;
use super::float::FloatObject;
use super::string::StringObject;
use super::tuple::TupleObject;

pub type CastResult<T: Object> = Result<T, Error>;


#[derive(Clone, Debug)]
pub enum Builtin {
    Integer(IntegerObject),
    Float(FloatObject),
    String(StringObject),
    Tuple(TupleObject),
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
            &Builtin::String(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Tuple(ref lhs) => lhs.add(runtime, rhs),
            ref other => Err(Error(ErrorType::Type, "Add not implemented for type"))
        }
    }
}

impl object::TypeInfo for Builtin {}

impl object::Object for Builtin {}


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
            &Builtin::Float(ref obj) => write!(f, "{}", obj),
            &Builtin::String(ref obj) => write!(f, "{}", obj),
            &Builtin::Tuple(ref obj) => write!(f, "{}", obj),
            _ => write!(f, "BuiltinType")
        }
    }
}
