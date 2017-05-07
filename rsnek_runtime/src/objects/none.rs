use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use runtime::Runtime;
use ::runtime::traits::{BooleanProvider, StringProvider};
use ::api::result::{ObjectResult, RtResult};
use api::selfref::{self, SelfRef};
use api::{RtValue, PyAPI, method, typing};

use ::system::primitives as rs;
use ::api::RtObject;
use ::modules::builtins::Type;


pub const NONE: &'static rs::None = &rs::None();
pub const NONE_STR: &'static str = "None";


pub struct PyNoneType {
    singleton_none: RtObject,
}


impl typing::BuiltinType for PyNoneType {
    type T = PyNone;
    type V = &'static rs::None;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        return self.singleton_none.clone();
    }

    fn init_type() -> Self {
        PyNoneType { singleton_none: PyNoneType::inject_selfref(PyNoneType::alloc(NONE)) }
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::None(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::None(ref none) => {
                none.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyNone {
            value: NoneValue(value, NONE_STR.to_string()),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct NoneValue(&'static rs::None, String);
pub type PyNone = RtValue<NoneValue>;



impl fmt::Display for PyNone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PyNone")
    }
}

impl fmt::Debug for PyNone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PyNone")
    }
}


impl PyAPI for PyNone {}


impl method::StringCast for PyNone {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.str(self.value.1.clone()))
    }

    fn native_str(&self) -> RtResult<rs::String> {
        Ok(self.value.1.clone())
    }
}


impl method::Equal for PyNone {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_eq(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_eq(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        match rhs {
            &Type::None(_) => Ok(true),
            _ => Ok(false),
        }
    }
}


impl method::BooleanCast for PyNone {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.bool(false))
    }

    fn native_bool(&self) -> RtResult<rs::Boolean> {
        Ok(false)
    }
}


method_not_implemented!(PyNone,
    AbsValue   Add   AddItem   Append   Await   BitwiseAnd   BitwiseOr
    BytesCast   Call   Clear   Close   ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Exit   Extend   FloatCast    FloorDivision   Get   GetAttr   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan   Hashed
    Id   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply   InPlacePow   InPlaceRightShift
    InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   Index
    Init   IntegerCast   InvertValue   Is
    IsDisjoint   IsNot   Items   Iter    Keys   LeftShift   Length   LengthHint
    LessOrEqual   LessThan   MatrixMultiply   Modulus
    Multiply   NegateValue   New   Next    NotEqual   Pop   PopItem   PositiveValue
    Pow   ReflectedAdd   ReflectedBitwiseAnd   ReflectedBitwiseOr
    ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   ReflectedMatrixMultiply
    ReflectedModulus   ReflectedMultiply   ReflectedPow   ReflectedRightShift
    ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding   Send    SetAttr   SetDefault   SetItem
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