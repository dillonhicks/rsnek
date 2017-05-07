
pub use self::internal::StrongRc;
pub use self::internal::WeakRc;



#[cfg(not(feature="rsnek_multithreaded"))]
mod internal {
    use std;

    pub use std::rc::Rc as StrongRc;
    pub use std::rc::Weak as WeakRc;
}


#[cfg(feature="rsnek_multithreaded")]
mod internal{
    use std;

    pub use std::sync::Arc as StrongRc;
    pub use std::sync::Weak as WeakRc;
}


