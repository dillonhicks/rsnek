use std::borrow::Borrow;

use ::object::method::{
    GetItem,
    Iter,
};
use ::runtime::Runtime;
use ::result::RuntimeResult;
use ::resource::strings;
use ::traits::{IntegerProvider, TupleProvider, DefaultTupleProvider};
use ::typedef::objectref::ObjectRef;
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, Func, FuncType, SignatureBuilder};

use ::builtin::precondition::{check_args, check_kwargs, check_args_range};

pub struct TupleFn;

const FUNC_NAME: &'static str = "tuple";


impl TupleFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => FUNC_NAME);
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_tuple);

        Func {
            name: String::from(FUNC_NAME),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_tuple(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => FUNC_NAME);

    let arg_count = check_args_range(0..2, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    if arg_count == 0 {
        return Ok(rt.default_tuple())
    }

    let boxed: &Box<Builtin> = pos_args.0.borrow();
    let zero = rt.int(0);

    let value = boxed.op_getitem(&rt, &zero).unwrap();
    let boxed: &Box<Builtin> = value.0.borrow();

    let iter = boxed.op_iter(&rt)?;
    let new_tuple = iter.collect::<native::Tuple>();
    Ok(rt.tuple(new_tuple))
}


#[cfg(test)]
mod tests {
    use ::traits::{DefaultDictProvider, ListProvider};
    use super::*;


    fn setup() -> Runtime {
        Runtime::new()
    }

    #[test]
    fn from_list() {
        let rt = setup();
        let tuple = rt.list(vec![rt.int(1), rt.int(2), rt.int(3)]);

        let expected = rt.tuple(vec![rt.int(1), rt.int(2), rt.int(3)]);
        let tuple = rs_builtin_tuple(
            &rt, &rt.tuple(vec![tuple]),
            &rt.default_tuple(),
            &rt.default_dict()).unwrap();

        assert_eq!(tuple, expected);
    }

    #[test]
    #[should_panic]
    fn from_int() {
        let rt = setup();
        let int = rt.int(29345);

        rs_builtin_tuple(
            &rt, &rt.tuple(vec![int]),
            &rt.default_tuple(),
            &rt.default_dict()).unwrap();
    }
}