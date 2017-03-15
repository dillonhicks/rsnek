use std::collections::HashSet;

use result::RuntimeResult;
use runtime::Runtime;

use super::objectref::{self, ObjectRef};

pub type Set = HashSet<ObjectRef>;

#[derive(Clone,Debug)]
pub struct SetObject{
    value: Set

}

impl SetObject {
    #[inline]
    pub fn new() -> SetObject {
        SetObject {
            value: Set::new()
        }
    }
}

impl objectref::ObjectBinaryOperations for SetObject {
    fn add(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}


use object;
impl object::api::Identity for SetObject{}
impl object::api::Hashable for SetObject{}