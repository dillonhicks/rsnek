use std::ops::Deref;
use std::borrow::Borrow;

use ::builtin::precondition::{check_args, check_kwargs};
use ::error::Error;
use ::result::{RuntimeResult};
use ::resource::strings;
use ::runtime::Runtime;
use ::traits::{TupleProvider, ModuleImporter};
use ::object::RtObject as ObjectRef;
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, Func, FuncType, SignatureBuilder};


const FUNC_NAME: &'static str = "globals";

pub struct GlobalsFn;


impl GlobalsFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => FUNC_NAME);
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_globals);

        Func {
            name: String::from(FUNC_NAME),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: [].as_args()
        }
    }
}


fn rs_builtin_globals(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => FUNC_NAME);

    match check_args(0, &pos_args) {
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

    let builtins = rt.import_module(strings::BUILTINS_MODULE)?;
    let boxed: &Box<Builtin> = builtins.0.borrow();
    let attrs = match boxed.deref() {
        &Builtin::Module(ref object) => object.dir()?,
        _ => return Err(Error::system(
                &format!("Module was not an object; file: {}, line: {}", file!(), line!())))
    };
    Ok(rt.tuple(attrs))
}
