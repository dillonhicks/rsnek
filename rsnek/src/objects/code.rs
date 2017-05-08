//! PyCode - Object wrapper around a bytecode object
//!
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use runtime::{Runtime};

use api::{self, RtValue};
use api::selfref::{self, SelfRef};
use api::typing;
use api::method;

use ::system::primitives as rs;
use ::api::RtObject;
use ::modules::builtins::Type;


pub struct PyCodeType {}

impl typing::BuiltinType for PyCodeType {
    type T = PyCode;
    type V = rs::Code;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyCodeType::inject_selfref(PyCodeType::alloc(value))
    }

    fn init_type() -> Self {
        PyCodeType {}
    }


    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::Code(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::Code(ref code) => {
                code.rc.set(&object.clone());
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


pub struct CodeValue(pub rs::Code);
pub type PyCode = RtValue<CodeValue>;


impl api::PyAPI for PyCode {}

impl fmt::Debug for PyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<code {:?}>", self.value.0.co_name)
    }
}

method_not_implemented!(PyCode,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close
    ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Equal   Exit   Extend   FloatCast
    FloorDivision   Get   GetAttr   GetAttribute
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
