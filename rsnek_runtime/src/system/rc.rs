//! Configure the type of reference counts through compile time feature flags.
//!
#[cfg(not(feature="rsnek_multithreaded"))]
use self::impl_single_threaded as internal;

#[cfg(feature="rsnek_multithreaded")]
use self::impl_multithreaded as internal;


pub use self::internal::StrongRc;
pub use self::internal::WeakRc;


/// Single threaded reference counts enabled by default
mod impl_single_threaded {
    use std;

    pub use std::rc::Rc as StrongRc;
    pub use std::rc::Weak as WeakRc;
}


/// Multithreaded reference counts enabled with `--cfg-feature="rsnek_multithreaded`
/// Note that they do incur a 30ns penalty (60ns vs 90ns) on the len() benchmark.
mod impl_multithreaded {
    use std;

    pub use std::sync::Arc as StrongRc;
    pub use std::sync::Weak as WeakRc;
}


