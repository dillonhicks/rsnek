use std::borrow::Borrow;

use ::api::method::BooleanCast;
use ::runtime::Runtime;
use ::api::result::ObjectResult;
use ::modules::builtins::Type;
use ::api::RtObject as ObjectRef;
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