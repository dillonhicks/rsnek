pub mod method;
pub mod model;
pub mod selfref;
pub mod typing;
pub mod identity;
pub mod operator;
pub mod hashed;
pub mod number;
pub mod collection;
pub mod compare;
pub mod coroutine;
pub mod context;


/// Runtime Value delegate that holds its own self reference
pub type RtValue<T> = selfref::RefCountedValue<T, selfref::RefCount>;


#[cfg(test)]
mod impl_object {
    use num::{Zero,FromPrimitive};
    use std::borrow::Borrow;

    use super::*;
    use super::selfref::SelfRef;
    use typedef::native;
    use typedef::boolean::PyBoolean;
    use typedef::objectref::{WeakObjectRef, ObjectRef};
    use typedef::builtin::Builtin;

    impl PyBoolean {
        pub fn unmanaged(value: bool) -> Self{
            PyBoolean {
                value: if value {native::Integer::from_usize(1).unwrap()} else {native::Integer::zero()},
                rc: selfref::RefCount::default()
            }
        }
        pub fn managed(value: bool) -> ObjectRef {
            let rtvalue = PyBoolean::unmanaged(value);
            let objref = ObjectRef::new(Builtin::Bool(rtvalue));

            let new =  objref.clone();
            let builtin: &Box<Builtin> = objref.0.borrow();
            let bool: &PyBoolean = builtin.bool().unwrap();
            bool.rc.set(&objref.clone());
            new
        }
    }

    /// Gist of this test is to ensure that the SelfRef, ObjectRef, and WeakObjectRef
    /// machinery is working as intended so that SelfRefs do not cause a
    #[test]
    fn test_refcount() {
        let objref = PyBoolean::managed(false);
        recurse_refcount(&objref, 1, 10);
        let builtin: &Box<Builtin> = objref.0.borrow();
        let bool: &PyBoolean = builtin.bool().unwrap();
        println!("strong: {}; weak: {}", bool.rc.strong_count(), bool.rc.weak_count());

    }

    fn recurse_weak(bool: &PyBoolean, weakref: WeakObjectRef, depth: usize, max: usize) {
        println!("rweak: {}; strong: {}; weak: {}", depth, bool.rc.strong_count(), bool.rc.weak_count());

        if max == depth {
            assert_eq!(bool.rc.weak_count(), native::Integer::from_usize(1 + depth).unwrap());
            return
        } else {
            recurse_weak(bool, bool.rc.get(), depth + 1, max)
        }
    }

    fn recurse_refcount(objref: &ObjectRef, depth: usize, max: usize) {
        let builtin: &Box<Builtin> = objref.0.borrow();
        let bool: &PyBoolean = builtin.bool().unwrap();
        println!("depth: {}; strong: {}; weak: {}", depth, bool.rc.strong_count(), bool.rc.weak_count());

        if max == depth {
            assert_eq!(bool.rc.strong_count(), native::Integer::from_usize(1 + depth).unwrap());
            assert_eq!(bool.rc.weak_count(), native::Integer::from_usize(1).unwrap());
            recurse_weak(bool, bool.rc.get(), 1, max)
        } else {
            recurse_refcount(&objref.clone(), depth + 1, max)
        }
    }

}