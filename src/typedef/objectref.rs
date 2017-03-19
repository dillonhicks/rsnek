/// Wrapper for the runtime housekeeping
use std;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

use runtime::Runtime;
use result::RuntimeResult;
use object;
use object::model::PyBehavior;


use typedef::builtin;
use typedef::builtin::Builtin;
use typedef::integer::IntegerObject;
use typedef::float::FloatObject;
use typedef::string::StringObject;
use typedef::tuple::TupleObject;
use typedef::list::ListObject;
use typedef::native;


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      Types and Structs
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
pub struct ObjectRef(pub native::RuntimeRef);
pub struct WeakObjectRef(pub native::RuntimeWeakRef);


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl ObjectRef {
    #[inline]
    pub fn new(value: Builtin) -> ObjectRef {
        ObjectRef(Rc::new(Box::new(value)))
    }

    /// Downgrade the ObjectRef to a WeakObjectRef
    pub fn downgrade(self) -> WeakObjectRef {
        WeakObjectRef(Rc::downgrade(&self.0))
    }
}


impl Default for WeakObjectRef {
    fn default() -> WeakObjectRef {
        WeakObjectRef(Weak::default())
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///        stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+


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

impl Hash for ObjectRef {
    fn hash<H: Hasher>(&self, s: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


impl Borrow<Box<Builtin>> for ObjectRef {
    fn borrow(&self) -> &Box<Builtin> {
        self.0.borrow()
    }
}


impl std::fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let builtin: &Box<Builtin> = self.0.borrow();

        match builtin.deref() {
            &Builtin::Integer(ref obj) => {
                write!(f, "<Integer({}): {:?}>", obj, obj as *const IntegerObject)
            }
            &Builtin::Float(ref obj) => {
                write!(f, "<Float({}) {:?}>", obj, obj as *const FloatObject)
            }
            &Builtin::String(ref obj) => {
                write!(f, "<String(\"{}\") {:?}>", obj, obj as *const StringObject)
            }
            &Builtin::Tuple(ref obj) => {
                write!(f, "<Tuple({}) {:?}>", obj, obj as *const TupleObject)
            }
            &Builtin::List(ref obj) => write!(f, "<List({}) {:?}>", obj, obj as *const ListObject),
            other => write!(f, "<{:?} {:?}>", other, other.native_identity()),
        }
    }
}

impl std::fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let builtin: &Box<Builtin> = self.0.borrow();

        match builtin.deref() {
            &Builtin::Integer(ref obj) => {
                write!(f, "<Integer({}): {:?}>", obj, obj as *const IntegerObject)
            }
            &Builtin::Float(ref obj) => {
                write!(f, "<Float({}) {:?}>", obj, obj as *const FloatObject)
            }
            &Builtin::String(ref obj) => {
                write!(f, "<String({}) {:?}>", obj, obj as *const StringObject)
            }
            &Builtin::Tuple(ref obj) => {
                write!(f, "<Tuple({}) {:?}>", obj, obj as *const TupleObject)
            }
            &Builtin::List(ref obj) => write!(f, "<List({}) {:?}>", obj, obj as *const ListObject),
            other => write!(f, "<{:?} {:?}>", other, other as *const _),
        }
    }
}


/// Convert between types the intermediate Builtin/ObjectRef/Etc types
pub trait ToRtWrapperType<T> {
    fn to(self) -> T;
}


// TODO: move to object::api if needed
pub trait TypeInfo {}


// TODO: Move me to object::api
pub trait RtObject:
    object::model::PyBehavior +
    ToRtWrapperType<ObjectRef> +
    ToRtWrapperType<Builtin> +
    Display where Self: std::marker::Sized {

}
