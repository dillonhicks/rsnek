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
            &Builtin::Code(ref code) => {
                code.rc.set(&objref.clone());
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


impl object::PyAPI for PyCode {}

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
        println!("stub");
    }
}
