/// Wrapper for the runtime housekeeping
use std;
use std::rc::{Rc, Weak};

use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::hash::{Hash, Hasher};

use num::Zero;

use error::{Error, ErrorType};
use result::RuntimeResult;
use object::method::{Id, Next, StringCast};

use typedef::builtin::Builtin;
use typedef::native;


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      Types and Structs
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
pub struct ObjectRef(pub native::RuntimeRef);
pub struct WeakObjectRef(pub native::RuntimeWeakRef);


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//       Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl ObjectRef {
    #[inline]
    pub fn new(value: Builtin) -> ObjectRef {
        ObjectRef(Rc::new(Box::new(value)))
    }

    /// Downgrade the ObjectRef to a WeakObjectRef
    pub fn downgrade(&self) -> WeakObjectRef {
        WeakObjectRef(Rc::downgrade(&self.0))
    }

    pub fn strong_count(&self) -> native::Integer {
        native::Integer::from(Rc::strong_count(&self.0))
    }

    pub fn weak_count(&self) -> native::Integer {
        native::Integer::from(Rc::weak_count(&self.0))
    }

    pub fn to_string(&self) -> native::String {
        let boxed: &Box<Builtin> = self.0.borrow();
        match boxed.native_str() {
            Ok(string) => string,
            Err(err) => format!("{}", self)
        }
    }
}

impl Default for WeakObjectRef {
    fn default() -> WeakObjectRef {
        WeakObjectRef(Weak::default())
    }
}


impl WeakObjectRef {
    pub fn weak_count(&self) -> native::Integer {
        let count: native::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return native::Integer::zero(),
            };

            count = objref.weak_count();
            drop(objref)
        }

        count
    }

    pub fn strong_count(&self) -> native::Integer {
        let count: native::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return native::Integer::zero(),
            };

            count = objref.strong_count();
            drop(objref)
        }

        count
    }

    pub fn upgrade(&self) -> RuntimeResult {
        match Weak::upgrade(&self.0) {
            None => Err(Error::runtime("Attempted to create a strong ref to a an object with no existing refs")),
            Some(objref) => Ok(ObjectRef(objref)),
        }
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl std::cmp::PartialEq for ObjectRef {
    fn eq(&self, rhs: &ObjectRef) -> bool {
        let lhs_box: &Box<Builtin> = self.0.borrow();

        let rhs_box: &Box<Builtin> = rhs.0.borrow();
        *lhs_box.deref() == *rhs_box.deref()
    }
}


impl std::cmp::Eq for ObjectRef {}


impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        ObjectRef((self.0).clone())
    }
}


impl Clone for WeakObjectRef {
    fn clone(&self) -> Self {
        WeakObjectRef((self.0).clone())
    }
}

impl Hash for ObjectRef {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, s: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}

impl Hash for WeakObjectRef {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


impl std::fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl std::fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl Iterator for ObjectRef {
    type Item = ObjectRef;

    fn next(&mut self) -> Option<Self::Item> {
        let mut boxed: &Box<Builtin> = self.0.borrow();
        match boxed.deref() {
            &Builtin::Iter(ref iterator) => {
                match iterator.native_next() {
                    Ok(objref) => Some(objref),
                    Err(Error(ErrorType::StopIteration, _)) => None,
                    Err(_) => panic!("Iterator logic fault")
                }
            }
            _ => None
        }
    }
}
