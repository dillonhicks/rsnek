
use num;
use num::FromPrimitive;

use object;
use builtin::Builtin;
use result::RuntimeResult;
use std::rc::{Weak,Rc};
use runtime::Runtime;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use object::ObjectRef;
use std::ops::Deref;
use num::BigInt;
use num::ToPrimitive;
use builtin::CastResult;
use integer::IntegerObject;
use error::{Error, ErrorType};


pub type Float = f64;


#[derive(Clone,Debug)]
pub struct FloatObject {
    value: Float
}


impl object::ObjectMethods for FloatObject {

    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();

        match borrowed.borrow_mut().deref() {
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::new(self.value + obj.value).as_builtin();
                runtime.push_object(new_number.as_object_ref())
            },
            &Builtin::Integer(ref obj) => {
                let new_number = FloatObject::add_integer(&self, &obj)?.as_builtin();
                runtime.push_object(new_number.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to float"))
        }
    }
}


impl fmt::Display for FloatObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl FloatObject {
    pub fn new(value: f64) -> FloatObject {
        return FloatObject {
            value: value
        }
    }

    pub fn as_builtin(self) -> Builtin {
        return Builtin::Float(self)
    }


    pub fn add_integer(float: &FloatObject, integer: &IntegerObject) -> CastResult<FloatObject> {
        match integer.value.to_f64() {
            Some(other) => Ok(FloatObject::new(float.value + other)),
            None => Err(Error(ErrorType::Overflow, "Floating Point Overflow"))
        }
    }
}

impl object::TypeInfo for FloatObject {
}

impl object::Object for FloatObject {
}