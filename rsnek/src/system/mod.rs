//! Primitive type aliases for the concrete backing types, concurrency, and interfaces to the
//! the underlying platform.
mod exit;
mod rc;
mod thread;
pub mod primitives;

pub use self::exit::ExitCode;
pub use self::rc::{StrongRc, WeakRc};
pub use self::thread::{ThreadModel, Thread, GreenThread, Pthread};

use runtime::Runtime;

// TODO: make configurable
/// The maximum number of call frames allowed before a RecursionError
/// is thrown.
pub const RECURSION_LIMIT: usize = 256;

/// The totally standard args string vector
pub type Argv<'a> =  &'a [&'a str];

/// Alias for a function pointer to the main function
pub type MainFn = Fn(&Runtime) -> i64;

/// Reference of `MainFn`
pub type MainFnRef<'a> = &'a Fn(&Runtime) -> (i64);


/// Wrapper around the `MainFnRef` so we have an owned
/// type on which we can specify clone semantics for
/// `Send` and `Sync` traits which allows the function reference
/// to be shared across threads.
#[derive(Clone)]
pub struct SharedMainFnRef<'a>(pub MainFnRef<'a>);
unsafe impl<'a> Sync for SharedMainFnRef<'a> {}
unsafe impl<'a> Send for SharedMainFnRef<'a> {}


