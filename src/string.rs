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
use std;

pub type String = std::string::String;



#[derive(Clone,Debug)]
pub struct StringObject {
    pub value: String,
}


impl object::ObjectMethods for StringObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd

        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::String(ref obj) => {
                let new_string = self.value.clone() + obj.value.borrow();
                let new_number = StringObject::new(new_string).as_builtin();
                runtime.push_object(new_number.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to str"))
        }
    }

}

impl object::TypeInfo for StringObject {

}


impl fmt::Display for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl StringObject {


    pub fn new(value: std::string::String) -> StringObject {
        let string = StringObject {
            value: value,
        };

        return string
    }

    pub fn as_builtin(self) -> Builtin {
        return Builtin::String(self)
    }
}

impl object::Object for StringObject {

}