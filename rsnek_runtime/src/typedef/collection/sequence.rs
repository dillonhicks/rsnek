use std::iter::FromIterator;
use std::ops::Deref;
use std::borrow::Borrow;

use num::ToPrimitive;

use ::error::Error;
use ::result::NativeResult;
use ::resource::strings;
use ::typedef::builtin::Builtin;
use ::typedef::native;
use ::object::RtObject;


pub fn equals<'a>(left: &'a [RtObject], right: &'a [RtObject]) -> native::Boolean {
    ((left.len() == right.len()) &&
        left.iter().zip(right.iter())
            .all(|(l, r)| l == r))
}

pub fn contains<'a>(seq: &'a [RtObject], item: &Builtin) -> native::Boolean {
    seq.iter()
        .map(|objref| objref.0.borrow())
        .any(|value: &Box<Builtin>| {
            *(value.deref()) == *item
        })
}

/// Create a new sequnces which is slice repeated `factor` times.
/// ```ignore
/// let rt = Runtime::new();
/// let objects = vec![rt.int(1), rt.int(2), rt.int(3)];
/// let repeated_3_times = multiply(&objects, 3);
/// assert_eq!(repeated_3_times.len(), objects.len() * 3);
/// ```
pub fn multiply<'a, T>(seq: &'a [RtObject], factor: usize) -> T
    where T: FromIterator<RtObject> {
    // TODO: {T3092} determine the efficiency of this vs. preallocating a vector
    // and cloning the slice into the vector n times.
    (0..factor)
        .flat_map(|_| seq.iter().cloned())
        .collect::<T>()
}

/// Get index using the normal +/- indexing rules
pub fn get_index<'a, T>(seq: &'a [T], index: &ToPrimitive) -> NativeResult<T>
    where T: Clone, { //V: Index<usize, Output=T>
    // TODO: {T3091} update "sequence" to be the type name
    let index_err = Err(rsnek_exception_index!("sequence"));
    let idx = match index.to_i64() {
        Some(idx) => idx,
        _ => return index_err,
    };

    let len = seq.len() as i64;
    let pos_range = (0 <= idx) && (idx < len);
    let neg_range = (-len <= idx) && (idx < 0);

    match (pos_range, neg_range) {
        (true, false) => {
            let i = idx as usize;
            Ok(seq.get(i).unwrap().clone())
        },
        (false, true) => {
            let i = (idx  + len) as usize;
            Ok(seq.get(i).unwrap().clone())
        },
        _ => index_err
    }
}