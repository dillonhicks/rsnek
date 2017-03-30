use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::method;


/// Cohesive Number trait for convenience
pub trait Number:
    method::BooleanCast +
    method::IntegerCast +
    method::FloatCast +
    method::ComplexCast +
    method::Rounding +
    method::Index +
    method::NegateValue +
    method::AbsValue +
    method::PositiveValue +
    method::InvertValue {}
