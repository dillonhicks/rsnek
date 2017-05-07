use std;
use std::ops::{Deref, Neg};
use std::borrow::Borrow;
use num::{Signed, Zero, FromPrimitive, ToPrimitive};

use api::{self, RtValue, typing, method};
use api::method::{BooleanCast, IntegerCast, StringRepresentation};
use api::selfref::{self, SelfRef};

use ::runtime::Runtime;
use ::traits::{BooleanProvider, StringProvider, IntegerProvider, FloatProvider};
use ::result::{ObjectResult, RtResult};
use ::typedef::builtin::Builtin;
use ::api::RtObject;
use ::typedef::number;
use ::typedef::native::{self, Number, HashId};


pub const TRUE_STR: &'static str = "True";
pub const FALSE_STR: &'static str = "False";
pub const TRUE_BYTES: &'static [u8] = &[1];
pub const FALSE_BYTES: &'static [u8] = &[0];


#[derive(Clone)]
pub struct PyBooleanType {
    singleton_true: RtObject,
    singleton_false: RtObject,
}


impl typing::BuiltinType for PyBooleanType {
    type T = PyBoolean;
    type V = native::Boolean;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        if value {
            self.singleton_true.clone()
        } else {
            self.singleton_false.clone()
        }
    }

    fn init_type() -> Self {
        PyBooleanType {
            singleton_true: PyBooleanType::inject_selfref(PyBooleanType::alloc(true)),
            singleton_false: PyBooleanType::inject_selfref(PyBooleanType::alloc(false)),
        }
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Bool(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Bool(ref boolean) => {
                boolean.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        let int = if value {
            native::Integer::from_usize(1).unwrap()
        } else {
            native::Integer::zero()
        };
        PyBoolean {
            value: BoolValue(int),
            rc: selfref::RefCount::default(),
        }
    }
}


#[derive(Clone)]
pub struct BoolValue(pub native::Integer);
pub type PyBoolean = RtValue<BoolValue>;


impl std::fmt::Debug for PyBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.value.0.is_zero() {
            write!(f, "{}", FALSE_STR)
        } else {
            write!(f, "{}", TRUE_STR)
        }
    }
}


impl api::PyAPI for PyBoolean {}


impl method::Hashed for PyBoolean {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        let hash = self.native_hash()?;
        Ok(rt.int(hash))
    }

    fn native_hash(&self) -> RtResult<HashId> {
        Ok(number::hash_int(&self.value.0))
    }
}

impl method::StringCast for PyBoolean {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        self.op_repr(rt)
    }

    fn native_str(&self) -> RtResult<native::String> {
        self.native_repr()
    }
}

impl method::BytesCast for PyBoolean {

    fn native_bytes(&self) -> RtResult<native::Bytes> {
        let result = if self.value.0.is_zero() {
            FALSE_BYTES.to_vec()
        } else {
            TRUE_BYTES.to_vec()
        };
        Ok(result)
    }
}

impl method::StringRepresentation for PyBoolean {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        match self.native_repr() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_repr(&self) -> RtResult<native::String> {
        let value = if self.value.0.is_zero() {
            FALSE_STR
        } else {
            TRUE_STR
        };
        Ok(value.to_string())
    }
}

/// `x == y`
impl method::Equal for PyBoolean {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match self.native_eq(rhs.as_ref()) {
            Ok(value) => {
                if value {
                    Ok(rt.bool(true))
                } else {
                    Ok(rt.bool(false))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool().unwrap() == value),
            Err(err) => Err(err),
        }
    }
}

impl method::NotEqual for PyBoolean {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match self.native_ne(rhs.as_ref()) {
            Ok(value) => {
                if value {
                    Ok(rt.bool(true))
                } else {
                    Ok(rt.bool(false))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_ne(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool()? != value),
            Err(err) => Err(err),
        }
    }
}



impl method::BooleanCast for PyBoolean {
    #[allow(unused_variables)]
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        self.rc.upgrade()
    }

    fn native_bool(&self) -> RtResult<native::Boolean> {
        Ok(!self.value.0.is_zero())
    }
}
impl method::IntegerCast for PyBoolean {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.value.0.clone()))
    }

    fn native_int(&self) -> RtResult<native::Integer> {
        Ok(self.value.0.clone())
    }
}

impl method::FloatCast for PyBoolean {
        fn op_float(&self, rt: &Runtime) -> ObjectResult {
            let value = if self.value.0.is_zero() {0.0} else {1.0};
            Ok(rt.float(value))
        }

        fn native_float(&self) -> RtResult<native::Float> {
            return Ok(self.value.0.to_f64().unwrap());
        }
}


/// `round(True) => 1` `round(False) => 0`
impl method::Rounding for PyBoolean {
    fn op_round(&self, rt: &Runtime) -> ObjectResult {
        match self.native_round() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_round(&self) -> RtResult<Number> {
        Ok(Number::Int(self.value.0.clone()))
    }
}

/// `__index___`
impl method::Index for PyBoolean {
    fn op_index(&self, rt: &Runtime) -> ObjectResult {
        match self.native_index() {
            Ok(int) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_index(&self) -> RtResult<native::Integer> {
        self.native_int()
    }
}

/// `-self`
impl method::NegateValue for PyBoolean {
    fn op_neg(&self, rt: &Runtime) -> ObjectResult {
        match self.native_neg() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_neg(&self) -> RtResult<Number> {
        Ok(Number::Int(self.value
                           .0
                           .clone()
                           .neg()))
    }
}

/// `__abs__`
impl method::AbsValue for PyBoolean {
    fn op_abs(&self, rt: &Runtime) -> ObjectResult {
        match self.native_abs() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_abs(&self) -> RtResult<Number> {
        Ok(Number::Int(self.value.0.abs()))
    }
}

/// `+self`
impl method::PositiveValue for PyBoolean {
    fn op_pos(&self, rt: &Runtime) -> ObjectResult {
        match self.native_pos() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_pos(&self) -> RtResult<Number> {
        Ok(Number::Int(self.value.0.clone()))
    }
}

method_not_implemented!(PyBoolean,
    New   Init   Delete   GetAttr   
    GetAttribute   SetAttr   DelAttr   StringFormat   
    ComplexCast   LessThan   LessOrEqual   GreaterOrEqual   
    GreaterThan   InvertValue   Add   BitwiseAnd   
    DivMod   FloorDivision   LeftShift   Modulus   
    Multiply   MatrixMultiply   BitwiseOr   Pow   
    RightShift   Subtract   TrueDivision   XOr   
    ReflectedAdd   ReflectedBitwiseAnd   ReflectedDivMod   ReflectedFloorDivision   
    ReflectedLeftShift   ReflectedModulus   ReflectedMultiply   ReflectedMatrixMultiply   
    ReflectedBitwiseOr   ReflectedPow   ReflectedRightShift   ReflectedSubtract   
    ReflectedTrueDivision   ReflectedXOr   InPlaceAdd   InPlaceBitwiseAnd   
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceModulus   
    InPlaceMultiply   InPlaceMatrixMultiply   InPlaceBitwiseOr   InPlacePow   
    InPlaceRightShift   InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   
    Contains   Iter   Call   Length   
    LengthHint   Next   Reversed   GetItem   
    SetItem   DeleteItem   Count   Append   
    Extend   Pop   Remove   IsDisjoint   
    AddItem   Discard   Clear   Get   
    Keys   Values   Items   PopItem   
    Update   SetDefault   Await   Send   
    Throw   Close   Exit   Enter   
    DescriptorGet   DescriptorSet   DescriptorSetName
);


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod tests {
    use api::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();
        let t = rt.bool(true);

        let result = f.op_is(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(true), "BooleanObject is(op_is)");

        let result = f.op_is(&rt, &t).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    #[test]
    fn is_not() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();
        let t = rt.bool(true);


        let result = f.op_is_not(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(false), "BooleanObject is(op_is)");

        let result = f.op_is_not(&rt, &t).unwrap();
        assert_eq!(result, rt.bool(true));
    }


    #[test]
    fn __eq__() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();

        let result = f.op_eq(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(true))
    }

    #[test]
    fn __bool__() {
        let rt = setup_test();

        let (t, f) = (rt.bool(true), rt.bool(false));

        let result = t.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = f.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    #[test]
    fn __int__() {
        let rt = setup_test();

        let one = rt.int(1);

        let t = rt.bool(true);

        let result = t.op_int(&rt).unwrap();
        assert_eq!(result, one);
    }


    #[test]
    fn __float__() {
        let rt = setup_test();

        let one = rt.float(1.0);
        let t = rt.bool(true);

        let result = t.op_float(&rt).unwrap();
        assert_eq!(result, one);
    }

}
