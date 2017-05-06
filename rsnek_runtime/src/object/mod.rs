pub mod method;
pub mod selfref;
pub mod typing;
mod rtobject;

pub use self::rtobject::{RtObject, WeakRtObject};


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
