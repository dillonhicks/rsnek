use result::RuntimeResult;
use runtime::Runtime;
use object;

use super::objectref::{self, ObjectRef};

pub type FrozenSet = ();

#[derive(Clone, Debug)]
pub struct FrozenSetObject {
    value: FrozenSet
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///    Python Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::model::PythonObject for FrozenSetObject {}
impl object::api::Identifiable for FrozenSetObject {}
impl object::api::Hashable for FrozenSetObject {}

impl objectref::ObjectBinaryOperations for FrozenSetObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
