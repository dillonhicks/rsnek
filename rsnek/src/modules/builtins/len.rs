//! `len()` - builtin function
//!
//! Get the size of a collection or implementer of `op_len` or `__len__`.
//!
use std::borrow::Borrow;

use ::api::method::{GetItem, Length};
use ::api::result::{ObjectResult};
use ::api::RtObject;
use ::modules::builtins::Type;
use ::modules::precondition::{check_args, check_kwargs};
use ::resources::strings;
use ::runtime::Runtime;
use ::runtime::traits::IntegerProvider;
use ::system::primitives as rs;
use ::system::primitives::{SignatureBuilder, Func, FuncType};


pub struct LenFn;


impl LenFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => "len");
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_len);

        Func {
            name: String::from("len"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["sequence"].as_args()
        }
    }
}


fn rs_builtin_len(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call builtin"; "native" => "len");
    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let arg = pos_args.op_getitem(&rt, &rt.int(0))?;
    arg.op_len(&rt)
}
