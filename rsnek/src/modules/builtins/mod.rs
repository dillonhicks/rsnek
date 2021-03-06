//! `builtins` - The builtin types, constants, and functions that are available at anytime
//! and in any namespace.
//!
mod len;
mod print;
mod typefn;
mod str;
mod int;
mod any;
mod all;
mod and;
mod or;
mod list;
mod globals;
mod tuple;
mod types;

pub use self::all::{AllFn, iterator_all};
pub use self::and::logical_and;
pub use self::any::{AnyFn, iterator_any};
pub use self::globals::GlobalsFn;
pub use self::int::IntFn;
pub use self::len::LenFn;
pub use self::list::ListFn;
pub use self::or::logical_or;
pub use self::print::PrintFn;
pub use self::str::StrFn;
pub use self::tuple::TupleFn;
pub use self::typefn::TypeFn;
pub use self::types::Type;