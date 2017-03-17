use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};

use num::{BigInt, FromPrimitive};

use object;
use result::RuntimeResult;
use runtime::Runtime;
use error::{Error, ErrorType};

use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;

pub type String = std::string::String;


#[derive(Clone, Debug)]
pub struct StringObject {
    pub value: String,
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl StringObject {
    pub fn from_str(value: &'static str) -> StringObject {
        return StringObject::new(value.to_string())
    }

    pub fn new(value: std::string::String) -> StringObject {
        let string = StringObject {
            value: value,
        };

        return string
    }

    #[deprecated]
    pub fn as_builtin(self) -> Builtin {
        return Builtin::String(self)
    }

    #[deprecated]
    pub fn as_objref(self) -> ObjectRef {
        return ObjectRef::new(self.as_builtin())
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl object::model::PythonObject for StringObject {}
impl objectref::RtObject for StringObject {}
impl objectref::TypeInfo for StringObject {}
impl object::api::Identifiable for StringObject {}
impl object::api::Hashable for StringObject {}

impl objectref::ObjectBinaryOperations for StringObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();
        match builtin.deref() {
            &Builtin::String(ref obj) => {
                let new_string = StringObject::new(self.value.clone() + obj.value.borrow());
                runtime.alloc(ObjectRef::new(Builtin::String(new_string)))
            },

            _ => Err(Error(ErrorType::Type, "TypeError cannot add to str"))
        }
    }
    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}


impl objectref::ToRtWrapperType<Builtin> for StringObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::String(self)
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for StringObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
