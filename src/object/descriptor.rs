use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;
use object::method;


pub trait Descriptor:
    method::DescriptorGet +
    method::DescriptorSet +
    method::DescriptorDelete +
    method::DescriptorSetName {}
