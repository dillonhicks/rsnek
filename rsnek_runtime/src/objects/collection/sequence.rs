use std::iter::FromIterator;
use std::ops::Deref;
use std::borrow::Borrow;

use num::ToPrimitive;

use ::api::result::Error;
use ::api::result::RtResult;
use ::resources::strings;
use ::objects::builtin::Builtin;
use ::objects::native;
use ::api::RtObject;


pub fn equals<'a>(left: &'a [RtObject], right: &'a [RtObject]) -> native::Boolean {
    ((left.len() == right.len()) &&
        left.iter().zip(right.iter())
            .all(|(l, r)| l == r))
}

pub fn contains<'a>(seq: &'a [RtObject], item: &Builtin) -> native::Boolean {
    seq.iter()
        .map(|objref| objref.as_ref())
        .any(|value: &Builtin| {
            *(value.deref()) == *item
        })
}

/// Create a new sequences which is slice repeated `factor` times.
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
///
/// Note: This is a particularly hot path. Any changes should compare against the most
/// recent benchmarks to ensure that it does not cause a regression in unforeseen ways.
#[inline(always)]
pub fn get_index<'a, T>(seq: &'a [T], index: &ToPrimitive) -> RtResult<T>
    where T: Clone, { //V: Index<usize, Output=T>
    // TODO: {T3091} update "sequence" to be the type name
    let idx = match index.to_isize() {
        Some(idx) => idx,
        _ => return Err(rsnek_exception_index!("sequence")),
    };

    let len = seq.len() as isize;

    if 0 <= idx {
        if idx < len {
            return Ok(seq[idx as usize].clone())
        }
    } else if -len <= idx {
        return Ok(seq[(len + idx) as usize].clone())
    }

    Err(rsnek_exception_index!("sequence"))

}