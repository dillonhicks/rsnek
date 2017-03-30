use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::method;


pub trait RichComparison
    : method::Equal + method::NotEqual + method::LessThan + method::LessOrEqual + method::GreaterThan + method::GreaterOrEqual
    {}
