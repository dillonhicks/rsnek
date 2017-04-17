use std::fmt::{self, Display};


#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq, Ord, Hash, Default)]
pub struct Micros(pub i64);


impl Display for Micros {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} us", self.0)
    }
}