use std::borrow::Borrow;

use ::modules::precondition::{check_args, check_kwargs};
use ::api::method::{GetItem};
use ::api::RtObject as ObjectRef;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider, StringProvider};
use ::objects::builtin::Builtin;
use ::objects::native::{self, Signature, Func, FuncType};


pub struct TypeFn;


impl TypeFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "type");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_typefn);

        Func {
            name: String::from("type"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: Signature::new(
                &["obj"], &[], Some("*objects"), None)
        }
    }
}


fn rs_builtin_typefn(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> ObjectResult {
    trace!("call"; "native_builtin" => "type");
    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;


    // Hack for demo purposes since there are not type and class objects yet
    // TODO: {T49} return the type when type objects are a thing
    Ok(rt.str(value.as_ref().debug_name()))
}
