use std::borrow::Borrow;

use object::method::{GetItem, Iter, BooleanCast};
use runtime::Runtime;
use traits::{IntegerProvider, BooleanProvider};

use result::{RuntimeResult};
use typedef::objectref::ObjectRef;
use ::resource::strings;
use typedef::builtin::Builtin;
use typedef::native::{self, Func, FuncType, SignatureBuilder};

use builtin::precondition::{check_args, check_kwargs};

pub struct AnyFn;


impl AnyFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "any");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_any);

        Func {
            name: String::from("any"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: ["iterable"].as_args()
        }
    }
}


fn rs_builtin_any(rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => "any");

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

    let iterable = match boxed.op_iter(&rt) {
        Ok(objref) => objref,
        Err(err) => return Err(err)
    };

    Ok(rt.bool(iterator_any(&rt, iterable)))
}


pub fn iterator_any<I>(rt: &Runtime, iterator: I) -> native::Boolean
    where I: Iterator<Item=ObjectRef> {
    iterator.map(|objref| {
        let builtin: &Box<Builtin> = objref.0.borrow();
        builtin.op_bool(&rt).unwrap_or(rt.bool(true))
    })
    .any(|objref| objref == rt.bool(true))
}


#[cfg(test)]
mod tests {
    use super::*;
    use runtime::Runtime;
    use traits::{IteratorProvider,
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

        assert_eq!(iterator_any(&rt, iterator), false);
    }


    #[test]
    fn all_false() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![f.clone(),f.clone(),f.clone(),f.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), false);
    }

    #[test]
    fn all_true() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(),t.clone(),t.clone(),t.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn one_true() {
        let rt = setup();
        let f = rt.bool(false);
        let tuple = rt.tuple(vec![rt.bool(true), f.clone(), f.clone(), f.clone(), f.clone()]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn one_false() {
        let rt = setup();
        let t = rt.bool(true);
        let tuple = rt.tuple(vec![t.clone(), t.clone(), t.clone(), t.clone(), rt.bool(false)]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), true);
    }

    #[test]
    fn sequences() {
        let rt = setup();
        let tuple = rt.tuple(vec![rt.str("")]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

        assert_eq!(iterator_any(&rt, iterator), false);

        let tuple = rt.tuple(vec![rt.str(" ")]);
        let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

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