use result::RuntimeResult;
use runtime::Runtime;
use object;
use super::objectref::{self, ObjectRef};

pub type Complex = ();

#[derive(Clone, Debug)]
pub struct ComplexObject {
    value: Complex
}

impl object::model::PyObject for ComplexObject {}
impl object::model::PyBehavior for ComplexObject {}
