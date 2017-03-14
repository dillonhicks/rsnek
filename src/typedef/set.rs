use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

pub type Set = ();

#[derive(Clone,Debug)]
pub struct SetObject{
    value: Set

}

impl objectref::ObjectBinaryOperations for SetObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}