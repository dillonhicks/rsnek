//! Native number coercions and comparisons
use std;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use num::{ToPrimitive};

use objects::native::{self, HashId};


pub fn format_float(float: &native::Float) -> native::String {
    format!("{:?}", *float)

}

pub fn format_int(int: &native::Integer) -> native::String {
    format!("{}", *int)
}


pub fn hash_int(int: &native::Integer) -> HashId {
    let mut s = DefaultHasher::new();
    int.hash(&mut s);
    s.finish()
}

// To make int == float not such a pain in the ass
pub struct IntAdapter<'a>(pub &'a native::Integer);
pub struct FloatAdapter<'a>(pub &'a native::Float);

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
//    type Output = native::Float;
//
//    fn add(self, rhs: FloatAdapter) -> Self::Output {
//        match self.0
//    }
//}
