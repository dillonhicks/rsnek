use std::borrow::Borrow;

use ::api::method::BooleanCast;
use ::runtime::Runtime;
use ::api::result::ObjectResult;
use ::objects::builtin::Builtin;
use ::api::RtObject as ObjectRef;
use ::runtime::traits::BooleanProvider;


pub fn logical_and<'a>(rt: &Runtime, lhs: &ObjectRef, rhs: &ObjectRef) -> ObjectResult {
    match lhs.op_bool(rt) {
        Ok(object) => {
            if object == rt.bool(false) {
                return Ok(lhs.clone())
            }
        },
        Err(err) => return Err(err)
    };

    Ok(rhs.clone())
}
