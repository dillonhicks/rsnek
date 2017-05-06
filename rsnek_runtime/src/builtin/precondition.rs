///! this whole module is hacky garbage that should be refactored during {T3094} / {T111} (v0.8.0)
use std::ops::Range;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::Ref;

use ::error::Error;
use ::object::RtObject as ObjectRef;
use ::result::{RtResult};
use ::typedef::builtin::Builtin;
use ::typedef::native;


#[inline(always)]
pub fn check_args(count: usize, pos_args: &ObjectRef) -> RtResult<native::None> {
    match pos_args.as_ref() {
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

pub fn check_args_range(range: Range<usize>, pos_args: &ObjectRef) -> RtResult<usize> {
    match pos_args.as_ref() {
        &Builtin::Tuple(ref tuple) => {
            if range.contains(tuple.value.0.len()) {
                Ok(tuple.value.0.len())
            } else {
                Err(Error::typerr(&format!("Expected {:?} args, got {}", range, tuple.value.0.len())))
            }
        }
        _ => Err(Error::typerr("Expected type tuple for pos_args")),
    }
}

#[inline(always)]
pub fn check_kwargs(count: usize, kwargs: &ObjectRef) -> RtResult<native::None> {
    match kwargs.as_ref() {

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
pub fn check_fnargs_rt(fnargs: &native::FnArgs) -> RtResult<native::NativeFnArgs> {
    let &(ref pos_args, ref starargs, ref kwargs) = fnargs;

    let arg0: native::Tuple = match pos_args.as_ref() {
        &Builtin::Tuple(ref tuple) => {
            tuple.value
                .0
                .iter()
                .map(|objref| objref.clone())
                .collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for pos_args")),
    };

    let arg1: native::Tuple = match starargs.as_ref() {
        &Builtin::Tuple(ref tuple) => {
            tuple.value
                .0
                .iter()
                .map(|objref| objref.clone())
                .collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for *args")),
    };

    let arg2: native::Dict = match kwargs.as_ref() {
        &Builtin::Dict(ref dict) => {
            let borrowed: Ref<native::Dict> = dict.value.0.borrow();
            borrowed.iter().map(|(key, value)| (key.clone(), value.clone())).collect()
        }
        _ => return Err(Error::typerr("Expected type tuple for **args")),
    };

    Ok((arg0, arg1, arg2))
}