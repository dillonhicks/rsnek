use std::borrow::Borrow;

use ::modules::precondition::{check_args, check_kwargs};
use ::api::method::{GetItem, StringCast};
use ::api::RtObject as ObjectRef;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider};
use ::modules::builtins::Type;
use ::system::primitives::{SignatureBuilder, Func, FuncType};
use ::system::primitives as rs;


pub struct StrFn;


impl StrFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => "str");
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_str);

        Func {
            name: String::from("str"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["obj"].as_args(),
        }
    }
}


fn rs_builtin_str(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> ObjectResult {
    trace!("call builtin"; "native" => "str");

    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;
    value.op_str(&rt)
}
