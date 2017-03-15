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

use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;


#[derive(Clone)]
pub struct Tuple(Box<[ObjectRef]>);


#[derive(Clone)]
pub struct TupleObject {
    pub value: Tuple
}


impl objectref::ObjectBinaryOperations for TupleObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::Tuple(ref obj) => {
                let mut array = self.value.0.clone().into_vec();
                array.extend_from_slice(obj.value.0.as_ref());
                let new_tuple = TupleObject::new(&array).as_builtin();
                runtime.alloc(new_tuple.as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to Tuple"))
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

impl objectref::TypeInfo for TupleObject {

}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.as_ref())
    }
}

impl fmt::Display for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Tuple {
    fn from_vec(vector: &Vec<ObjectRef>) -> Tuple {
        Tuple(vector.clone().into_boxed_slice())
    }
}

impl TupleObject {


    pub fn new(value: &Vec<ObjectRef>) -> TupleObject {
        let tuple = TupleObject {
            value: Tuple::from_vec(&value.clone()),
        };

        return tuple
    }

    pub fn as_builtin(self) -> Builtin {
        return Builtin::Tuple(self)
    }
}

impl objectref::ToType<Builtin> for TupleObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Tuple(self)
    }
}

impl objectref::ToType<ObjectRef> for TupleObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


impl objectref::Object for TupleObject {

}


use object;
impl object::api::Identity for TupleObject{}
