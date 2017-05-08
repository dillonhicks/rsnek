use std::fmt::{self, Display};

use nom;
use ::token::Id;
use ::slice::TkSlice;

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq, Ord, Hash, Default)]
pub struct Micros(pub i64);


impl Display for Micros {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} us", self.0)
    }
}


/// For intra statement and expression space filtering
tk_named!(pub filter_non_critical_python_whitespace, drop_tokens!(&[
    Id::Space,
    Id::Tab,
    // A LineContinuation is a newline after a backslash
    Id::LineContinuation,
    Id::Comment
]));