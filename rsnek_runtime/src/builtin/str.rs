use std::borrow::Borrow;

use ::builtin::precondition::{check_args, check_kwargs};
use ::object::method::{GetItem, StringCast};
use ::object::RtObject as ObjectRef;
use ::resource::strings;
use ::result::{ObjectResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider};
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, SignatureBuilder, Func, FuncType};


pub struct StrFn;


impl StrFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "str");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_str);

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
