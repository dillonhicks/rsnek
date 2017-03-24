
use runtime::Runtime;
use result::{NativeResult, RuntimeResult};
use typdef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::native;


fn builtin_func_len(rt: &Runtime, objref: &ObjectRef) -> RuntimeResult {
    let builtin: &Builtin = objref.0.borrow();

    foreach_builtin!(builtin, op_len, lhs)
}
