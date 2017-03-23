use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::identity::DefaultIdentity;
use object::method;


pub trait RichComparison:
    DefaultEqual +
    DefaultNotEqual +
    method::LessThan +
    method::LessOrEqual +
    method::GreaterThan +
    method::GreaterOrEqual {}


/// The object comparison functions are useful for all objects,
/// and are named after the rich comparison operators they support
pub trait DefaultEqual: DefaultIdentity + method::Equal {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if DefaultEqual::native_eq(self, rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return DefaultIdentity::native_is(self, other);
    }
}


pub trait DefaultNotEqual: DefaultEqual + method::NotEqual {

    /// Default implementation of equals fallsbacks to op_is_not.
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if DefaultNotEqual::native_ne(self, rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_ne(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return Ok(!DefaultEqual::native_eq(self, other).unwrap());
    }

}
