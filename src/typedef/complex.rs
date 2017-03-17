use result::RuntimeResult;
use runtime::Runtime;
use object;
use super::objectref::{self, ObjectRef};

pub type Complex = ();

#[derive(Clone, Debug)]
pub struct ComplexObject {
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

impl object::model::PythonObject for ComplexObject {}
impl object::api::Identifiable for ComplexObject {}
impl object::api::Hashable for ComplexObject {}