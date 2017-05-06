use std::borrow::Borrow;

use ::object::method::BooleanCast;
use ::runtime::Runtime;
use ::result::RuntimeResult;
use ::typedef::builtin::Builtin;
use ::object::RtObject as ObjectRef;
use ::traits::BooleanProvider;


pub fn logical_and<'a>(rt: &Runtime, lhs: &ObjectRef, rhs: &ObjectRef) -> RuntimeResult {
    let builtin: &Box<Builtin> = lhs.0.borrow();
    match builtin.op_bool(rt) {
        Ok(objref) => {
            if objref == rt.bool(false) {
                return Ok(lhs.clone())
            }
        },
        Err(err) => return Err(err)
    };

    Ok(rhs.clone())
}
