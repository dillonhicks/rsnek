//! `globals()` - builtin function
//!
//! Return the tuple of names defined in the builtin module. This is not CPython behavior but
//! until scopes are implemented, it serves as an example.
use std::ops::Deref;
use std::borrow::Borrow;

use ::api::result::Error;
use ::api::result::{ObjectResult};
use ::api::RtObject;
use ::modules::builtins::Type;
use ::modules::precondition::{check_args, check_kwargs};
use ::resources::strings;
use ::runtime::Runtime;
use ::runtime::traits::{TupleProvider, ModuleImporter};
use ::system::primitives as rs;
use ::system::primitives::{Func, FuncType, SignatureBuilder};


const FUNC_NAME: &'static str = "globals";

pub struct GlobalsFn;


impl GlobalsFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => FUNC_NAME);
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_globals);

        Func {
            name: String::from(FUNC_NAME),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: [].as_args()
        }
    }
}


fn rs_builtin_globals(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call"; "native_builtin" => FUNC_NAME);
    check_args(0, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let builtins = rt.import_module(strings::BUILTINS_MODULE)?;

    let attrs = match builtins.as_ref() {
        &Type::Module(ref object) => object.dir()?,
        _ => return Err(Error::system(
                &format!("Module was not an object; file: {}, line: {}", file!(), line!())))
    };
    Ok(rt.tuple(attrs))
}
