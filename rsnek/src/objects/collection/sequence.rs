//! Common functions for sequence like data. Generally this mean that the backing data type
//! is representable as a slice or an iterator.
use std::iter::FromIterator;
use std::ops::Deref;
use std::borrow::Borrow;

use num::ToPrimitive;

use ::api::result::Error;
use ::api::result::RtResult;
use ::resources::strings;
use ::modules::builtins::Type;
use ::system::primitives as rs;
use ::api::RtObject;

/// Returns true if `object` looks like a sequence type
pub fn is_sequence(object: &RtObject) -> rs::Boolean {
    match object.as_ref() {
        &Type::List(_)       |
        &Type::Tuple(_)      |
        &Type::Dict(_)       |
        &Type::Set(_)        |
        &Type::FrozenSet(_)  |
        &Type::Str(_)        |
        &Type::Bytes(_)      => true,
        _ => false
    }
}

/// Determine if two two sequences are equal by first comparing their lengths
/// then comparing each of their elements, in order.
pub fn equals<'a>(left: &'a [RtObject], right: &'a [RtObject]) -> rs::Boolean {
    ((left.len() == right.len()) &&
        left.iter().zip(right.iter())
            .all(|(l, r)| l == r))
}

/// Test if `item` is contained in the sequence `seq`
pub fn contains<'a>(seq: &'a [RtObject], item: &Type) -> rs::Boolean {
    seq.iter()
        .map(|objref| objref.as_ref())
        .any(|value: &Type| {
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