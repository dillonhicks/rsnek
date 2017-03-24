use typedef::native;
use object::{self, RtValue};
use object::method;

////
////struct PyBuiltinFunction {
////    locals: (),
////    consts: (),
////    etc: ()
////}
//struct PyNativeFunction {
//
//}
//
//struct PyMethodWrapper {
//
//}
//
//
//struct PyFunction {
//    code: (),
//    args: (),
//    locals: (),
//    consts: ()
//}
//
//enum Function {
//    Builtin(PyNativeFunction),
//
//}
//
//struct PyMethod {
//
//}


pub struct TypeValue(pub native::Type);
pub type PyType = RtValue<TypeValue>;


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::PyAPI for PyType {}
impl method::New for PyType {}
impl method::Init for PyType {}
impl method::Delete for PyType {}
impl method::GetAttr for PyType {}
impl method::GetAttribute for PyType {}
impl method::SetAttr for PyType {}
impl method::DelAttr for PyType {}
impl method::Id for PyType {}
impl method::Is for PyType {}
impl method::IsNot for PyType {}
impl method::Hashed for PyType {}
impl method::Equal for PyType {}
impl method::NotEqual for PyType {}
impl method::LessThan for PyType {}
impl method::LessOrEqual for PyType {}
impl method::GreaterOrEqual for PyType {}
impl method::GreaterThan for PyType {}
impl method::BooleanCast for PyType {}
impl method::IntegerCast for PyType {}
impl method::FloatCast for PyType {}
impl method::ComplexCast for PyType {}
impl method::Rounding for PyType {}
impl method::Index for PyType {}
impl method::NegateValue for PyType {}
impl method::AbsValue for PyType {}
impl method::PositiveValue for PyType {}
impl method::InvertValue for PyType {}
impl method::Add for PyType {}
impl method::BitwiseAnd for PyType {}
impl method::DivMod for PyType {}
impl method::FloorDivision for PyType {}
impl method::LeftShift for PyType {}
impl method::Modulus for PyType {}
impl method::Multiply for PyType {}
impl method::MatrixMultiply for PyType {}
impl method::BitwiseOr for PyType {}
impl method::Pow for PyType {}
impl method::RightShift for PyType {}
impl method::Subtract for PyType {}
impl method::TrueDivision for PyType {}
impl method::XOr for PyType {}
impl method::ReflectedAdd for PyType {}
impl method::ReflectedBitwiseAnd for PyType {}
impl method::ReflectedDivMod for PyType {}
impl method::ReflectedFloorDivision for PyType {}
impl method::ReflectedLeftShift for PyType {}
impl method::ReflectedModulus for PyType {}
impl method::ReflectedMultiply for PyType {}
impl method::ReflectedMatrixMultiply for PyType {}
impl method::ReflectedBitwiseOr for PyType {}
impl method::ReflectedPow for PyType {}
impl method::ReflectedRightShift for PyType {}
impl method::ReflectedSubtract for PyType {}
impl method::ReflectedTrueDivision for PyType {}
impl method::ReflectedXOr for PyType {}
impl method::InPlaceAdd for PyType {}
impl method::InPlaceBitwiseAnd for PyType {}
impl method::InPlaceDivMod for PyType {}
impl method::InPlaceFloorDivision for PyType {}
impl method::InPlaceLeftShift for PyType {}
impl method::InPlaceModulus for PyType {}
impl method::InPlaceMultiply for PyType {}
impl method::InPlaceMatrixMultiply for PyType {}
impl method::InPlaceBitwiseOr for PyType {}
impl method::InPlacePow for PyType {}
impl method::InPlaceRightShift for PyType {}
impl method::InPlaceSubtract for PyType {}
impl method::InPlaceTrueDivision for PyType {}
impl method::InPlaceXOr for PyType {}
impl method::Contains for PyType {}
impl method::Iter for PyType {}
impl method::Call for PyType {}
impl method::Length for PyType {}
impl method::LengthHint for PyType {}
impl method::Next for PyType {}
impl method::Reversed for PyType {}
impl method::GetItem for PyType {}
impl method::SetItem for PyType {}
impl method::DeleteItem for PyType {}
impl method::Count for PyType {}
impl method::Append for PyType {}
impl method::Extend for PyType {}
impl method::Pop for PyType {}
impl method::Remove for PyType {}
impl method::IsDisjoint for PyType {}
impl method::AddItem for PyType {}
impl method::Discard for PyType {}
impl method::Clear for PyType {}
impl method::Get for PyType {}
impl method::Keys for PyType {}
impl method::Values for PyType {}
impl method::Items for PyType {}
impl method::PopItem for PyType {}
impl method::Update for PyType {}
impl method::SetDefault for PyType {}
impl method::Await for PyType {}
impl method::Send for PyType {}
impl method::Throw for PyType {}
impl method::Close for PyType {}
impl method::Exit for PyType {}
impl method::Enter for PyType {}
impl method::DescriptorGet for PyType {}
impl method::DescriptorSet for PyType {}
impl method::DescriptorSetName for PyType {}

