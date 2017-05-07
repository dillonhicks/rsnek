use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use runtime::Runtime;
use api::{RtValue, PyAPI, method, typing};
use api::selfref::{self, SelfRef};

use ::system::primitives as rs;
use ::modules::builtins::Type;
use ::api::RtObject;


#[derive(Clone)]
pub struct PyComplexType {}


impl typing::BuiltinType for PyComplexType {
    type T = PyComplex;
    type V = rs::Complex;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyComplexType::inject_selfref(PyComplexType::alloc(value))
    }

    fn init_type() -> Self {
        PyComplexType {}
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::Complex(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::Complex(ref complex) => {
                complex.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyComplex {
            value: ComplexValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}



#[derive(Clone)]
pub struct ComplexValue(rs::Complex);
pub type PyComplex = RtValue<ComplexValue>;


impl fmt::Display for PyComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}


impl fmt::Debug for PyComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl PyAPI for PyComplex {}


method_not_implemented!(PyComplex,
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