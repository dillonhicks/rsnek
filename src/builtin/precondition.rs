use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::Ref;

use result::{NativeResult};
use error::Error;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


#[inline(always)]
pub fn check_args(count: usize, pos_args: &ObjectRef) -> NativeResult<native::None> {
    let boxed: &Box<Builtin> = pos_args.0.borrow();
    match boxed.deref() {
        &Builtin::Tuple(ref tuple) => {
            if tuple.value.0.len() == count {
                Ok(native::None())
            } else {
                Err(Error::typerr("Argument mismatch 1"))
            }
        }
        _ => Err(Error::typerr("Expected type tuple for pos_args")),
    }
}


#[inline(always)]
pub fn check_kwargs(count: usize, kwargs: &ObjectRef) -> NativeResult<native::None> {
    let boxed: &Box<Builtin> = kwargs.0.borrow();
    match boxed.deref() {

        &Builtin::Dict(ref dict) => {
            let borrowed: Ref<native::Dict> = dict.value.0.borrow();

            if borrowed.len() == count {
                Ok(native::None())
            } else {
                Err(Error::typerr("Argument mismatch 2"))
            }
        }
        _ => Err(Error::typerr("Expected type tuple for pos_args")),
    }

}


/// Check and copy args as part of the native method calling conventions
#[inline(always)]
pub fn check_fnargs_rt(fnargs: &native::FnArgs) -> NativeResult<native::NativeFnArgs> {
    let &(ref pos_args, ref starargs, ref kwargs) = fnargs;

    let boxed: &Box<Builtin> = pos_args.0.borrow();
    let arg0: native::Tuple = match boxed.deref() {
        &Builtin::Tuple(ref tuple) => {
            tuple.value
                .0
                .iter()
                .map(|objref| objref.clone())
                .collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for pos_args")),
    };

    let boxed: &Box<Builtin> = starargs.0.borrow();
    let arg1: native::Tuple = match boxed.deref() {
        &Builtin::Tuple(ref tuple) => {
            tuple.value
                .0
                .iter()
                .map(|objref| objref.clone())
                .collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for *args")),
    };

    let boxed: &Box<Builtin> = kwargs.0.borrow();
    let arg2: native::Dict = match boxed.deref() {
        &Builtin::Dict(ref dict) => {
            let borrowed: Ref<native::Dict> = dict.value.0.borrow();
            borrowed.iter().map(|(key, value)| (key.clone(), value.clone())).collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for **args")),
    };

    Ok((arg0, arg1, arg2))
}