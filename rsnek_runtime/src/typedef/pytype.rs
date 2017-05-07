use std::fmt;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;

use ::resource::strings;
use runtime::Runtime;
use api::{self, RtValue, method, typing};
use api::selfref::{self, SelfRef};

use typedef::native;
use ::api::RtObject;
use typedef::builtin::Builtin;


pub struct PyMeta {
    pub pytype: RtObject,
}


impl typing::BuiltinType for PyMeta {
    type T = PyType;
    type V = native::Type;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyMeta::inject_selfref(PyMeta::alloc(value))
    }

    fn init_type() -> Self {
        PyMeta {
            pytype: PyMeta::inject_selfref(PyMeta::alloc(native::Type {
                                                             name: "type".to_string(),
                                                             module: strings::BUILTINS_MODULE.to_string(),
                                                             bases: Vec::new(),
                                                             subclasses: RefCell::new(Vec::new()),
                                                         })),
        }
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Type(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Type(ref pytype) => {
                pytype.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyType {
            value: TypeValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub type PyType = RtValue<TypeValue>;
pub struct TypeValue(pub native::Type);


impl fmt::Display for PyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}

impl fmt::Debug for PyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl api::PyAPI for PyType {}


method_not_implemented!(PyType,
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