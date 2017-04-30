use std::ops::Deref;
use std::borrow::Borrow;
use std::fmt;

use object::{self, PyAPI, RtValue, typing, method};
use object::selfref::{self, SelfRef};
use error::Error;
use runtime::Runtime;
use traits::{BooleanProvider, IntegerProvider, NoneProvider};

use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

pub const FRAME_MAX_BLOCKS: usize = 20;


#[derive(Clone)]
pub struct PyFrameType {}


impl typing::BuiltinType for PyFrameType {
    type T = PyFrame;
    type V = native::Frame;

    fn init_type() -> Self {
        PyFrameType {}
    }

    fn alloc(frame: Self::V) -> Self::T {
        PyFrame {
            value: FrameValue(frame),
            rc: selfref::RefCount::default(),
        }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Frame(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Frame(ref boolean) => {
                boolean.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyFrameType::inject_selfref(PyFrameType::alloc(value))
    }

}

// ---------------
//  stdlib traits
// ---------------

impl fmt::Display for PyFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}

impl fmt::Debug for PyFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


#[derive(Debug, Clone)]
pub struct FrameValue(pub native::Frame);
pub type PyFrame = RtValue<FrameValue>;


impl PyAPI for PyFrame { }
impl method::New for PyFrame { }
impl method::Init for PyFrame { }
impl method::Delete for PyFrame { }

impl method::GetAttr for PyFrame {
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = name.0.borrow();

        let attr: &str = match builtin.deref() {
            &Builtin::Str(ref string) => &string.value.0,
            other => return Err(Error::typerr(
                &string_error_bad_attr_type!("str", other.debug_name()))),
        };

        match attr {
            "f_back" => Ok(self.value.0.f_back.clone()),
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!("object", missing)))
        }
    }
}

impl method::GetAttribute for PyFrame { }
impl method::SetAttr for PyFrame { }
impl method::DelAttr for PyFrame { }
impl method::Hashed for PyFrame { }
impl method::StringCast for PyFrame { }
impl method::BytesCast for PyFrame { }
impl method::StringFormat for PyFrame { }
impl method::StringRepresentation for PyFrame { }
impl method::Equal for PyFrame { }
impl method::NotEqual for PyFrame { }
impl method::LessThan for PyFrame { }
impl method::LessOrEqual for PyFrame { }
impl method::GreaterOrEqual for PyFrame { }
impl method::GreaterThan for PyFrame { }
impl method::BooleanCast for PyFrame { }
impl method::IntegerCast for PyFrame { }
impl method::FloatCast for PyFrame { }
impl method::ComplexCast for PyFrame { }
impl method::Rounding for PyFrame { }
impl method::Index for PyFrame { }
impl method::NegateValue for PyFrame { }
impl method::AbsValue for PyFrame { }
impl method::PositiveValue for PyFrame { }
impl method::InvertValue for PyFrame { }
impl method::Add for PyFrame { }
impl method::BitwiseAnd for PyFrame { }
impl method::DivMod for PyFrame { }
impl method::FloorDivision for PyFrame { }
impl method::LeftShift for PyFrame { }
impl method::Modulus for PyFrame { }
impl method::Multiply for PyFrame { }
impl method::MatrixMultiply for PyFrame { }
impl method::BitwiseOr for PyFrame { }
impl method::Pow for PyFrame { }
impl method::RightShift for PyFrame { }
impl method::Subtract for PyFrame { }
impl method::TrueDivision for PyFrame { }
impl method::XOr for PyFrame { }
impl method::ReflectedAdd for PyFrame { }
impl method::ReflectedBitwiseAnd for PyFrame { }
impl method::ReflectedDivMod for PyFrame { }
impl method::ReflectedFloorDivision for PyFrame { }
impl method::ReflectedLeftShift for PyFrame { }
impl method::ReflectedModulus for PyFrame { }
impl method::ReflectedMultiply for PyFrame { }
impl method::ReflectedMatrixMultiply for PyFrame { }
impl method::ReflectedBitwiseOr for PyFrame { }
impl method::ReflectedPow for PyFrame { }
impl method::ReflectedRightShift for PyFrame { }
impl method::ReflectedSubtract for PyFrame { }
impl method::ReflectedTrueDivision for PyFrame { }
impl method::ReflectedXOr for PyFrame { }
impl method::InPlaceAdd for PyFrame { }
impl method::InPlaceBitwiseAnd for PyFrame { }
impl method::InPlaceDivMod for PyFrame { }
impl method::InPlaceFloorDivision for PyFrame { }
impl method::InPlaceLeftShift for PyFrame { }
impl method::InPlaceModulus for PyFrame { }
impl method::InPlaceMultiply for PyFrame { }
impl method::InPlaceMatrixMultiply for PyFrame { }
impl method::InPlaceBitwiseOr for PyFrame { }
impl method::InPlacePow for PyFrame { }
impl method::InPlaceRightShift for PyFrame { }
impl method::InPlaceSubtract for PyFrame { }
impl method::InPlaceTrueDivision for PyFrame { }
impl method::InPlaceXOr for PyFrame { }
impl method::Contains for PyFrame { }
impl method::Iter for PyFrame { }
impl method::Call for PyFrame { }
impl method::Length for PyFrame { }
impl method::LengthHint for PyFrame { }
impl method::Next for PyFrame { }
impl method::Reversed for PyFrame { }
impl method::GetItem for PyFrame { }
impl method::SetItem for PyFrame { }
impl method::DeleteItem for PyFrame { }
impl method::Count for PyFrame { }
impl method::Append for PyFrame { }
impl method::Extend for PyFrame { }
impl method::Pop for PyFrame { }
impl method::Remove for PyFrame { }
impl method::IsDisjoint for PyFrame { }
impl method::AddItem for PyFrame { }
impl method::Discard for PyFrame { }
impl method::Clear for PyFrame { }
impl method::Get for PyFrame { }
impl method::Keys for PyFrame { }
impl method::Values for PyFrame { }
impl method::Items for PyFrame { }
impl method::PopItem for PyFrame { }
impl method::Update for PyFrame { }
impl method::SetDefault for PyFrame { }
impl method::Await for PyFrame { }
impl method::Send for PyFrame { }
impl method::Throw for PyFrame { }
impl method::Close for PyFrame { }
impl method::Exit for PyFrame { }
impl method::Enter for PyFrame { }
impl method::DescriptorGet for PyFrame { }
impl method::DescriptorSet for PyFrame { }
impl method::DescriptorSetName for PyFrame { }