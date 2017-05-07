use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::str::FromStr;

use num::ToPrimitive;

use ::modules::precondition::{check_args, check_kwargs};
use ::error::Error;
use ::api::method;
use ::api::method::*;
use ::api::RtObject;
use ::api::selfref::{self, SelfRef};
use ::api::typing::{self, BuiltinType};
use ::api::{self, RtValue};
use ::resource::strings;
use ::result::{RtResult, ObjectResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider, BooleanProvider, StringProvider, DefaultStringProvider,
               FunctionProvider, IteratorProvider};
use ::objects::builtin::Builtin;
use ::objects::collection::sequence;
use ::objects::native::{self, SignatureBuilder};


const TYPE_NAME: &'static str = "str";


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
        let object = RtObject::new(Builtin::Str(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Str(ref string) => {
                string.rc.set(&object.clone());
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


impl PyString {
    /* Missing
        [('__ceil__', <function int.__ceil__>),
        ('__class__', int),
        ('__dir__', <function int.__dir__>),
        ('__floor__', <function int.__floor__>),
        ('__format__', <function int.__format__>),
        ('__getnewargs__', <function int.__getnewargs__>),
        ('__init_subclass__', <function int.__init_subclass__>),
        ('__new__', <function int.__new__>),
        ('__reduce__', <function int.__reduce__>),
        ('__reduce_ex__', <function int.__reduce_ex__>),
        ('__round__', <function int.__round__>),
        ('__sizeof__', <function int.__sizeof__>),
        ('__subclasshook__', <function int.__subclasshook__>),
        ('__trunc__', <function int.__trunc__>),
        ('bit_length', <function int.bit_length>),
        ('conjugate', <function int.conjugate>),
        ('denominator', 1),
        ('from_bytes', <function int.from_bytes>),
        ('imag', 0),
        ('numerator', 1),
        ('real', 1),
        ('to_bytes', <function int.to_bytes>)]
    */
    pub fn get_attribute(&self, rt: &Runtime, name: &str) -> ObjectResult {
        match name {
            "__doc__"           => self.try_get_name(rt, name),
            "__abs__"           |
            "__bool__"          |
            "__float__"         |
            "__hash__"          |
            "__index__"         |
            "__int__"           |
            "__invert__"        |
            "__neg__"           |
            "__pos__"           |
            "__repr__"          |
            "__str__"           => self.try_get_unary_method(rt, name),
            "__add__"           |
            "__and__"           |
            "__delattr__"       |
            "__divmod__"        |
            "__eq__"            |
            "__floordiv__"      |
            "__ge__"            |
            "__getattribute__"  |
            "__gt__"            |
            "__le__"            |
            "__lshift__"        |
            "__lt__"            |
            "__mod__"           |
            "__mul__"           |
            "__ne__"            |
            "__or__"            |
            "__radd__"          |
            "__rand__"          |
            "__rdivmod__"       |
            "__rfloordiv__"     |
            "__rlshift__"       |
            "__rmod__"          |
            "__rmul__"          |
            "__ror__"           |
            "__rrshift__"       |
            "__rshift__"        |
            "__rsub__"          |
            "__rtruediv__"      |
            "__rxor__"          |
            "__sub__"           |
            "__truediv__"       |
            "__xor__"           => self.try_get_binary_method(rt, name),
            "__pow__"           |
            "__rpow__"          |
            "__setattr__"       => self.try_get_ternary_method(rt, name),
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        }
    }

    fn try_get_name(&self, rt: &Runtime, name: &str) -> ObjectResult {
        match name {
            "__doc__" => Ok(rt.str(strings::INT_DOC_STRING)),
            missing => Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        }
    }

    fn try_get_unary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__abs__"       => {PyString::op_abs},
            "__bool__"      => {PyString::op_bool},
            "__float__"     => {PyString::op_float},
            "__hash__"      => {PyString::op_hash},
            "__index__"     => {PyString::op_index},
            "__int__"       => {PyString::op_int},
            "__invert__"    => {PyString::op_invert},
            "__neg__"       => {PyString::op_neg},
            "__pos__"       => {PyString::op_pos},
            "__repr__"      => {PyString::op_repr},
            "__str__"       => {PyString::op_str},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        unary_method_wrapper!(self, TYPE_NAME, name, rt, Builtin::Str, func)
    }

    fn try_get_binary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__add__"          => {PyString::op_add},
            "__and__"          => {PyString::op_and},
            "__delattr__"      => {PyString::op_delattr},
            "__divmod__"       => {PyString::op_divmod},
            "__eq__"           => {PyString::op_eq},
            "__floordiv__"     => {PyString::op_floordiv},
            "__ge__"           => {PyString::op_ge},
            "__getattribute__" => {PyString::op_getattribute},
            "__gt__"           => {PyString::op_gt},
            "__le__"           => {PyString::op_le},
            "__lshift__"       => {PyString::op_lshift},
            "__lt__"           => {PyString::op_lt},
            "__mod__"          => {PyString::op_mod},
            "__mul__"          => {PyString::op_mul},
            "__ne__"           => {PyString::op_ne},
            "__or__"           => {PyString::op_or},
            "__radd__"         => {PyString::op_radd},
            "__rand__"         => {PyString::op_rand},
            "__rdivmod__"      => {PyString::op_rdivmod},
            "__rfloordiv__"    => {PyString::op_rfloordiv},
            "__rlshift__"      => {PyString::op_rlshift},
            "__rmod__"         => {PyString::op_rmod},
            "__rmul__"         => {PyString::op_rmul},
            "__ror__"          => {PyString::op_ror},
            "__rrshift__"      => {PyString::op_rrshift},
            "__rshift__"       => {PyString::op_rshift},
            "__rsub__"         => {PyString::op_rsub},
            "__rtruediv__"     => {PyString::op_rtruediv},
            "__rxor__"         => {PyString::op_rxor},
            "__sub__"          => {PyString::op_sub},
            "__truediv__"      => {PyString::op_truediv},
            "__xor__"          => {PyString::op_xor},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        binary_method_wrapper!(self, TYPE_NAME, name, rt, Builtin::Str, func)
    }

    fn try_get_ternary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__pow__"          => {PyString::op_pow},
            //"__rpow__"         => {PyString::op_rpow},
            "__setattr__"      => {PyString::op_setattr},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        ternary_method_wrapper!(self, TYPE_NAME, name, rt, Builtin::Str, func)
    }

}

impl fmt::Debug for PyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String {{ {:?} }}", self.value.0)
    }
}


impl api::PyAPI for PyString {}

/// `self.rhs`
impl method::GetAttr for PyString {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {

        match name.as_ref() {
            &Builtin::Str(ref pystring) => {
                let string = pystring.value.0.clone();
                self.get_attribute(&rt, &string)
            }
            other => Err(Error::typerr(&format!(
                "getattr <{}>' requires string for attribute names, not {}",
                TYPE_NAME, other.debug_name())))
        }
    }
}

impl method::Hashed for PyString {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        match self.native_hash() {
            Ok(value) => Ok(rt.int(native::Integer::from(value))),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> RtResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.value.0.hash(&mut s);
        Ok(s.finish())
    }
}


impl method::StringCast for PyString {

    #[allow(unused_variables)]
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        self.rc.upgrade()
    }

    fn native_str(&self) -> RtResult<native::String> {
        Ok(self.value.0.clone())
    }

}


impl method::Equal for PyString {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match self.native_eq(rhs.as_ref()) {
            Ok(value) => Ok(rt.bool(value)),
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        match rhs {
            &Builtin::Str(ref string) => Ok(self.value.0.eq(&string.value.0)),
            _ => Ok(false),
        }
    }
}


impl method::BooleanCast for PyString {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        match self.native_bool() {
            Ok(bool) => Ok(rt.bool(bool)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> RtResult<native::Boolean> {
        Ok(!self.value.0.is_empty())
    }
}


impl method::IntegerCast for PyString {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        match self.native_int() {
            Ok(int) => Ok(rt.int(int)),
            Err(err) => Err(err)
        }
    }

    fn native_int(&self) -> RtResult<native::Integer> {
        match native::Integer::from_str(&self.value.0) {
            Ok(int) => Ok(int),
            Err(_) => Err(Error::value(
                &format!("Invalid literal '{}' for int", self.value.0)))
        }
    }

}


impl method::Add for PyString {

    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match rhs.as_ref() {
            &Builtin::Str(ref other) => Ok(rt.str([&self.value.0[..], &other.value.0[..]].concat())),
            other => Err(Error::typerr(
                &strings_error_bad_operand!("+", "str", other.debug_name()))),
        }
    }

}

impl method::Multiply for PyString {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {

        match rhs.as_ref() {
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
                &strings_error_bad_operand!("*", TYPE_NAME, other.debug_name())))
        }
    }

}


impl method::Contains for PyString {
    fn op_contains(&self, rt: &Runtime, item: &RtObject) -> ObjectResult {
        let truth = self.native_contains(item.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Builtin) -> RtResult<native::Boolean> {
        match item {
            &Builtin::Str(ref string) => {
                Ok(self.value.0.contains(&string.value.0))
            },
            other => Err(Error::typerr(&format!(
                "in <{}>' requires string as left operand, not {}",
                TYPE_NAME, other.debug_name())))
        }
    }
}

impl method::Iter for PyString {
    fn op_iter(&self, rt: &Runtime) -> ObjectResult {
        let iter = self.native_iter()?;
        Ok(rt.iter(iter))
    }

    fn native_iter(&self) -> RtResult<native::Iterator> {
        match self.rc.upgrade() {
            Ok(selfref) => Ok(native::Iterator::new(&selfref)?),
            Err(err) => Err(err)
        }
    }
}

impl method::Length for PyString {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.value.0.len()))
    }

    fn native_len(&self) -> RtResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.len()))
    }
}

impl method::GetItem for PyString {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, item: &RtObject) -> ObjectResult {
        self.native_getitem(item.as_ref())
    }

    fn native_getitem(&self, index: &Builtin) -> ObjectResult {
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
    FloorDivision   Get  GetAttribute   GreaterOrEqual   GreaterThan
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