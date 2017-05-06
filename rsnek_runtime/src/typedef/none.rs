use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use runtime::Runtime;
use traits::{BooleanProvider, StringProvider};
use result::{RuntimeResult, NativeResult};
use object::selfref::{self, SelfRef};
use object::{RtValue, PyAPI, method, typing};

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub const NONE: &'static native::None = &native::None();
pub const NONE_STR: &'static str = "None";


pub struct PyNoneType {
    singleton_none: ObjectRef,
}


impl typing::BuiltinType for PyNoneType {
    type T = PyNone;
    type V = &'static native::None;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        return self.singleton_none.clone();
    }

    fn init_type() -> Self {
        PyNoneType { singleton_none: PyNoneType::inject_selfref(PyNoneType::alloc(NONE)) }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::None(value));

        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::None(ref none) => {
                none.rc.set(&objref.clone());
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


pub struct NoneValue(&'static native::None, String);
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
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.str(self.value.1.clone()))
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(self.value.1.clone())
    }
}


impl method::Equal for PyNone {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();
        match self.native_eq(boxed) {
            Ok(truth) => Ok(rt.bool(truth)),
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::None(_) => Ok(true),
            _ => Ok(false),
        }
    }
}


impl method::BooleanCast for PyNone {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.bool(false))
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
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
        println!("stub");
    }
}