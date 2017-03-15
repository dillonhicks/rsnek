use std::rc::Rc;
use std::fmt;

use runtime::Runtime;
use result::RuntimeResult;
use error::{Error, ErrorType};

use super::objectref;
use super::objectref::{Object, ObjectRef};
use super::integer::IntegerObject;
use super::float::FloatObject;
use super::string::StringObject;
use super::tuple::TupleObject;
use super::list::ListObject;
use super::complex::ComplexObject;
use super::set::SetObject;
use super::frozenset::FrozenSetObject;
use super::dictionary::DictionaryObject;


pub type CastResult<T: Object> = Result<T, Error>;


#[derive(Clone, Debug)]
pub enum Builtin {
    Integer(IntegerObject),
    Float(FloatObject),
    String(StringObject),
    Tuple(TupleObject),
    List(ListObject),
    Set(SetObject),
    FrozenSet(FrozenSetObject),
    Dictionary(DictionaryObject),
    Complex(ComplexObject),

    // Not yet implemented
    Boolean(()),
    Object(()),
    Function(()),
    Method(()),
    None(()),
    Module(())

//    Type(TypeObject),
//    Meta(MetaObject),
//    None(NoneObject),
}

impl objectref::ObjectBinaryOperations for Builtin {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        match self {
            &Builtin::Integer(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Float(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::String(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Tuple(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::List(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Set(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::FrozenSet(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Complex(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Dictionary(ref lhs) => lhs.add(runtime, rhs),
            ref other => Err(Error(ErrorType::Type, "Add not implemented for type"))
        }
    }

    fn subtract(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        match self {
            &Builtin::Integer(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Float(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::String(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Tuple(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::List(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Set(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::FrozenSet(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Complex(ref lhs) => lhs.add(runtime, rhs),
            &Builtin::Dictionary(ref lhs) => lhs.add(runtime, rhs),
            ref other => Err(Error(ErrorType::Type, "Add not implemented for type"))
        }
    }
}

impl objectref::TypeInfo for Builtin {}

impl objectref::Object for Builtin {}


impl Builtin {
    pub fn int(&self) -> CastResult<&IntegerObject> {
        match *self {
            Builtin::Integer(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not an IntegerObject"))
        }
    }

    pub fn float(&self) -> CastResult<&FloatObject> {
        match *self {
            Builtin::Float(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a FloatObject"))
        }
    }

    pub fn tuple(&self) -> CastResult<&TupleObject> {
        match *self {
            Builtin::Tuple(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a TupleObject"))
        }
    }

    pub fn list(&self) -> CastResult<&ListObject> {
        match *self {
            Builtin::List(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a ListObject"))
        }
    }

    pub fn string(&self) -> CastResult<&StringObject> {
        match *self {
            Builtin::String(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a StringObject"))
        }
    }

    pub fn as_object_ref(self) -> ObjectRef {
        ObjectRef::new(self)
    }
}


impl objectref::ToType<Builtin> for Builtin {
    #[inline]
    fn to(self) -> Builtin {
        self
    }
}

impl objectref::ToType<ObjectRef> for Builtin {
    #[inline]
    fn to(self) -> ObjectRef {
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
            &Builtin::List(ref obj) => write!(f, "{}", obj),
            _ => write!(f, "BuiltinType")
        }
    }
}
