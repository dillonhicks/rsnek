use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use traits::{IntegerProvider, BooleanProvider};

use object::{self, RtValue};
use object::selfref::{self, SelfRef};
use object::typing;
use object::method;

use typedef::native;
use ::object::RtObject;
use typedef::builtin::Builtin;


pub struct PyBytesType {
    pub empty: RtObject,
}


impl typing::BuiltinType for PyBytesType {
    type T = PyBytes;
    type V = native::Bytes;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyBytesType::inject_selfref(PyBytesType::alloc(value))
    }

    fn init_type() -> Self {
        PyBytesType { empty: PyBytesType::inject_selfref(PyBytesType::alloc(native::Bytes::new())) }
    }


    fn inject_selfref(value: Self::T) -> RtObject {
        let objref = RtObject::new(Builtin::Bytes(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Bytes(ref string) => {
                string.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }


    fn alloc(value: Self::V) -> Self::T {
        PyBytes {
            value: StringValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct StringValue(pub native::Bytes);
pub type PyBytes = RtValue<StringValue>;


impl fmt::Debug for PyBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bytes {{ {:?} }}", self.value.0)
    }
}


impl object::PyAPI for PyBytes {}


impl method::Hashed for PyBytes {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => Ok(rt.int(native::Integer::from(value))),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.value.0.hash(&mut s);
        Ok(s.finish())
    }
}

impl method::Equal for PyBytes {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(boxed) {
            Ok(value) => Ok(rt.bool(value)),
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::Bytes(ref bytes) => Ok(self.value.0 == bytes.value.0),
            _ => Ok(false),
        }
    }
}


method_not_implemented!(PyBytes,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close
    ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Exit   Extend   FloatCast
    FloorDivision   Get   GetAttr   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan
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