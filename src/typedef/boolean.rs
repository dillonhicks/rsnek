use std;
use std::fmt;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

use object;
use object::api;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};

use super::builtin;
use super::builtin::Builtin;
use super::objectref;
use super::objectref::ObjectRef;
use super::native;
use num::ToPrimitive;


pub type Boolean = native::Integer;


#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BooleanObject {
    value: Boolean,
}


pub const SINGLETON_TRUE: BooleanObject = BooleanObject { value: 1 };
pub const SINGLETON_FALSE: BooleanObject = BooleanObject { value: 0 };
pub const SINGLETON_TRUE_BUILTIN: builtin::Builtin = builtin::Builtin::Boolean(SINGLETON_TRUE);
pub const SINGLETON_FALSE_BUILTIN: builtin::Builtin = builtin::Builtin::Boolean(SINGLETON_FALSE);

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl BooleanObject {
    pub fn new_u64(value: u64) -> BooleanObject {
        match value {
            0 => SINGLETON_FALSE,
            _ => SINGLETON_TRUE,
        }
    }

    pub fn new_f64(value: f64) -> BooleanObject {
        match value {
            0.0 => SINGLETON_FALSE,
            _ => SINGLETON_TRUE,
        }
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     RtObject Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl objectref::RtObject for BooleanObject {}
impl objectref::TypeInfo for BooleanObject {}
impl object::api::Identifiable for BooleanObject {
    fn op_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        let builtin = borrowed.borrow();

        match self.native_equals(builtin.deref()) {
            Ok(value) => {
                if value {
                    rt.alloc(ObjectRef::new(SINGLETON_TRUE_BUILTIN))
                } else {
                    rt.alloc(ObjectRef::new(SINGLETON_FALSE_BUILTIN))
                }
            },
            Err(err) => Err(err)
        }
    }

    fn native_equals(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        let equality = match other {
            &Builtin::Boolean(ref obj) => self.value == obj.value,
            &Builtin::Integer(ref obj) => BooleanObject::new_u64(obj.value.to_u64().unwrap_or_default()) == *self,
            &Builtin::Float(ref obj) => BooleanObject::new_f64(obj.value) == *self,
            _ => self.value == 1,
        };

        Ok(equality)
    }
}
impl object::api::Hashable for BooleanObject {}


impl objectref::ToRtWrapperType<builtin::Builtin> for BooleanObject {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Boolean(self)
    }
}

impl objectref::ToRtWrapperType<objectref::ObjectRef> for BooleanObject {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Boolean(self))
    }
}

impl objectref::ObjectBinaryOperations for BooleanObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///        stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for BooleanObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
