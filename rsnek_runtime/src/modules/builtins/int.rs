use std::borrow::Borrow;

use ::modules::precondition::{check_args, check_kwargs};
use ::api::method::{GetItem, IntegerCast};
use ::api::RtObject;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider};
use ::modules::builtins::Type;
use ::objects::native::{self, Func, FuncType, SignatureBuilder};


pub struct IntFn;


impl IntFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "int");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_int);

        Func {
            name: String::from("int"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["obj"].as_args()
        }
    }
}


fn rs_builtin_int(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call"; "native_builtin" => "int");

    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let arg = pos_args.op_getitem(&rt, &rt.int(0))?;
    arg.op_int(&rt)
}
