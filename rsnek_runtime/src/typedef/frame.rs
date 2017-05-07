use std::ops::Deref;
use std::borrow::Borrow;
use std::fmt;

use api::{PyAPI, RtValue, typing, method};
use api::selfref::{self, SelfRef};
use error::Error;
use runtime::Runtime;
//use traits::{BooleanProvider, IntegerProvider, NoneProvider};

use result::{ObjectResult};
use typedef::builtin::Builtin;
use typedef::native;
use ::api::RtObject;

//pub const FRAME_MAX_BLOCKS: usize = 20;


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

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Frame(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Frame(ref boolean) => {
                boolean.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyFrameType::inject_selfref(PyFrameType::alloc(value))
    }

}


#[derive(Debug, Clone)]
pub struct FrameValue(pub native::Frame);
pub type PyFrame = RtValue<FrameValue>;


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


impl PyAPI for PyFrame { }


impl method::GetAttr for PyFrame {
    #[allow(unused_variables)]
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        let attr: &str = match name.as_ref() {
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


method_not_implemented!(PyFrame,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close
    ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Equal   Exit   Extend   FloatCast
    FloorDivision   Get   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan   Hashed
    Id   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply   InPlacePow   InPlaceRightShift
    InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   Index
    Init   IntegerCast   InvertValue   Is
    IsDisjoint   IsNot   Items   Iter
    Keys   LeftShift   Length   LengthHint
    LessOrEqual   LessThan   MatrixMultiply   Modulus
    Multiply   NegateValue   New   Next
    NotEqual   Pop   PopItem   PositiveValue
    Pow   ReflectedAdd   ReflectedBitwiseAnd   ReflectedBitwiseOr
    ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   ReflectedMatrixMultiply
    ReflectedModulus   ReflectedMultiply   ReflectedPow   ReflectedRightShift
    ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding   Send
    SetAttr   SetDefault   SetItem   StringCast
    StringFormat   StringRepresentation   Subtract   Throw
    TrueDivision   Update   Values   XOr
);


#[cfg(test)]
mod tests {
    use ::runtime::Runtime;

    fn setup() -> (Runtime, ) {
        (Runtime::new(), )
    }

    #[test]
    fn stub() {
        info!("stub");
    }
}