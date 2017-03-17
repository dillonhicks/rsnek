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



/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///    Python Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::model::PythonObject for DictionaryObject {}
impl object::api::Identifiable for DictionaryObject {}
impl object::api::Hashable for DictionaryObject {}

impl objectref::ObjectBinaryOperations for DictionaryObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

