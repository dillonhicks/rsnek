use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

use object;


pub type Dictionary = ();

#[derive(Clone, Debug)]
pub struct DictionaryObject {
    value: Dictionary
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl DictionaryObject {}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///    Python Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::model::PyObject for DictionaryObject {}
impl object::model::PyBehavior for DictionaryObject {}

