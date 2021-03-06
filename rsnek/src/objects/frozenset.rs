//! PyFrozenSet - immutable collection of unique objects
//!
//! ```ignore
//! fozenset({2.0, "sets", 4, "reps"})
//! ```
//!
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
pub struct PyFrozenSetType {}


impl typing::BuiltinType for PyFrozenSetType {
    type T = PyFrozenSet;
    type V = rs::Set;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyFrozenSetType::inject_selfref(PyFrozenSetType::alloc(value))
    }

    fn init_type() -> Self {
        PyFrozenSetType {}
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::FrozenSet(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::FrozenSet(ref complex) => {
                complex.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyFrozenSet {
            value: FrozenSetValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}



#[derive(Clone)]
pub struct FrozenSetValue(rs::Set);
pub type PyFrozenSet = RtValue<FrozenSetValue>;


impl fmt::Display for PyFrozenSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl fmt::Debug for PyFrozenSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl PyAPI for PyFrozenSet {}


method_not_implemented!(PyFrozenSet,
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