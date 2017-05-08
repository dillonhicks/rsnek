//! `or` - builtin operator function
//!
//! CPython does some special casing around logical operators with test-and-jump opcodes.
//! rsnek just declares them to be binop methods for now.
//!
//! As per CPython, the `or` logical operator returns the reference to the object
//! that first test `True` in the expression or the last one to test `False`.
use std::borrow::Borrow;

use ::api::method::BooleanCast;
use ::api::result::ObjectResult;
use ::api::RtObject as ObjectRef;
use ::modules::builtins::Type;
use ::runtime::Runtime;
use ::runtime::traits::BooleanProvider;


pub fn logical_or<'a>(rt: &Runtime, lhs: &ObjectRef, rhs: &ObjectRef) -> ObjectResult {
    match lhs.op_bool(rt) {
        Ok(objref) => {
            if objref == rt.bool(true) {
                return Ok(lhs.clone())
            }
        },
        Err(err) => return Err(err)
    };

    Ok(rhs.clone())
}