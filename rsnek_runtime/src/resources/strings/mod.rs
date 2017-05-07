pub const PROGRAM: &'static str = include_str!("program.in");
pub const AUTHORS: &'static str = include_str!("authors.in");
pub const LICENSE: &'static str = include_str!("license.in");
pub const ABOUT:   &'static str = include_str!("about.in");
pub const BANNER:  &'static str = include_str!("banner.in");
pub const BANNER2: &'static str = include_str!("banner2.in");
pub const VERSION: &'static str = include_str!("version.in");
pub const BUILD:   &'static str = include_str!("build.cfg.in");
pub const PROMPT:  &'static str = include_str!("prompt.in");


pub const BUILTINS_MODULE: &'static str = "builtins";


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

pub const LIST_DOC_STRING: &'static str = r#"list() -> new empty list
list(iterable) -> new list initialized from iterable's items
"#;

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

pub const FUNCTION_DOC_STRING: &'static str = r#"<attribute '__doc__' of 'builtin_function_or_method' objects>
"#;

// Error Strings and format strings
// if a constant ends in a _F{N} then it is a format string with N positional arguments
pub const ERROR_NATIVE_INT_OVERFLOW: &'static str = "Python int too large to convert to rust usize";
pub const ERROR_NEG_BIT_SHIFT: &'static str = "negative shift count";
pub const ERROR_INDEX_OUT_OF_RANGE: &'static str = "index out of range";