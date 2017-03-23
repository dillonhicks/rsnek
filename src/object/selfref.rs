use std::fmt::{Debug, Formatter, Result};
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;

use num::Zero;

use result::{RuntimeResult, NativeResult};
use error::Error;
use runtime::Runtime;

use typedef::objectref::{WeakObjectRef, ObjectRef};
use typedef::native;


/// A trait that must be implemented on a refcount wrapper type
/// in order to provide the necessary behavior for a value to
/// contain a reference to itself.
pub trait SelfRef: Sized {
    fn strong_count(&self) -> native::Integer;
    fn weak_count(&self) -> native::Integer;
    fn set(&self, &ObjectRef);
    fn upgrade(&self) -> RuntimeResult;
}


/// A wrapper around a value with its own reference count
/// in the runtime.
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct RefCountedValue<T, V: SelfRef> {
    pub value: T,
    pub rc: V,
}


/// RefCount struct that holds a mutable and optional weakref
/// this is so instances can have a reference to their own RefCount
#[derive(Clone)]
pub struct RefCount(pub RefCell<Option<WeakObjectRef>>);


impl SelfRef for RefCount {
    fn strong_count(&self) -> native::Integer {
        match *self.0.borrow().deref() {
            Some(ref weak) => weak.upgrade().unwrap().refcount(),
            None => native::Integer::zero()
        }
    }

    fn weak_count(&self) -> native::Integer {
        match *self.0.borrow().deref() {
            Some(ref weak) => weak.weak_refcount(),
            None => native::Integer::zero()
        }
    }

    fn set(&self, selfref: &ObjectRef) {
        let mut rc = self.0.borrow_mut();
        match *rc {
            None => *rc = Some(selfref.downgrade()),
            _ => panic!("Tried to overwrite self reference")

        }
    }

    fn upgrade(&self) -> RuntimeResult {
        match *self.0.borrow_mut().deref() {
            None => Err(Error::runtime("Explosions!")),
            Some(_) => Err(Error::runtime("Explosions!")),
        }
    }
}


impl Debug for RefCount {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}/{}", self.strong_count(), self.weak_count())
    }
}


impl Default for RefCount {
    fn default() -> Self {
        RefCount(RefCell::new(None))
    }
}