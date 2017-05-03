use std::borrow::Borrow;

use object::method::{GetItem, StringCast};
use runtime::Runtime;
use traits::{IntegerProvider};

use result::{RuntimeResult};
use typedef::objectref::ObjectRef;
use ::resource::strings;
use typedef::builtin::Builtin;
use typedef::native::{self, SignatureBuilder, Func, FuncType};

use builtin::precondition::{check_args, check_kwargs};

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


fn rs_builtin_str(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call builtin"; "native" => "str");

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

    let value = boxed.op_getitem(&rt, &zero).unwrap();
    let boxed: &Box<Builtin> = value.0.borrow();

    boxed.op_str(&rt)
}
