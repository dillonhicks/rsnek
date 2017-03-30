pub mod method;
pub mod selfref;
pub mod typing;
pub mod operator;
pub mod number;
pub mod collection;
pub mod compare;
pub mod coroutine;
pub mod context;
pub mod attribute;


/// Runtime Value delegate that holds its own self reference
pub type RtValue<T> = selfref::RefCountedValue<T, selfref::RefCount>;

/// Trait defining all possible method and props
pub trait PyAPI:
method::New +
method::Init +
method::Delete +
method::GetAttr +
method::GetAttribute +
method::SetAttr +
method::DelAttr +
method::Id +
method::Is +
method::IsNot +
method::Hashed +
method::StringCast +
method::BytesCast +
method::StringFormat +
method::StringRepresentation +
method::Equal +
method::NotEqual +
method::LessThan +
method::LessOrEqual +
method::GreaterOrEqual +
method::GreaterThan +
method::BooleanCast +
method::IntegerCast +
method::FloatCast +
method::ComplexCast +
method::Rounding +
method::Index +
method::NegateValue +
method::AbsValue +
method::PositiveValue +
method::InvertValue +
method::Add +
method::BitwiseAnd +
method::DivMod +
method::FloorDivision +
method::LeftShift +
method::Modulus +
method::Multiply +
method::MatrixMultiply +
method::BitwiseOr +
method::Pow +
method::RightShift +
method::Subtract +
method::TrueDivision +
method::XOr +
method::ReflectedAdd +
method::ReflectedBitwiseAnd +
method::ReflectedDivMod +
method::ReflectedFloorDivision +
method::ReflectedLeftShift +
method::ReflectedModulus +
method::ReflectedMultiply +
method::ReflectedMatrixMultiply +
method::ReflectedBitwiseOr +
method::ReflectedPow +
method::ReflectedRightShift +
method::ReflectedSubtract +
method::ReflectedTrueDivision +
method::ReflectedXOr +
method::InPlaceAdd +
method::InPlaceBitwiseAnd +
method::InPlaceDivMod +
method::InPlaceFloorDivision +
method::InPlaceLeftShift +
method::InPlaceModulus +
method::InPlaceMultiply +
method::InPlaceMatrixMultiply +
method::InPlaceBitwiseOr +
method::InPlacePow +
method::InPlaceRightShift +
method::InPlaceSubtract +
method::InPlaceTrueDivision +
method::InPlaceXOr +
method::Contains +
method::Iter +
method::Call +
method::Length +
method::LengthHint +
method::Next +
method::Reversed +
method::GetItem +
method::SetItem +
method::DeleteItem +
method::Count +
method::Append +
method::Extend +
method::Pop +
method::Remove +
method::IsDisjoint +
method::AddItem +
method::Discard +
method::Clear +
method::Get +
method::Keys +
method::Values +
method::Items +
method::PopItem +
method::Update +
method::SetDefault +
method::Await +
method::Send +
method::Throw +
method::Close +
method::Exit +
method::Enter +
method::DescriptorGet +
method::DescriptorSet +
method::DescriptorSetName {}

macro_rules! object_api {
    ($typename:ident) => {
        impl PyAPI for $typename {}
        impl method::New for $typename {}
        impl method::Init for $typename {}
        impl method::Delete for $typename {}
        impl method::GetAttr for $typename {}
        impl method::GetAttribute for $typename {}
        impl method::SetAttr for $typename {}
        impl method::DelAttr for $typename {}
        impl method::Id for $typename {}
        impl method::Is for $typename {}
        impl method::IsNot for $typename {}
        impl method::Hashed for $typename {}
        impl method::StringCast for $typename {}
        impl method::BytesCast for $typename {}
        impl method::StringFormat for $typename {}
        impl method::StringRepresentation for $typename {}
        impl method::Equal for $typename {}
        impl method::NotEqual for $typename {}
        impl method::LessThan for $typename {}
        impl method::LessOrEqual for $typename {}
        impl method::GreaterOrEqual for $typename {}
        impl method::GreaterThan for $typename {}
        impl method::BooleanCast for $typename {}
        impl method::IntegerCast for $typename {}
        impl method::FloatCast for $typename {}
        impl method::ComplexCast for $typename {}
        impl method::Rounding for $typename {}
        impl method::Index for $typename {}
        impl method::NegateValue for $typename {}
        impl method::AbsValue for $typename {}
        impl method::PositiveValue for $typename {}
        impl method::InvertValue for $typename {}
        impl method::Add for $typename {}
        impl method::BitwiseAnd for $typename {}
        impl method::DivMod for $typename {}
        impl method::FloorDivision for $typename {}
        impl method::LeftShift for $typename {}
        impl method::Modulus for $typename {}
        impl method::Multiply for $typename {}
        impl method::MatrixMultiply for $typename {}
        impl method::BitwiseOr for $typename {}
        impl method::Pow for $typename {}
        impl method::RightShift for $typename {}
        impl method::Subtract for $typename {}
        impl method::TrueDivision for $typename {}
        impl method::XOr for $typename {}
        impl method::ReflectedAdd for $typename {}
        impl method::ReflectedBitwiseAnd for $typename {}
        impl method::ReflectedDivMod for $typename {}
        impl method::ReflectedFloorDivision for $typename {}
        impl method::ReflectedLeftShift for $typename {}
        impl method::ReflectedModulus for $typename {}
        impl method::ReflectedMultiply for $typename {}
        impl method::ReflectedMatrixMultiply for $typename {}
        impl method::ReflectedBitwiseOr for $typename {}
        impl method::ReflectedPow for $typename {}
        impl method::ReflectedRightShift for $typename {}
        impl method::ReflectedSubtract for $typename {}
        impl method::ReflectedTrueDivision for $typename {}
        impl method::ReflectedXOr for $typename {}
        impl method::InPlaceAdd for $typename {}
        impl method::InPlaceBitwiseAnd for $typename {}
        impl method::InPlaceDivMod for $typename {}
        impl method::InPlaceFloorDivision for $typename {}
        impl method::InPlaceLeftShift for $typename {}
        impl method::InPlaceModulus for $typename {}
        impl method::InPlaceMultiply for $typename {}
        impl method::InPlaceMatrixMultiply for $typename {}
        impl method::InPlaceBitwiseOr for $typename {}
        impl method::InPlacePow for $typename {}
        impl method::InPlaceRightShift for $typename {}
        impl method::InPlaceSubtract for $typename {}
        impl method::InPlaceTrueDivision for $typename {}
        impl method::InPlaceXOr for $typename {}
        impl method::Contains for $typename {}
        impl method::Iter for $typename {}
        impl method::Call for $typename {}
        impl method::Length for $typename {}
        impl method::LengthHint for $typename {}
        impl method::Next for $typename {}
        impl method::Reversed for $typename {}
        impl method::GetItem for $typename {}
        impl method::SetItem for $typename {}
        impl method::DeleteItem for $typename {}
        impl method::Count for $typename {}
        impl method::Append for $typename {}
        impl method::Extend for $typename {}
        impl method::Pop for $typename {}
        impl method::Remove for $typename {}
        impl method::IsDisjoint for $typename {}
        impl method::AddItem for $typename {}
        impl method::Discard for $typename {}
        impl method::Clear for $typename {}
        impl method::Get for $typename {}
        impl method::Keys for $typename {}
        impl method::Values for $typename {}
        impl method::Items for $typename {}
        impl method::PopItem for $typename {}
        impl method::Update for $typename {}
        impl method::SetDefault for $typename {}
        impl method::Await for $typename {}
        impl method::Send for $typename {}
        impl method::Throw for $typename {}
        impl method::Close for $typename {}
        impl method::Exit for $typename {}
        impl method::Enter for $typename {}
        impl method::DescriptorGet for $typename {}
        impl method::DescriptorSet for $typename {}
        impl method::DescriptorSetName for $typename {}

    }
}

struct PyNewType;
object_api!(PyNewType);


#[cfg(all(feature="old", test))]
mod impl_object {
    use num::{Zero, FromPrimitive};
    use std::borrow::Borrow;

    use super::*;
    use super::selfref::SelfRef;
    use typedef::native;
    use typedef::boolean::{PyBoolean, BoolValue};
    use typedef::objectref::{WeakObjectRef, ObjectRef};
    use typedef::builtin::Builtin;

    impl PyBoolean {
        pub fn unmanaged(value: bool) -> Self {
            PyBoolean {
                value: if value {
                    BoolValue(native::Integer::from_usize(1).unwrap())
                } else {
                    BoolValue(native::Integer::zero())
                },
                rc: selfref::RefCount::default(),
            }
        }
        pub fn managed(value: bool) -> ObjectRef {
            let rtvalue = PyBoolean::unmanaged(value);
            let objref = ObjectRef::new(Builtin::Bool(rtvalue));

            let new = objref.clone();
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
            return;
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
