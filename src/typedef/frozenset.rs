
use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

pub type FrozenSet = ();

#[derive(Clone,Debug)]
pub struct FrozenSetObject{
    value: FrozenSet
}

impl objectref::ObjectBinaryOperations for FrozenSetObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}