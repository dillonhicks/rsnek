use std::fmt;
use std::borrow::Borrow;
use std::ops::Deref;

use num::Zero;
use num::ToPrimitive;

use runtime::Runtime;
use traits::{BooleanProvider, StringProvider, IntegerProvider, FloatProvider};
use ::api::error::Error;
use ::api::result::{RtResult, ObjectResult};
use api::{self, RtValue, method, typing};
use api::selfref::{self, SelfRef};

use objects::native;
use ::api::RtObject;
use objects::builtin::Builtin;
use objects::number::{self, FloatAdapter, IntAdapter};


#[derive(Clone)]
pub struct PyFloatType {}


impl typing::BuiltinType for PyFloatType {
    type T = PyFloat;
    type V = native::Float;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: native::Float) -> RtObject {
        // TODO: {T99} Investigate object interning, see the methodology in integer.rs.
        // Can that be generalized?
        PyFloatType::inject_selfref(PyFloatType::alloc(value))
    }

    fn init_type() -> Self {
        PyFloatType {}
    }

    fn inject_selfref(value: PyFloat) -> RtObject {
        let object = RtObject::new(Builtin::Float(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Float(ref int) => {
                int.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyFloat {
            value: FloatValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct FloatValue(pub native::Float);
pub type PyFloat = RtValue<FloatValue>;


impl fmt::Display for PyFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}

impl fmt::Debug for PyFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


impl api::PyAPI for PyFloat {}


impl method::Hashed for PyFloat {
    // TODO: {T87} python has its own algo for hashing floats ensure to look at that for compat.
}

impl method::StringCast for PyFloat {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        match self.native_str() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_str(&self) -> RtResult<native::String> {
        Ok(number::format_float(&self.value.0))
    }
}


impl method::StringRepresentation for PyFloat {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        match self.native_repr() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_repr(&self) -> RtResult<native::String> {
        Ok(number::format_float(&self.value.0))
    }
}

impl method::Equal for PyFloat {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match self.native_eq(rhs.as_ref()) {
            Ok(value) => Ok(rt.bool(value)),
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, other: &Builtin) -> RtResult<native::Boolean> {
        match *other {
            Builtin::Float(ref float) => Ok(self.value.0 == float.value.0),
            Builtin::Int(ref int) => Ok(FloatAdapter(&self.value.0) == IntAdapter(&int.value.0)),
            _ => Ok(false),
        }
    }
}


impl method::BooleanCast for PyFloat {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        if self.native_bool()? {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_bool(&self) -> RtResult<native::Boolean> {
        return Ok(!self.value.0.is_zero());
    }
}

impl method::IntegerCast for PyFloat {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        match self.native_int() {
            Ok(int) => Ok(rt.int(int)),
            _ => unreachable!()
        }
    }

    fn native_int(&self) -> RtResult<native::Integer> {
        return Ok(native::Integer::from(self.value.0 as i64));
    }
}

impl method::FloatCast for PyFloat {
    #[allow(unused_variables)]
    fn op_float(&self, rt: &Runtime) -> ObjectResult {
        self.rc.upgrade()
    }

    fn native_float(&self) -> RtResult<native::Float> {
        return Ok(self.value.0);
    }
}


impl method::Add for PyFloat {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match rhs.as_ref(){
            &Builtin::Float(ref rhs) => {
                // TODO: {T103} Use checked arithmetic where appropriate... this is not the only
                // example. But the float (and some int) methods are likely to be the highest
                // frequency.
                Ok(rt.float(self.value.0 + rhs.value.0))
            }
            &Builtin::Int(ref rhs) => {
                match rhs.value.0.to_f64() {
                    Some(float) => Ok(rt.float(self.value.0 + float)),
                    None => Err(Error::overflow(&format!("{:?} + {} overflows", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(&format!("Cannot add {} to float", other.debug_name()))),
        }
    }

}

method_not_implemented!(PyFloat,
    AbsValue   AddItem   Append   Await   
    BitwiseAnd   BitwiseOr   BytesCast   Call   
    Clear   Close   ComplexCast   Contains   
    Count   DelAttr   Delete   DeleteItem   
    DescriptorGet   DescriptorSet   DescriptorSetName   Discard   
    DivMod   Enter   Exit   Extend   
    FloorDivision   Get   GetAttr   GetAttribute   
    GetItem   GreaterOrEqual   GreaterThan   Id   
    Index   Init   InPlaceAdd   InPlaceBitwiseAnd   
    InPlaceBitwiseOr   InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   
    InPlaceMatrixMultiply   InPlaceModulus   InPlaceMultiply   InPlacePow   
    InPlaceRightShift   InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   
    InvertValue   Is   IsDisjoint   IsNot   
    Items   Iter   Keys   LeftShift   
    Length   LengthHint   LessOrEqual   LessThan   
    MatrixMultiply   Modulus   Multiply   NegateValue   
    New   Next   NotEqual   Pop   
    PopItem   PositiveValue   Pow   ReflectedAdd   
    ReflectedBitwiseAnd   ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   
    ReflectedLeftShift   ReflectedMatrixMultiply   ReflectedModulus   ReflectedMultiply   
    ReflectedPow   ReflectedRightShift   ReflectedSubtract   ReflectedTrueDivision   
    ReflectedXOr   Remove   Reversed   RightShift   
    Rounding   Send   SetAttr   SetDefault   
    SetItem   StringFormat   Subtract   Throw   
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
