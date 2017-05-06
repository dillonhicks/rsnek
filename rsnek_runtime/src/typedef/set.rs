use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use runtime::Runtime;
use object::{RtValue, PyAPI, method, typing};
use object::selfref::{self, SelfRef};

use typedef::native;
use typedef::builtin::Builtin;
use ::object::RtObject;


#[derive(Clone)]
pub struct PySetType {}


impl typing::BuiltinType for PySetType {
    type T = PySet;
    type V = native::Set;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PySetType::inject_selfref(PySetType::alloc(value))
    }

    fn init_type() -> Self {
        PySetType {}
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let objref = RtObject::new(Builtin::Set(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Set(ref complex) => {
                complex.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PySet {
            value: SetValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}



#[derive(Clone)]
pub struct SetValue(native::Set);
pub type PySet = RtValue<SetValue>;


impl fmt::Display for PySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl fmt::Debug for PySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl PyAPI for PySet {}


method_not_implemented!(PySet,
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