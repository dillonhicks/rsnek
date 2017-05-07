use std::borrow::Borrow;

use ::builtin::precondition::{check_args, check_kwargs, check_args_range};
use ::api::method::{GetItem,Iter};
use ::api::RtObject;
use ::resource::strings;
use ::result::{ObjectResult};
use ::runtime::Runtime;
use ::traits::{ListProvider, DefaultListProvider, IntegerProvider};
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, Func, FuncType, SignatureBuilder};


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


fn rs_builtin_list(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call"; "native_builtin" => "list");

    let arg_count = check_args_range(0..2, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;
    
    if arg_count == 0 {
        return Ok(rt.default_list())
    }

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;
    let new_list  = value.op_iter(&rt)?.collect::<native::List>();
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