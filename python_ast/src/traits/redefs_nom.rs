//! Traits input types have to implement to work with nom combinators
//!
use ::token;


/// Abstract method to calculate the input length. Redefined in this crate
/// because of the rust orphan rules that prevent using other library's
/// traits on types not defined by the library.
pub trait InputLengthRedef {
    /// calculates the input length, as indicated by its name,
    /// and the name of the trait itself
    #[inline]
    fn input_len(&self) -> usize;
}

/// Generic implementation of the `nom::InputLength` trait for slices that
/// can be used in this crate.
impl<'a, T> InputLengthRedef for &'a[T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}


/// Each fixed size array represents a specific type. While a potential pain in the ass
/// we just throw some metaprogramming at it with a macro and we can generate implementations
/// for arrays of our choosing. See `redef_array_impls`.
macro_rules! redef_array_impls {
  ($i:ty, $($N:expr)+) => {
    $(
      impl InputLengthRedef for [$i; $N] {
        #[inline]
        fn input_len(&self) -> usize {
          self.len()
        }
      }

      impl<'a> InputLengthRedef for &'a [$i; $N] {
        #[inline]
        fn input_len(&self) -> usize {
          self.len()
        }
      }
    )+
  };
}


/// Create the implementations for arrays of `token::Id` from sizes 0...32. Required
/// for some macros.
redef_array_impls! { token::Id,
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}
