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
//! macro_rules! tk_named (
//!   ($name:ident, $submac:ident!( $($args:tt)* )) => (
//!     fn $name<'a>( i: TkSlice<'a> ) -> nom::IResult<'a,TkSlice<'a>, TkSlice<'a>> {
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
use nom;
use slice::TkSlice;


#[macro_export]
macro_rules! tk_tag (
  ($i:expr, $tag: expr) => (
    {
      use nom::{Compare,CompareResult,InputLength,Slice};
      use $crate::traits::redefs_nom::InputLength;
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
/// named!(my_function( TkSlice<'a> ) -> TkSlice<'a>, tag!("abcd"));
/// // first type parameter is input, second is output
/// named!(my_function<TkSlice<'a>, TkSlice<'a>>,     tag!("abcd"));
/// // will have TkSlice<'a> as input type, TkSlice<'a> as output type
/// named!(my_function,                   tag!("abcd"));
/// // will use TkSlice<'a> as input type (use this if the compiler
/// // complains about lifetime issues
/// named!(my_function<TkSlice<'a>>,            tag!("abcd"));
/// //prefix them with 'pub' to make the functions public
/// named!(pub my_function,               tag!("abcd"));
/// ```
#[macro_export]
macro_rules! tk_named (
    (#$($args:tt)*) => (
        tk_named_attr!(#$($args)*);
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
        fn $name<'a>( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        fn $name( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, TkSlice<'a>, u32> {
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
        pub fn $name( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident, $submac:ident!( $($args:tt)* )) => (
        #[allow(unused_variables)]
        pub fn $name<'a>( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, TkSlice<'a>, u32> {
            $submac!(i, $($args)*)
        }
    );
);

/// Makes a function from a parser combination with arguments.
#[macro_export]
macro_rules! tk_named_args {
    (pub $func_name:ident ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        pub fn $func_name(input: TkSlice<'a>, $( $arg : $typ ),*) -> nom::IResult<TkSlice<'a>, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    (pub $func_name:ident < 'a > ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        pub fn $func_name<'a>(input: TkSlice<'a>, $( $arg : $typ ),*) -> nom::IResult<TkSlice<'a>, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    ($func_name:ident ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        fn $func_name(input: TkSlice<'a>, $( $arg : $typ ),*) -> nom::IResult<TkSlice<'a>, $return_type> {
            $submac!(input, $($args)*)
        }
    };
    ($func_name:ident < 'a > ( $( $arg:ident : $typ:ty ),* ) < $return_type:ty > , $submac:ident!( $($args:tt)* ) ) => {
        fn $func_name<'a>(input: TkSlice<'a>, $( $arg : $typ ),*) -> nom::IResult<TkSlice<'a>, $return_type> {
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
/// named_attr!(#[doc = "My Func"], my_function( TkSlice<'a> ) -> TkSlice<'a>, tag!("abcd"));
/// // Also works for pub functions, and multiple lines
/// named!(#[doc = "My Func\nRecognise abcd"], pub my_function, tag!("abcd"));
/// // Multiple attributes can be passed if required
/// named!(#[doc = "My Func"] #[inline(always)], pub my_function, tag!("abcd"));
/// ```
#[macro_export]
macro_rules! tk_named_attr (
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
        fn $name<'a>( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, $name:ident, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        fn $name( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, TkSlice<'a>, u32> {
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
        pub fn $name( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, $o, u32> {
            $submac!(i, $($args)*)
        }
    );
    ($(#[$attr:meta])*, pub $name:ident, $submac:ident!( $($args:tt)* )) => (
        $(#[$attr])*
        pub fn $name<'a>( i: TkSlice<'a> ) -> nom::IResult<TkSlice<'a>, TkSlice<'a>, u32> {
            $submac!(i, $($args)*)
        }
    );
);

    //! Method macro combinators
//!
//! These macros make parsers as methods of structs
//! and that can take methods of structs to call
//! as parsers.
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
//! But when used as methods in other combinators, are used
//! like this:
//!
//! ```ignore
//! method!(my_function<Parser<'a> >, self, tag!("abcd"));
//! ```
//!
//! Internally, other combinators will rewrite
//! that call to pass the input as second argument:
//!
//! ```ignore
//! macro_rules! method (
//!   ($name:ident<$a:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
//!     fn $name( $self_: $a, i: &[u8] ) -> $crate::IResult<&[u8], &[u8]> {
//!       $submac!(i, $($args)*)
//!     }
//!   );
//! );
//! ```
//!
//! The `method!` macro is similar to the `named!` macro in the macros module.
//! While `named!` will create a parser function, `method!` will create a parser
//! method on the struct it is defined in.
//!
//! Compared to the `named!` macro there are a few differences in how they are
//! invoked. A `method!` invocation always has to have the type of `self`
//! declared and it can't be a reference due to Rust's borrow lifetime
//! restrictions:
//! ```ignore
//! //                  -`self`'s type-
//! method!(method_name<  Parser<'a> >, ...);
//! ```
//! `self`'s type always comes first.
//! The next difference is you have to input the self struct. Due to Rust's
//! macro hygiene the macro can't declare it on it's own.
//! ```ignore
//! //                                                 -self-
//! method!(method_name<Parser<'a>, &'a str, &'a str>, self, ...);
//! ```
//! When making a parsing struct with parsing methods, due to the static borrow
//! checker,calling any parsing methods on self (or any other parsing struct)
//! will cause self to be moved for the rest of the method.To get around this
//! restriction all self is moved into the called method and then the called
//! method will return self to the caller.
//!
//! To call a method on self you need to use the `call_m!` macro. For example:
//! ```ignore
//! struct<'a> Parser<'a> {
//!   parsed: &'a str,
//! }
//! impl<'a> Parser<'a> {
//!   // Constructor omitted for brevity
//!   method!(take4<Parser<'a>, &'a str, &'a str>, self, take!(4));
//!   method!(caller<Parser<'a>, &'a str, &'a str>, self, call_m!(self.take4));
//! }
//! ```
//! More complicated combinations still mostly look the same as their `named!`
//! counterparts:
//! ```ignore
//!    method!(pub simple_chain<&mut Parser<'a>, &'a str, &'a str>, self,
//!      do_parse!(
//!             call_m!(self.tag_abc)                                        >>
//!             call_m!(self.tag_def)                                        >>
//!             call_m!(self.tag_ghi)                                        >>
//!       last: map!(call_m!(self.simple_peek), |parsed| sb.parsed = parsed) >>
//!       (last)
//!      )
//!    );
//! ```
//! The three additions to method definitions to remember are:
//! 1. Specify `self`'s type
//! 2. Pass `self` to the macro
//! 4. Call parser methods using the `call_m!` macro.

/// Makes a method from a parser combination
///
/// The must be set up because the compiler needs
/// the information
///
/// ```ignore
/// tk_method!(my_function<Parser<'a> >( TkSlice<'a> ) -> TkSlice<'a>, tag!("abcd"));
/// // first type parameter is `self`'s type, second is input, third is output
/// tk_method!(my_function<Parser<'a>, TkSlice<'a>, TkSlice<'a>>,     tag!("abcd"));
/// //prefix them with 'pub' to make the methods public
/// tk_method!(pub my_function<Parser<'a>,TkSlice<'a>, TkSlice<'a>>, tag!("abcd"));
/// ```
#[macro_export]
macro_rules! tk_method (
  // Non-public immutable self
  ($name:ident<$a:ty>( $i:ty ) -> $o:ty, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  ($name:ident<$a:ty,$i:ty,$o:ty,$e:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( $self_: $a, i: $i ) -> ($a, $nom::IResult<$i, $o, $e>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  ($name:ident<$a:ty,$i:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  ($name:ident<$a:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, $o, u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  ($name:ident<$a:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, TkSlice<'a>, u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  // Public immutable self
  (pub $name:ident<$a:ty>( $i:ty ) -> $o:ty, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  (pub $name:ident<$a:ty,$i:ty,$o:ty,$e:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( $self_: $a, i: $i ) -> ($a, $nom::IResult<$i, $o, $e>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  (pub $name:ident<$a:ty,$i:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( $self_: $a,i: $i ) -> ($a, $nom::IResult<$i,$o,u32>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  (pub $name:ident<$a:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, $o, u32>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  (pub $name:ident<$a:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, TkSlice<'a>, u32>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  // Non-public mutable self
  ($name:ident<$a:ty>( $i:ty ) -> $o:ty, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( mut $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  ($name:ident<$a:ty,$i:ty,$o:ty,$e:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( mut $self_: $a, i: $i ) -> ($a, $nom::IResult<$i, $o, $e>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
      }
  );
  ($name:ident<$a:ty,$i:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( mut $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  ($name:ident<$a:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( mut $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, $o, u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  ($name:ident<$a:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( mut $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, TkSlice<'a>, u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  // Public mutable self
  (pub $name:ident<$a:ty>( $i:ty ) -> $o:ty, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      pub fn $name( mut $self_: $a, i: $i ) -> ($a, $nom::IResult<$i,$o,u32>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  (pub $name:ident<$a:ty,$i:ty,$o:ty,$e:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
      #[allow(unused_variables)]
      fn $name( mut $self_: $a, i: $i ) -> ($a, $nom::IResult<$i, $o, $e>) {
        let result = $submac!(i, $($args)*);
        ($self_, result)
      }
  );
  (pub $name:ident<$a:ty,$i:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( mut $self_: $a,i: $i ) -> ($a, $nom::IResult<$i,$o,u32>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  (pub $name:ident<$a:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( mut $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, $o, u32>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  (pub $name:ident<$a:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( mut $self_: $a, i: TkSlice<'a> ) -> ($a, $nom::IResult<TkSlice<'a>, TkSlice<'a>, u32>) {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
);