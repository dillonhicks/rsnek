use std::borrow::Borrow;

use ::object::method::BooleanCast;
use ::runtime::Runtime;
use ::result::RuntimeResult;
use ::typedef::builtin::Builtin;
use ::typedef::objectref::ObjectRef;
use ::typedef::native;
use ::traits::BooleanProvider;


pub fn logical_or<'a>(rt: &Runtime, lhs: &ObjectRef, rhs: &ObjectRef) -> RuntimeResult {
    let builtin: &Box<Builtin> = lhs.0.borrow();
    match builtin.op_bool(rt) {
        Ok(objref) => {
            if objref == rt.bool(true) {
                return Ok(lhs.clone())
            }
        },
        Err(err) => return Err(err)
    };

    let builtin: &Box<Builtin> = rhs.0.borrow();
    match builtin.op_bool(rt) {
        Ok(objref) => Ok(rhs.clone()),
        Err(err) => Err(err)
    }

}