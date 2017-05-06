use std::borrow::Borrow;

use ::object::method::BooleanCast;
use ::runtime::Runtime;
use ::result::ObjectResult;
use ::typedef::builtin::Builtin;
use ::object::RtObject as ObjectRef;
use ::traits::BooleanProvider;


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
