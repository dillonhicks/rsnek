use std::fmt;
use std::iter::FromIterator;
use std::ops::{Add, Deref};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use itertools::Itertools;
use num::{ToPrimitive, Zero};

use ::resource::strings;
use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{BooleanProvider, IntegerProvider, IteratorProvider, DefaultTupleProvider, TupleProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id, Length};
use object::selfref::{self, SelfRef};

use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;


pub mod sequence {
    use super::*;

    pub fn equals<'a>(left: &'a [ObjectRef], right: &'a [ObjectRef]) -> native::Boolean {
        ((left.len() == right.len()) &&
            left.iter().zip(right.iter())
                .all(|(l, r)| l == r))
    }

    pub fn contains<'a>(seq: &'a [ObjectRef], item: &Builtin) -> native::Boolean {
        seq.iter()
            .map(|objref| objref.0.borrow())
            .any(|value: &Box<Builtin>| {
                *(value.deref()) == *item
            })
    }

    pub fn multiply<'a, T>(seq: &'a [ObjectRef], factor: usize) -> T
        where T: FromIterator<ObjectRef> {

        (0..factor)
            .flat_map(|_| seq.iter().cloned())
            .collect::<T>()
    }

    /// Get index using the normal +/- indexing rules
    pub fn get_index<'a>(seq: &'a [ObjectRef], index: &ToPrimitive) -> NativeResult<ObjectRef> {
        let index_err = Err(Error::index("Index out of range"));
        let idx = match index.to_isize() {
            Some(idx) => idx,
            _ => return index_err,
        };

        let len = seq.len() as isize;
        let pos_range = (0 <= idx) && (idx < len);
        let neg_range = (-len <= idx) && (idx < 0);

        match (pos_range, neg_range) {
            (true, false) => Ok(seq[idx as usize].clone()),
            (false, true) => Ok(seq[(idx + len) as usize].clone()),
            _ => index_err
        }
    }
}