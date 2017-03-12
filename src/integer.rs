use num;
use num::FromPrimitive;
use num::BigInt;

use object;
use object::ObjectRef;
use builtin::Builtin;
use result::RuntimeResult;
use std::rc::{Weak,Rc};
use runtime::Runtime;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use float::FloatObject;
use error::{Error, ErrorType};


pub type Integer = BigInt;



#[derive(Clone,Debug)]
pub struct IntegerObject {
    pub value: Integer,
}


impl object::ObjectMethods for IntegerObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd

        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::Integer(ref obj) => {
                let new_number = IntegerObject::new_bigint(&self.value + &obj.value).as_builtin();
                runtime.push_object(new_number.as_object_ref())
            },
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::add_integer(obj, &self)?.as_builtin();
                runtime.push_object(new_number.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to int"))
        }
    }

}

impl object::TypeInfo for IntegerObject {

}


impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl IntegerObject {


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

    pub fn as_builtin(self) -> Builtin {
        return Builtin::Integer(self)
    }
}

impl object::Object for IntegerObject {
}