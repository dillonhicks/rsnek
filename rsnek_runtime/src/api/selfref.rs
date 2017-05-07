use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::ops::Deref;

use num::Zero;

use ::api::error::Error;
use ::api::{RtObject, WeakRtObject};
use ::api::result::ObjectResult;
use ::objects::native;


/// A trait that must be implemented on a refcount wrapper type
/// in order to provide the necessary behavior for a value to
/// contain a reference to itself.
///
/// One of the admittedly weak areas of rust is cyclic data structures because of the strong
/// lifetime guarantees. In order to get a self reference to work in a way that will ensure
/// that the type attached to the selfref is properly deallocated the following must be true:
///
/// 1. The stored reference must be weak so the strong count can go to 0 (see std::rc::Rc)
/// 2. The selfref can only be set after the containing structure is created and therefore
///    must be set after the struct is moved into the appropriate `Box` and `Rc` containers.
///    So the field must be
pub trait SelfRef: Sized {
    fn strong_count(&self) -> native::Integer;
    fn weak_count(&self) -> native::Integer;
    fn set(&self, &RtObject);
    fn get(&self) -> WeakRtObject;
    fn upgrade(&self) -> ObjectResult;
}


/// A wrapper around a value with its own reference count
/// in the runtime.
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct RefCountedValue<T, V: SelfRef> {
    pub value: T,
    pub rc: V,
}

/// RefCount struct that holds a mutable and optional weakref
/// this is so instances can have a reference to their own RefCount
///
pub struct RefCount(pub RefCell<Option<WeakRtObject>>);

impl Clone for RefCount {
    fn clone(&self) -> Self {
        match *self.0.borrow().deref() {
            Some(ref weak) => RefCount(RefCell::new(Some(weak.clone()))),
            None => RefCount(RefCell::new(None)),
        }
    }
}

impl SelfRef for RefCount {
    /// Unwrap the optional type and proxy to the underlying WeakRtObject if present
    /// otherwise return 0.
    fn strong_count(&self) -> native::Integer {
        match *self.0.borrow().deref() {
            Some(ref weak) => weak.strong_count(),
            None => native::Integer::zero(),
        }
    }

    /// Unwrap the optional type and proxy to the underlying WeakRtObject if present
    /// otherwise return 0.
    fn weak_count(&self) -> native::Integer {
        let count: native::Integer;
        // use a scope to ensure that the borrow is dropped
        {
            count = match *self.0.borrow().deref() {
                Some(ref weak) => weak.weak_count(),
                None => native::Integer::zero(),
            }
        }

        count
    }

    /// Set the `SelfRef` from strong `RtObject` by cloning and downgrading that
    /// reference.
    fn set(&self, selfref: &RtObject) {
        let mut rc = self.0.borrow_mut();
        match *rc {
            None => *rc = Some(selfref.downgrade()),
            // TODO: {T97} Make this an error and not a panic
            _ => panic!("Tried to overwrite self reference"),
        }
    }

    /// Return a clone of of the backing `WeakRtObject`
    fn get(&self) -> WeakRtObject {
        match *self.0.borrow().deref() {
            Some(ref weak) => weak.clone(),
            // TODO: {T97} Make this an error and not a runtime panic
            None => panic!("Unable to retrieve unset weak object reference"),
        }
    }

    /// Take the `WeakRtObject` backing the `SelfRef` and attempt to upgrade it
    /// to its strong version `RtObject`.
    fn upgrade(&self) -> ObjectResult {
        match *self.0.borrow().deref() {
            Some(ref weak) => weak.clone().upgrade(),
            None => Err(Error::runtime("Cannot upgrade a None weakref!")),
        }
    }
}


/// Display the strong and weak reference counts
impl Debug for RefCount {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RefCount(strong: {}, weak: {})", self.strong_count(), self.weak_count())
    }
}


/// Default to an inner cell value of None meaning that the selfref has not been set
impl Default for RefCount {
    fn default() -> Self {
        RefCount(RefCell::new(None))
    }
}
