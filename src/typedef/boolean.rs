
use object;
use super::objectref;


type Boolean = bool;

#[derive(Clone,Debug)]
struct BooleanObject {
    value: Boolean
}


impl objectref::ObjectBinaryOperations for DictionaryObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

impl object::api::Identity for DictionaryObject{}
impl object::api::Hashable for DictionaryObject{}
