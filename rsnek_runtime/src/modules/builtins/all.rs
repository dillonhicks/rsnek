use std::borrow::Borrow;

use ::api::method::{GetItem, Iter, BooleanCast};
use ::api::RtObject;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider, BooleanProvider};
use ::objects::builtin::Builtin;
use ::objects::native::{self, Func, FuncType, SignatureBuilder};

use ::modules::precondition::{check_args, check_kwargs};

pub struct AllFn;


impl AllFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "all");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_all);

        Func {
            name: String::from("all"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_all(rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
    trace!("call"; "native_builtin" => "all");

    check_args(1, &pos_args)?;
    check_args(0, &starargs)?;
    check_kwargs(0, &kwargs)?;

    let value = pos_args.op_getitem(&rt, &rt.int(0))?;
    let iterable = value.op_iter(&rt)?;

    Ok(rt.bool(iterator_all(&rt, iterable)))
}


pub fn iterator_all<I>(rt: &Runtime, iterator: I) -> native::Boolean
    where I: Iterator<Item=RtObject> {

    let true_ = rt.bool(true);

    iterator
        .map(|obj| obj.op_bool(&rt).unwrap_or(rt.bool(false)))
        .all(|t| t == true_)
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
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), true);
    }


    #[test]
    fn all_false() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![f.clone(),f.clone(),f.clone(),f.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), false);
    }

    #[test]
    fn all_true() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(),t.clone(),t.clone(),t.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), true);
    }

    #[test]
    fn one_true() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![rt.bool(true), f.clone(), f.clone(), f.clone(), f.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), false);
    }

    #[test]
    fn one_false() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(), t.clone(), t.clone(), t.clone(), rt.bool(false)]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), false);
    }


    #[test]
    fn sequences() {
        let rt = setup();
        let tuple = rt.tuple(vec![rt.str("")]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), false);

        let tuple = rt.tuple(vec![rt.str(" ")]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_all(&rt, iterator), true);
    }

    #[test]
    fn arrays() {
        let rt = setup();
        assert_eq!(iterator_all(&rt, [rt.none()].iter().cloned()), false);
        assert_eq!(iterator_all(&rt, [rt.none()].iter().cloned()), false);

        //assert_eq!(native_all(&rt, iterator), true);
    }
}