//! # mod strings: Keep human readable text out of your code
//!
//! mod strings exists to experiment with keeping the constant strings and
//! other build resources out of the codebase for locale targeting
//! among other things.
//!
//! Two examples exist:
//!
//! - `*.in`: Which are to be included as a `!include_str()` directive. In the future, these may
//! be preprocessed by build.rs for translation, spellchecking, etc.
//!
//! - `*.rsrc.xml`: Idea to keep in mind for a more general build resource a la android.
//! With serde_xml it should be easy to create a build manifest that is available as a library
//! to all consumers.
//!
//! **Current Guidance:** Use the `*.in` format for now. Re-evaluate when we have
//! more resources and resource types.
//!

/// The name of the binary
pub const PROGRAM: &'static str = include_str!("program.in");

/// The mouth breather(s) responsible for this abomination
pub const AUTHORS: &'static str = include_str!("authors.in");

/// Because you need it to drive... and to kill
pub const LICENSE: &'static str = include_str!("license.in");

/// Let's be fair, you weren't going to read it anyway.
pub const ABOUT:   &'static str = include_str!("about.in");

/// Banners particularly aimed to piss off people with needless
/// text before you get started.
pub const BANNER:  &'static str = include_str!("banner.in");
pub const BANNER2: &'static str = include_str!("banner2.in");

/// The current build version
pub const VERSION: &'static str = include_str!("version.in");

/// The version of rustc used to build the monstrosity
pub const BUILD:   &'static str = include_str!("build.cfg.in");

/// Prompt as a service
pub const PROMPT:  &'static str = include_str!("prompt.in");

/// Static strings are love, static strings are life
pub const BUILTINS_MODULE: &'static str = "builtins";

pub const COMPILED_SOURCE_EXT: &'static str = "rsc";

/// Stol'd from CPython
/// ```ignore
/// type(1).__doc__
/// ```
pub const INT_DOC_STRING: &'static str = r#"int(x=0) -> integer
int(x, base=10) -> integer

Convert a number or string to an integer, or return 0 if no arguments
are given.  If x is a number, return x.__int__().  For floating point
numbers, this truncates towards zero.

If x is not a number or if base is given, then x must be a string,
bytes, or bytearray instance representing an integer literal in the
given base.  The literal can be preceded by '+' or '-' and be surrounded
by whitespace.  The base defaults to 10.  Valid bases are 0 and 2-36.
Base 0 means to interpret the base from the string as an integer literal.
>>> int('0b100', base=0)
4
"#;

/// Stol'd from CPython
/// ```ignore
/// type([]).__doc__
/// ```
pub const LIST_DOC_STRING: &'static str = r#"list() -> new empty list
list(iterable) -> new list initialized from iterable's items
"#;

/// Stol'd from CPython
/// ```ignore
/// type('').__doc__
/// ```
pub const STR_DOC_STRING: &'static str = r#"str(object='') -> str
str(bytes_or_buffer[, encoding[, errors]]) -> str

Create a new string object from the given object. If encoding or
errors is specified, then the object must expose a data buffer
that will be decoded using the given encoding and error handler.
Otherwise, returns the result of object.__str__() (if defined)
or repr(object).
encoding defaults to sys.getdefaultencoding().
errors defaults to 'strict'.
"#;

/// Stol'd from CPython
/// ```ignore
/// type(len).__doc__
/// ```
pub const FUNCTION_DOC_STRING: &'static str = r#"<attribute '__doc__' of 'builtin_function_or_method' objects>
"#;


/// Size matters when it is sufficiently large
pub const ERROR_NATIVE_INT_OVERFLOW: &'static str = "Python int too large to convert to rust usize";

/// You cant go back in time, unbreak hearts, or take back your shifted bits
pub const ERROR_NEG_BIT_SHIFT: &'static str = "negative shift count";

/// The comment for this constant is conveniently out of range
pub const ERROR_INDEX_OUT_OF_RANGE: &'static str = "index out of range";