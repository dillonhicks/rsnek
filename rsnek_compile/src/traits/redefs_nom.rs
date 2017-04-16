//! Traits input types have to implement to work with nom combinators
//!
use std::ops::{Range,RangeTo,RangeFrom,RangeFull};
use std::iter::Enumerate;

use token;


/// abstract method to calculate the input length
pub trait InputLength {
    /// calculates the input length, as indicated by its name,
    /// and the name of the trait itself
    #[inline]
    fn input_len(&self) -> usize;
}


impl<'a, T> InputLength for &'a[T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}



macro_rules! redef_array_impls {
  ($i:ty, $($N:expr)+) => {
    $(
      impl InputLength for [$i; $N] {
        #[inline]
        fn input_len(&self) -> usize {
          self.len()
        }
      }

      impl<'a> InputLength for &'a [$i; $N] {
        #[inline]
        fn input_len(&self) -> usize {
          self.len()
        }
      }
    )+
  };
}


redef_array_impls! { token::Id,
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}
