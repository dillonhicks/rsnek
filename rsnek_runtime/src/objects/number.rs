//! Native number coercions and comparisons. I think rust already does this with the `Wrapped`
//! traits...
use std;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use num::{ToPrimitive};

use ::system::primitives::{HashId};
use ::system::primitives as rs;


pub fn format_float(float: &rs::Float) -> rs::String {
    format!("{:?}", *float)

}

pub fn format_int(int: &rs::Integer) -> rs::String {
    format!("{}", *int)
}


pub fn hash_int(int: &rs::Integer) -> HashId {
    let mut s = DefaultHasher::new();
    int.hash(&mut s);
    s.finish()
}

// To make int == float not such a pain in the ass
pub struct IntAdapter<'a>(pub &'a rs::Integer);
pub struct FloatAdapter<'a>(pub &'a rs::Float);

impl<'a, 'b> std::cmp::PartialEq<IntAdapter<'b>> for FloatAdapter<'a> {
    fn eq(&self, other: &IntAdapter) -> bool {
        match other.0.to_f64() {
            Some(num)   => *self.0 == num,
            None        => false
        }
    }
}

impl<'a, 'b> std::cmp::PartialEq<FloatAdapter<'b>> for IntAdapter<'a> {
    fn eq(&self, other: &FloatAdapter) -> bool {
        match self.0.to_f64() {
            Some(num)   => num == *other.0,
            None        => false
        }
    }
}

//
//impl<'a, 'b> std::ops::Add<FloatAdapter<'b>> for IntAdapter<'a> {
//    type Output = rs::Float;
//
//    fn add(self, rhs: FloatAdapter) -> Self::Output {
//        match self.0
//    }
//}
