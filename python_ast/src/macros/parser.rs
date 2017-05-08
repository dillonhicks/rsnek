#![allow(unused_imports)]

use ::token::Id;
use ::slice::TkSlice;
use traits::redefs_nom::InputLengthRedef;

/// Generalized form of nom's `eat_seperator!` macro
///
/// helper macros to build a separator parser
///
/// ```ignore
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
///
/// named!(pub consume_spaces_and_tabs, drop_tokens!(&[Id::Space, Id::Tab]));
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! drop_tokens (
      ($i:expr, $arr:expr) => (
        {
          use nom;
          use nom::{InputLength,InputIter,Slice,FindToken};
          if ($i).input_len() == 0 {
            nom::IResult::Done(($i).slice(0..), ($i).slice(0..0))
          } else {
            match ($i).iter_indices().map(|(j, item)| {
                (j, item.find_token($arr))
               })
                .filter(|&(_, is_token)| !is_token)
                .map(|(j, _)| j)
                .next() {
              ::std::option::Option::Some(index) => {
                nom::IResult::Done(($i).slice(index..), ($i).slice(..index))
              },
              ::std::option::Option::None        => {
                nom::IResult::Done(($i).slice(($i).input_len()..), ($i))
              }
            }
          }
        }
      );
    );


/// Redef of noms ws!() macro. Ignores spaces and tabs for the scope of the parser.
#[macro_export]
macro_rules! ignore_spaces (
  ($i:expr, $($args:tt)*) => (
    {
      use $crate::util::filter_non_critical_python_whitespace;
      sep!($i, filter_non_critical_python_whitespace, $($args)*)
    }
  )
);


/// Matches one of the provided tokens.
/// Generalized form of nom's `one_of!` macro.
#[macro_export]
macro_rules! tk_is_one_of (
    ($i:expr, $inp: expr) => (
        {
          use nom::Slice;
          use nom::AsChar;
          use nom::FindToken;
          use nom::InputIter;

          match ($i).iter_elements().next().map(|c| {
            c.find_token($inp)
          }) {
            None        => nom::IResult::Incomplete::<_, _>(nom::Needed::Size(1)),
            Some(false) => nom::IResult::Error(error_position!(nom::ErrorKind::OneOf, $i)),
            //the unwrap should be safe here
            Some(true)  => nom::IResult::Done($i.slice(1..), $i.iter_elements().next().unwrap())
          }
        }
    );
);


/// Matches one of the provided tokens.
/// Generalized form of nom's `none_of!` macro. which just takes the .as_char() off
/// of the Some(true) case
#[macro_export]
macro_rules! tk_is_none_of (
  ($i:expr, $inp: expr) => (
    {
      use nom::Slice;
      use nom::AsChar;
      use nom::FindToken;
      use nom::InputIter;
      use $crate::slice::TkSlice;

      match ($i).iter_elements().next().map(|c| {
        !c.find_token($inp)
      }) {
        None        => nom::IResult::Incomplete::<_, _>(nom::Needed::Size(1)),
        Some(false) => nom::IResult::Error(error_position!(nom::ErrorKind::NoneOf, $i)),
        //the unwrap should be safe here
        Some(true)  => nom::IResult::Done($i.slice(1..), $i.slice(..1))
      }
    }
  );
);
