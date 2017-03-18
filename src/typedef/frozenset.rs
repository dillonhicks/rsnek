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


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::model::PyObject for FrozenSetObject {}
impl object::model::PyBehavior for FrozenSetObject {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
