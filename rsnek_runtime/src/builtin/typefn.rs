use std::borrow::Borrow;

use object::method::{GetItem};
use runtime::Runtime;
use traits::{IntegerProvider, StringProvider};

use result::{RuntimeResult};
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::native::{self, Signature};

use builtin::precondition::{check_args, check_kwargs};

pub struct TypeFn;


impl TypeFn {
    pub fn create() -> (&'static str, native::FuncType) {
        trace!("create builtin"; "function" => "type");
        let func: Box<native::WrapperFn> = Box::new(rs_builtin_typefn);
        ("type", native::FuncType::Wrapper(func, Signature::new(
            &["obj"], &[], Some("*objects"), None)))
    }
}


fn rs_builtin_typefn(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => "type");
    match check_args(1, &pos_args) {
        Err(err) => return Err(err),
        _ => {}
    };

    match check_args(0, &starargs) {
        Err(err) => return Err(err),
        _ => {}
    };

    match check_kwargs(0, &kwargs) {
        Err(err) => return Err(err),
        _ => {}
    };

    let boxed: &Box<Builtin> = pos_args.0.borrow();
    let zero = rt.int(0);

    let name;

    {
        let value = boxed.op_getitem(&rt, &zero).unwrap();
        let boxed: &Box<Builtin> = value.0.borrow();
        name = boxed.debug_name().to_string();
    }

    // Hack for demo purposes since there are not type and class objects yet
    // TODO: {T49} return the type when type objects are a thing
    Ok(rt.str(name))
}
