use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::{ObjectRef, ToRtWrapperType};
use typedef::integer::IntegerObject;

use object::method;

pub trait DefaultIdentity: method::Id + method::Is + method::IsNot {
    fn op_id(&self, rt: &Runtime) -> RuntimeResult {
        let objref: ObjectRef = IntegerObject::new_u64(DefaultIdentity::native_id(self)).to();
        return rt.alloc(objref);
    }

    fn native_id(&self) -> native::ObjectId {
        return (&self as *const _) as native::ObjectId;
    }

    fn op_is(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if DefaultIdentity::native_is(self, rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    fn native_is(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(DefaultIdentity::native_id(self) == other.native_id())
    }

    fn op_is_not(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if DefaultIdentity::native_is_not(self, rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }


    fn native_is_not(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(!DefaultIdentity::native_is(self, other).unwrap())
    }
}
