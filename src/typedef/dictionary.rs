use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

pub type Dictionary = ();

#[derive(Clone, Debug)]
pub struct DictionaryObject {
    value: Dictionary
}

impl objectref::ObjectBinaryOperations for DictionaryObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

use object;

impl object::api::Identifiable for DictionaryObject {}

impl object::api::Hashable for DictionaryObject {}