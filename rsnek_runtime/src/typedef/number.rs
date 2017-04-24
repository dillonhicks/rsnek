//! Native number coercions and comparisons
use std;

use num::{ToPrimitive};

use typedef::native;


#[inline(always)]
pub fn format_float(float: &native::Float) -> native::String {
    format!("{:?}", *float)

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
