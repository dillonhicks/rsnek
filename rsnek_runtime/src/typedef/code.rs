use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use runtime::{Runtime};

use object::{self, RtValue};
use object::selfref::{self, SelfRef};
use object::typing;
use object::method;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub struct PyCodeType {}


impl typing::BuiltinType for PyCodeType {
    type T = PyCode;
    type V = native::Code;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyCodeType::inject_selfref(PyCodeType::alloc(value))
    }

    fn init_type() -> Self {
        PyCodeType {}
    }


    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Code(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Str(ref string) => {
                string.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }


    fn alloc(value: Self::V) -> Self::T {
        PyCode {
            value: CodeValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct CodeValue(pub native::Code);
pub type PyCode = RtValue<CodeValue>;


impl fmt::Debug for PyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<code {:?}>", self.value.0.co_name)
    }
}


impl object::PyAPI for PyCode {}
impl method::New for PyCode {}
impl method::Init for PyCode {}
impl method::Delete for PyCode {}
impl method::GetAttr for PyCode {}
impl method::GetAttribute for PyCode {}
impl method::SetAttr for PyCode {}
impl method::DelAttr for PyCode {}
impl method::Id for PyCode {}
impl method::Is for PyCode {}
impl method::IsNot for PyCode {}
impl method::Hashed for PyCode {}
impl method::StringCast for PyCode {}
impl method::BytesCast for PyCode {}
impl method::StringFormat for PyCode {}
impl method::StringRepresentation for PyCode {}
impl method::Equal for PyCode {}
impl method::NotEqual for PyCode {}
impl method::LessThan for PyCode {}
impl method::LessOrEqual for PyCode {}
impl method::GreaterOrEqual for PyCode {}
impl method::GreaterThan for PyCode {}
impl method::BooleanCast for PyCode {}
impl method::IntegerCast for PyCode {}
impl method::FloatCast for PyCode {}
impl method::ComplexCast for PyCode {}
impl method::Rounding for PyCode {}
impl method::Index for PyCode {}
impl method::NegateValue for PyCode {}
impl method::AbsValue for PyCode {}
impl method::PositiveValue for PyCode {}
impl method::InvertValue for PyCode {}
impl method::Add for PyCode {}
impl method::BitwiseAnd for PyCode {}
impl method::DivMod for PyCode {}
impl method::FloorDivision for PyCode {}
impl method::LeftShift for PyCode {}
impl method::Modulus for PyCode {}
impl method::Multiply for PyCode {}
impl method::MatrixMultiply for PyCode {}
impl method::BitwiseOr for PyCode {}
impl method::Pow for PyCode {}
impl method::RightShift for PyCode {}
impl method::Subtract for PyCode {}
impl method::TrueDivision for PyCode {}
impl method::XOr for PyCode {}
impl method::ReflectedAdd for PyCode {}
impl method::ReflectedBitwiseAnd for PyCode {}
impl method::ReflectedDivMod for PyCode {}
impl method::ReflectedFloorDivision for PyCode {}
impl method::ReflectedLeftShift for PyCode {}
impl method::ReflectedModulus for PyCode {}
impl method::ReflectedMultiply for PyCode {}
impl method::ReflectedMatrixMultiply for PyCode {}
impl method::ReflectedBitwiseOr for PyCode {}
impl method::ReflectedPow for PyCode {}
impl method::ReflectedRightShift for PyCode {}
impl method::ReflectedSubtract for PyCode {}
impl method::ReflectedTrueDivision for PyCode {}
impl method::ReflectedXOr for PyCode {}
impl method::InPlaceAdd for PyCode {}
impl method::InPlaceBitwiseAnd for PyCode {}
impl method::InPlaceDivMod for PyCode {}
impl method::InPlaceFloorDivision for PyCode {}
impl method::InPlaceLeftShift for PyCode {}
impl method::InPlaceModulus for PyCode {}
impl method::InPlaceMultiply for PyCode {}
impl method::InPlaceMatrixMultiply for PyCode {}
impl method::InPlaceBitwiseOr for PyCode {}
impl method::InPlacePow for PyCode {}
impl method::InPlaceRightShift for PyCode {}
impl method::InPlaceSubtract for PyCode {}
impl method::InPlaceTrueDivision for PyCode {}
impl method::InPlaceXOr for PyCode {}
impl method::Contains for PyCode {}
impl method::Iter for PyCode {}
impl method::Call for PyCode {}
impl method::Length for PyCode {}
impl method::LengthHint for PyCode {}
impl method::Next for PyCode {}
impl method::Reversed for PyCode {}
impl method::GetItem for PyCode {}
impl method::SetItem for PyCode {}
impl method::DeleteItem for PyCode {}
impl method::Count for PyCode {}
impl method::Append for PyCode {}
impl method::Extend for PyCode {}
impl method::Pop for PyCode {}
impl method::Remove for PyCode {}
impl method::IsDisjoint for PyCode {}
impl method::AddItem for PyCode {}
impl method::Discard for PyCode {}
impl method::Clear for PyCode {}
impl method::Get for PyCode {}
impl method::Keys for PyCode {}
impl method::Values for PyCode {}
impl method::Items for PyCode {}
impl method::PopItem for PyCode {}
impl method::Update for PyCode {}
impl method::SetDefault for PyCode {}
impl method::Await for PyCode {}
impl method::Send for PyCode {}
impl method::Throw for PyCode {}
impl method::Close for PyCode {}
impl method::Exit for PyCode {}
impl method::Enter for PyCode {}
impl method::DescriptorGet for PyCode {}
impl method::DescriptorSet for PyCode {}
impl method::DescriptorSetName for PyCode {}
