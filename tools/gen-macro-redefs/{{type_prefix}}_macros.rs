
//! Macro combinators
//!
//! Macros are used to make combination easier,
//! since they often do not depend on the type
//! of the data they manipulate or return.
//!
//! There is a trick to make them easier to assemble,
//! combinators are defined like this:
//!
//! ```ignore
//! macro_rules! tag (
//!   ($i:expr, $inp: expr) => (
//!     {
//!       ...
//!     }
//!   );
//! );
//! ```
//!
//! But when used in other combinators, are Used
//! like this:
//!
//! ```ignore
//! named!(my_function, tag!("abcd"));
//! ```
//!
//! Internally, other combinators will rewrite
//! that call to pass the input as first argument:
//!
//! ```ignore
//! macro_rules! {{type_prefix}}_named (
//!   ($name:ident, $submac:ident!( $($args:tt)* )) => (
//!     fn $name<'a>( i: {{input_slice_type_explicit_lifetime}} ) -> nom::IResult<'a,{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}> {
//!       $submac!(i, $($args)*)
//!     }
//!   );
//! );
//! ```
//!
//! If you want to call a combinator directly, you can
//! do it like this:
//!
//! ```ignore
//! let res = { tag!(input, "abcd"); }
//! ```
//!
//! Combinators must have a specific variant for
//! non-macro arguments. Example: passing a function
//! to take_while! instead of another combinator.
//!
//! ```ignore
//! macro_rules! take_while(
//!   ($input:expr, $submac:ident!( $($args:tt)* )) => (
//!     {
//!       ...
//!     }
//!   );
//!
//!   // wrap the function in a macro to pass it to the main implementation
//!   ($input:expr, $f:expr) => (
//!     take_while!($input, call!($f));
//!   );
//! );
//!
#[allow(unused_variables)]
use slice::TkSlice;


#[macro_export]
macro_rules! {{type_prefix}}_tag (
  ($i:expr, $tag: expr) => (
    {
      use nom::{Compare,CompareResult,InputLength,Slice};
      {{crate_traits}}
      let res: nom::IResult<_,_> = match ($i).compare($tag) {
        CompareResult::Ok => {
          let blen = $tag.input_len();
          nom::IResult::Done($i.slice(blen..), $i.slice(..blen))
        },
        CompareResult::Incomplete => {
          nom::IResult::Incomplete(nom::Needed::Size($tag.input_len()))
        },
        CompareResult::Error => {
          nom::IResult::Error(error_position!(nom::ErrorKind::Tag, $i))
        }
      };
      res
    }
  );
);

/// Makes a function from a parser combination
///
/// The type can be set up if the compiler needs
/// more information
///
/// ```ignore
/// named!(my_function( {{input_slice_type_implicit_lifetime}} ) -> {{input_slice_type_implicit_lifetime}}, tag!("abcd"));
/// // first type parameter is input, second is output
/// named!(my_function<{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}>,     tag!("abcd"));
/// // will have {{input_slice_type_implicit_lifetime}} as input type, {{input_slice_type_implicit_lifetime}} as output type
/// named!(my_function,                   tag!("abcd"));
/// // will use {{input_slice_type_implicit_lifetime}} as input type (use this if the compiler
/// // complains about lifetime issues
/// named!(my_function<{{input_slice_type_implicit_lifetime}}>,            tag!("abcd"));
/// //prefix them with 'pub' to make the functions public
/// named!(pub my_function,               tag!("abcd"));
/// ```
#[macro_export]
macro_rules! {{type_prefix}}_named (
    (#$($args:tt)*) => (
        {{type_prefix}}_named_attr!(#$($args)*);
    );
    ($name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name( i: $i ) -> nom::IResult<$i,$o,u32> {
            $submac!(i, $($args)*)
        }
    );
    ($name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name( i: $i ) -> nom::IResult<$i, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    ($name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name( i: $i ) -> nom::IResult<$i, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name<'a>( i: {{input_slice_type_explicit_lifetime}} ) -> nom::IResult<{{input_slice_type_explicit_lifetime}}, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name( i: {{input_slice_type_implicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}, u32> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name( i: $i ) -> nom::IResult<$i,$o, u32> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name( i: $i ) -> nom::IResult<$i, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name( i: $i ) -> nom::IResult<$i, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name( i: {{input_slice_type_implicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name<'a>( i: {{input_slice_type_explicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}, u32> {
            $submac!(i, $($args)*)
        }
    );
);

/// Makes a function from a parser combination with arguments.
#[macro_export]
macro_rules! {{type_prefix}}_named_args {
    (pub $func_name:ident ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        pub fn $func_name(input: {{input_slice_type_implicit_lifetime}}, $( $arg : $typ ),*) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    (pub $func_name:ident < 'a > ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        pub fn $func_name<'a>(input: {{input_slice_type_explicit_lifetime}}, $( $arg : $typ ),*) -> nom::IResult<{{input_slice_type_explicit_lifetime}}, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    ($func_name:ident ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        fn $func_name(input: {{input_slice_type_implicit_lifetime}}, $( $arg : $typ ),*) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    ($func_name:ident < 'a > ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        fn $func_name<'a>(input: {{input_slice_type_explicit_lifetime}}, $( $arg : $typ ),*) -> nom::IResult<{{input_slice_type_explicit_lifetime}}, $return_type> {
            $submac!(input, $($args)*)
        }
    };
}

/// Makes a function from a parser combination, with attributes
///
/// The usage of this macro is almost identical to `named!`, except that
/// you also pass attributes to be attached to the generated function.
/// This is ideal for adding documentation to your parser.
///
/// ```ignore
/// // Create my_function as if you wrote it with the doc comment /// My Func
/// named_attr!(#[doc = "My Func"], my_function( {{input_slice_type_implicit_lifetime}} ) -> {{input_slice_type_implicit_lifetime}}, tag!("abcd"));
/// // Also works for pub functions, and multiple lines
/// named!(#[doc = "My Func\nRecognise abcd"], pub my_function, tag!("abcd"));
/// // Multiple attributes can be passed if required
/// named!(#[doc = "My Func"] #[inline(always)], pub my_function, tag!("abcd"));
/// ```
#[macro_export]
macro_rules! {{type_prefix}}_named_attr (
    ($(#[$attr:meta])*, $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name( i: $i ) -> nom::IResult<$i,$o,u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name( i: $i ) -> nom::IResult<$i, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name( i: $i ) -> nom::IResult<$i, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name<'a>( i: {{input_slice_type_explicit_lifetime}} ) -> nom::IResult<{{input_slice_type_explicit_lifetime}}, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, $name:ident, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name( i: {{input_slice_type_implicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name( i: $i ) -> nom::IResult<$i,$o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name( i: $i ) -> nom::IResult<$i, $o, $e> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name( i: $i ) -> nom::IResult<$i, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name( i: {{input_slice_type_implicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name<'a>( i: {{input_slice_type_explicit_lifetime}} ) -> nom::IResult<{{input_slice_type_implicit_lifetime}}, {{input_slice_type_implicit_lifetime}}, u32> {
            $submac!(i, $($args)*)
        }
    );
);

