use std::fmt;
use std::collections::HashSet;

use result::RuntimeResult;
use runtime::Runtime;

use object;
use super::objectref::{self, ObjectRef};
use super::builtin;

pub type Set = HashSet<ObjectRef>;

#[derive(Clone, Debug)]
pub struct SetObject {
    value: Set,
}

impl SetObject {
    #[inline]
    pub fn new() -> SetObject {
        SetObject { value: Set::new() }
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     RtObject Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for SetObject {}
impl objectref::TypeInfo for SetObject {}
impl object::api::Identifiable for SetObject {}
impl object::api::Hashable for SetObject {}

impl objectref::ToRtWrapperType<builtin::Builtin> for SetObject {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Set(self)
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for SetObject {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Set(self))
    }
}

impl objectref::ObjectBinaryOperations for SetObject {
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
///
impl fmt::Display for SetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
