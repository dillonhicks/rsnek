use std::borrow::Borrow;

use object::method::{GetItem, Length};
use runtime::Runtime;
use traits::IntegerProvider;

use result::{RuntimeResult};
use ::object::RtObject as ObjectRef;
use ::resource::strings;
use typedef::builtin::Builtin;
use typedef::native::{self, SignatureBuilder, Func, FuncType};

use builtin::precondition::{check_args, check_kwargs};

pub struct LenFn;


impl LenFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "len");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_len);

        Func {
            name: String::from("len"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["sequence"].as_args()
        }
    }
}


fn rs_builtin_len(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call builtin"; "native" => "len");

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

    boxed.op_len(&rt)
}
