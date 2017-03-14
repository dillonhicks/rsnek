use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak,Rc};

use num::{FromPrimitive, BigInt};

use error::{Error, ErrorType};
use result::RuntimeResult;
use runtime::Runtime;

use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;


pub type Integer = BigInt;



#[derive(Clone,Debug)]
pub struct IntegerObject {
    pub value: Integer,
}


impl objectref::ObjectBinaryOperations for IntegerObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd

        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::Integer(ref obj) => {
                let new_number = IntegerObject::new_bigint(&self.value + &obj.value).as_builtin();
                runtime.alloc(new_number.as_object_ref())
            },
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::add_integer(obj, &self)?.as_builtin();
                runtime.alloc(new_number.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to int"))
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

impl objectref::TypeInfo for IntegerObject {

}


impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl IntegerObject {
    #[inline]
    pub fn new_i64(value: i64) -> IntegerObject {
        let integer = IntegerObject {
            value: BigInt::from(value),
        };

        return integer
    }

    pub fn new_bigint(value: BigInt) -> IntegerObject {
        let integer = IntegerObject {
            value: BigInt::from(value),
        };

        return integer
    }

    #[inline]
    pub fn to_builtin(self) -> Builtin {
        return Builtin::Integer(self)
    }


    #[inline]
    pub fn as_builtin(self) -> Builtin {
        return Builtin::Integer(self)
    }


    #[inline]
    pub fn as_object_ref(self) -> ObjectRef {
        return Builtin::Integer(self).as_object_ref()
    }
}

impl objectref::ToType<Builtin> for IntegerObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Integer(self)
    }
}

impl objectref::ToType<ObjectRef> for IntegerObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}

impl objectref::Object for IntegerObject {
}

