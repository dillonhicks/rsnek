use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak,Rc};

use num::{BigInt, FromPrimitive};

use result::RuntimeResult;
use runtime::Runtime;
use error::{Error, ErrorType};

use super::object;
use super::object::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;


#[derive(Clone)]
pub struct List(Vec<ObjectRef>);


#[derive(Clone)]
pub struct ListObject {
    pub value: List
}


impl object::ObjectMethods for ListObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::List(ref obj) => {
                obj.value.0
                let new_tuple = ListObject::new(&array).as_builtin();
                runtime.alloc(new_tuple.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to List"))
        }
    }

}

impl object::TypeInfo for ListObject {

}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.as_ref())
    }
}

impl fmt::Display for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl ListObject {


    pub fn new(value: &Vec<ObjectRef>) -> ListObject {
        let tuple = ListObject {
            value: List::from_vec(&value.clone()),
        };

        return tuple
    }

    pub fn as_builtin(self) -> Builtin {
        return Builtin::List(self)
    }
}

impl object::ToType<Builtin> for ListObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::List(self)
    }
}

impl object::ToType<ObjectRef> for ListObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


impl object::Object for ListObject {

}
