use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::rc::{Weak, Rc};
use std::ops::Deref;

use num;
use num::{FromPrimitive, BigInt, ToPrimitive};

use error::{Error, ErrorType};
use result::RuntimeResult;
use runtime::Runtime;

use super::objectref;
use super::builtin::Builtin;
use super::builtin::CastResult;
use super::integer::IntegerObject;
use super::objectref::ObjectRef;


pub type Float = f64;


#[derive(Clone,Debug)]
pub struct FloatObject {
    value: Float
}


impl objectref::ObjectBinaryOperations for FloatObject {

    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();

        match borrowed.borrow_mut().deref() {
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::new(self.value + obj.value).as_builtin();
                runtime.alloc(new_number.as_object_ref())
            },
            &Builtin::Integer(ref obj) => {
                let new_number = FloatObject::add_integer(&self, &obj)?.as_builtin();
                runtime.alloc(new_number.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to float"))
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}


impl fmt::Display for FloatObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl objectref::ToType<Builtin> for FloatObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Float(self)
    }
}

impl objectref::ToType<ObjectRef> for FloatObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}

impl FloatObject {
    pub fn new(value: f64) -> FloatObject {
        return FloatObject {
            value: value
        }
    }

    #[deprecated]
    pub fn as_builtin(self) -> Builtin {
        return Builtin::Float(self)
    }

    #[deprecated]
    pub fn as_object_ref(self) -> ObjectRef{
        self.as_builtin().as_object_ref()
    }

    pub fn add_integer(float: &FloatObject, integer: &IntegerObject) -> CastResult<FloatObject> {
        match integer.value.to_f64() {
            Some(other) => Ok(FloatObject::new(float.value + other)),
            None => Err(Error(ErrorType::Overflow, "Floating Point Overflow"))
        }
    }
}


impl objectref::TypeInfo for FloatObject {}

impl objectref::Object for FloatObject {}

use object;
impl object::api::Identity for FloatObject{}
impl object::api::Hashable for FloatObject{}