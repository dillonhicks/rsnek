//! `any()` - builtin function
//!
//! Given an iterable, return `True` if any element produced by the iterator
//! returns `True` after calling its `PyAPI::op_bool` method (`__bool__` in python code).
use std::borrow::Borrow;

use ::modules::precondition::{check_args, check_kwargs};
use ::api::method::{GetItem, Iter, BooleanCast};
use ::api::RtObject;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider, BooleanProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Func, FuncType, SignatureBuilder};
use ::system::primitives as rs;


pub struct AnyFn;


impl AnyFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => "any");
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_any);

        Func {
            name: String::from("any"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_any(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call"; "native_builtin" => "any");

    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;
    let iterable = value.op_iter(&rt)?;

    Ok(rt.bool(iterator_any(&rt, iterable)))
}


pub fn iterator_any<I>(rt: &Runtime, iterator: I) -> rs::Boolean
    where I: Iterator<Item=RtObject> {

    let true_ = rt.bool(true);
    
    iterator
        .map(|obj| obj.op_bool(&rt).unwrap_or(rt.bool(false)))
        .any(|t| t == true_)

}


#[cfg(test)]
mod tests {
    use super::*;
    use runtime::Runtime;
    use ::runtime::traits::{IteratorProvider,
                 DefaultTupleProvider,
                 StringProvider,
                 TupleProvider,
                 NoneProvider};

    fn setup() -> Runtime {
        Runtime::new()
    }


    #[test]
    fn empty() {
        let rt = setup();
        let tuple = rt.default_tuple();
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), false);
    }


    #[test]
    fn all_false() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![f.clone(),f.clone(),f.clone(),f.clone()]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), false);
    }

    #[test]
    fn all_true() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(),t.clone(),t.clone(),t.clone()]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn one_true() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![rt.bool(true), f.clone(), f.clone(), f.clone(), f.clone()]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn one_false() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(), t.clone(), t.clone(), t.clone(), rt.bool(false)]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn sequences() {
        let rt = setup();
        let tuple = rt.tuple(vec![rt.str("")]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), false);

        let tuple = rt.tuple(vec![rt.str(" ")]);
        let iterator = rt.iter(rs::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn arrays() {
        let rt = setup();
        assert_eq!(iterator_any(&rt, [rt.none()].iter().cloned()), false);
        assert_eq!(iterator_any(&rt, [rt.none()].iter().cloned()), false);
        assert_eq!(iterator_any(&rt, [rt.int(1)].iter().cloned()), true);

        //assert_eq!(native_any(&rt, iterator), true);
    }
}