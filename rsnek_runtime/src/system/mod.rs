///! Primitive type aliases for the concrete backing types, concurrency, and interfaces to the
///! the underlying platform.
mod rc;
pub mod primitives;

pub use self::rc::{StrongRc, WeakRc};