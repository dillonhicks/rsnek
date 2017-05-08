//! `tuple()` - builtin function
//!
//! In the future this will be replaced by a type object.
use std::borrow::Borrow;

use ::api::method::{GetItem, Iter};
use ::api::result::ObjectResult;
use ::api::RtObject as ObjectRef;
use ::modules::builtins::Type;
use ::modules::precondition::{check_args, check_kwargs, check_args_range};
use ::resources::strings;
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider, TupleProvider, DefaultTupleProvider};
use ::system::primitives as rs;
use ::system::primitives::{Func, FuncType, SignatureBuilder};


pub struct TupleFn;

const FUNC_NAME: &'static str = "tuple";


impl TupleFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => FUNC_NAME);
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_tuple);

        Func {
            name: String::from(FUNC_NAME),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_tuple(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> ObjectResult {
    trace!("call"; "native_builtin" => FUNC_NAME);

    let arg_count = check_args_range(0..2, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    if arg_count == 0 {
        return Ok(rt.default_tuple())
    }

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;
    let new_tuple = value.op_iter(&rt)?.collect::<rs::List>();
    Ok(rt.tuple(new_tuple))
}


#[cfg(test)]
mod tests {
    use ::runtime::traits::{DefaultDictProvider, ListProvider};
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