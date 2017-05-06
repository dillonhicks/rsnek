use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::str::FromStr;

use num::ToPrimitive;

use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use traits::{IntegerProvider, BooleanProvider, StringProvider, DefaultStringProvider, IteratorProvider};
use error::Error;

use object::{self, RtValue};
use object::selfref::{self, SelfRef};
use object::typing::{self, BuiltinType};
use object::method;

use typedef::native;
use ::object::RtObject;
use typedef::builtin::Builtin;
use typedef::collection::sequence;
use resource::strings;

pub struct PyStringType {
    pub empty: RtObject,
}


impl typing::BuiltinType for PyStringType {
    type T = PyString;
    type V = native::String;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyStringType::inject_selfref(PyStringType::alloc(value))
    }



    fn init_type() -> Self {
        PyStringType { empty: PyStringType::inject_selfref(PyStringType::alloc("".to_string())) }
    }


    fn inject_selfref(value: Self::T) -> RtObject {
        let objref = RtObject::new(Builtin::Str(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Str(ref string) => {
                string.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }


    fn alloc(value: Self::V) -> Self::T {
        PyString {
            value: StringValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct StringValue(pub native::String);
pub type PyString = RtValue<StringValue>;

impl fmt::Debug for PyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String {{ {:?} }}", self.value.0)
    }
}


impl object::PyAPI for PyString {}

impl method::Hashed for PyString {
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


impl method::StringCast for PyString {

    #[allow(unused_variables)]
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.rc.upgrade()
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(self.value.0.clone())
    }

}


impl method::Equal for PyString {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(boxed) {
            Ok(value) => Ok(rt.bool(value)),
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::Str(ref string) => Ok(self.value.0.eq(&string.value.0)),
            _ => Ok(false),
        }
    }
}


impl method::BooleanCast for PyString {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_bool() {
            Ok(bool) => Ok(rt.bool(bool)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.0.is_empty())
    }
}


impl method::IntegerCast for PyString {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => Ok(rt.int(int)),
            Err(err) => Err(err)
        }
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        match native::Integer::from_str(&self.value.0) {
            Ok(int) => Ok(int),
            Err(_) => Err(Error::value(
                &format!("Invalid literal '{}' for int", self.value.0)))
        }
    }

}


impl method::Add for PyString {

    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();
        match builtin.deref() {
            &Builtin::Str(ref other) => Ok(rt.str([&self.value.0[..], &other.value.0[..]].concat())),
            other => Err(Error::typerr(
                &strings_error_bad_operand!("+", "str", other.debug_name()))),
        }
    }

}

impl method::Multiply for PyString {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref int) => {
                match int.value.0.to_usize() {
                    Some(int) if int <= 0   => Ok(rt.default_str()),
                    Some(int) if int == 1   => self.rc.upgrade(),
                    Some(int)               => {
                        let value: String = (0..int)
                            .map(|_| self.value.0.clone())
                            .collect::<Vec<_>>()
                            .concat();
                        Ok(rt.str(value))
                    },
                    None                    => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("*", "str", other.debug_name())))
        }
    }

}


impl method::Contains for PyString {
    fn op_contains(&self, rt: &Runtime, item: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        let truth = self.native_contains(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Builtin) -> NativeResult<native::Boolean> {
        match item {
            &Builtin::Str(ref string) => {
                Ok(self.value.0.contains(&string.value.0))
            },
            other => Err(Error::typerr(&format!(
                "in <string>' requires string as left operand, not {}",
                other.debug_name())))
        }
    }
}

impl method::Iter for PyString {
    fn op_iter(&self, rt: &Runtime) -> RuntimeResult {
        let iter = self.native_iter()?;
        Ok(rt.iter(iter))
    }

    fn native_iter(&self) -> NativeResult<native::Iterator> {
        match self.rc.upgrade() {
            Ok(selfref) => Ok(native::Iterator::new(&selfref)?),
            Err(err) => Err(err)
        }
    }
}

impl method::Length for PyString {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(self.value.0.len() as i64))
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.len()))
    }
}

impl method::GetItem for PyString {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, item: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        self.native_getitem(boxed)
    }

    fn native_getitem(&self, index: &Builtin) -> RuntimeResult {
        let substr = match index {
            &Builtin::Int(ref int) => {
                let byte = sequence::get_index(&self.value.0.as_bytes(), &int.value.0)?;
                // TODO: {T3093} Determine the best policy for strings as a sequence since any kind
                // of encoding ruins the uniform treatment of bytes as a singular index of a
                // string.
                String::from_utf8_lossy(&[byte][..]).to_string()
            }
            _ => return Err(Error::typerr("string indices must be integers")),
        };

        Ok(PyStringType::inject_selfref(PyStringType::alloc(substr)))
    }
}


method_not_implemented!(PyString,
    AbsValue   AddItem   Append   Await   BitwiseAnd   BitwiseOr   
    BytesCast   Call   Clear   Close   ComplexCast   Count   
    DelAttr   Delete   DeleteItem   DescriptorGet   DescriptorSet   DescriptorSetName   
    Discard   DivMod   Enter   Exit   Extend   FloatCast   
    FloorDivision   Get   GetAttr   GetAttribute   GreaterOrEqual   GreaterThan   
    Id   Index   Init   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr   
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply InPlacePow   InPlaceRightShift   InPlaceSubtract
    InPlaceTrueDivision   InPlaceXOr   InvertValue Is   IsDisjoint   IsNot   Items   Keys
    LeftShift LengthHint   LessOrEqual   LessThan   MatrixMultiply   Modulus   NegateValue
    New   Next   NotEqual   Pop   PopItem   PositiveValue  Pow   ReflectedAdd   ReflectedBitwiseAnd
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision ReflectedLeftShift
    ReflectedMatrixMultiply   ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift Rounding   Send   SetAttr   SetDefault   SetItem   StringFormat
    StringRepresentation   Subtract   Throw   TrueDivision   Update   Values XOr
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