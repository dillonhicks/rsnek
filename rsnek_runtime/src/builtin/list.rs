use std::borrow::Borrow;

use object::method::{
    GetItem,
    Iter,
};
use runtime::Runtime;
use traits::{
    ListProvider,
    DefaultListProvider,
    IntegerProvider
};

use result::{RuntimeResult};
use typedef::objectref::ObjectRef;
use ::resource::strings;
use typedef::builtin::Builtin;
use typedef::native::{self, Func, FuncType, SignatureBuilder};

use builtin::precondition::{check_args, check_kwargs, check_args_range};

pub struct ListFn;


impl ListFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "list");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_list);

        Func {
            name: String::from("list"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_list(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => "list");

    let arg_count = check_args_range(0..2, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    if arg_count == 0 {
        return Ok(rt.default_list())
    }

    let boxed: &Box<Builtin> = pos_args.0.borrow();
    let zero = rt.int(0);

    let value = boxed.op_getitem(&rt, &zero).unwrap();
    let boxed: &Box<Builtin> = value.0.borrow();

    let iter = boxed.op_iter(&rt)?;
    let new_list = iter.collect::<native::List>();
    Ok(rt.list(new_list))
}


#[cfg(test)]
mod tests {
    use ::traits::{
        TupleProvider,
        DefaultTupleProvider,
        DefaultDictProvider
    };
    use super::*;

    fn setup() -> Runtime {
        Runtime::new()
    }

    #[test]
    fn from_tuple() {
        let rt = setup();
        let tuple = rt.tuple(vec![rt.int(1), rt.int(2), rt.int(3)]);

        let expected = rt.list(vec![rt.int(1), rt.int(2), rt.int(3)]);
        let list = rs_builtin_list(
            &rt, &rt.tuple(vec![tuple]),
            &rt.default_tuple(),
            &rt.default_dict()).unwrap();

        assert_eq!(list, expected);
    }

    #[test]
    #[should_panic]
    fn from_int() {
        let rt = setup();
        let int = rt.int(29345);

        rs_builtin_list(
            &rt, &rt.tuple(vec![int]),
            &rt.default_tuple(),
            &rt.default_dict()).unwrap();
    }
}