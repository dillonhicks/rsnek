use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

pub type Complex = ();

#[derive(Clone,Debug)]
pub struct ComplexObject{
    value: Complex
}

impl objectref::ObjectBinaryOperations for ComplexObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

use object;
impl object::api::Identity for ComplexObject{}
impl object::api::Hashable for ComplexObject{}