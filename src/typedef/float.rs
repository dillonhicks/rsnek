use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::rc::{Weak, Rc};
use std::ops::Deref;

use num;
use num::{FromPrimitive, BigInt, ToPrimitive};

use object;
use error::{Error, ErrorType};
use result::RuntimeResult;
use runtime::Runtime;

use object::model::PyBehavior;
use super::objectref;
use super::builtin::Builtin;
use super::builtin::CastResult;
use super::integer::IntegerObject;
use super::objectref::ObjectRef;
use typedef::objectref::ToRtWrapperType;


pub type Float = f64;


#[derive(Clone, Debug)]
pub struct FloatObject {
    pub value: Float
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl FloatObject {
    pub fn new(value: f64) -> FloatObject {
        return FloatObject {
            value: value
        }
    }

    pub fn add_integer(float: &FloatObject, integer: &IntegerObject) -> CastResult<FloatObject> {
        match integer.value.to_f64() {
            Some(other) => Ok(FloatObject::new(float.value + other)),
            None => Err(Error(ErrorType::Overflow, "Floating Point Overflow"))
        }
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///    Python Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for FloatObject {}
impl object::model::PyObject for FloatObject {}
impl object::model::PyBehavior for FloatObject {

    fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::new(self.value + obj.value);
                let num_ref: ObjectRef = new_number.to();
                runtime.alloc(num_ref)
            },
            &Builtin::Integer(ref obj) => {
                let new_number = FloatObject::add_integer(&self, &obj)?;
                let num_ref: ObjectRef = new_number.to();
                runtime.alloc(num_ref)
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to float"))
        }
    }


}


impl objectref::ToRtWrapperType<Builtin> for FloatObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Float(self)
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for FloatObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for FloatObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}



