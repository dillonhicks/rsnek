//! PyInteger - Dynamically Sized Signed Integers
//!
//! ```ignore
//! int()
//! 1 + 3
//! ```
use std;
use std::fmt;
use std::borrow::Borrow;
use std::ops::Deref;

use num::{self, Zero, ToPrimitive};

use ::modules::precondition::{check_args, check_kwargs};
use ::api::result::Error;
use ::api::method::*;
use ::api::method::{Equal, Hashed, IntegerCast, StringCast, BooleanCast, NegateValue};
use ::api::RtObject;
use ::api::selfref::{self, SelfRef};
use ::api::{self, RtValue, method, typing};
use ::resources::strings;
use ::api::result::{RtResult, ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{BooleanProvider, StringProvider, FunctionProvider, IntegerProvider, FloatProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Native, HashId, SignatureBuilder};
use ::system::primitives as rs;
use ::objects::number::{self, FloatAdapter, IntAdapter, format_int};


const STATIC_INT_RANGE: std::ops::Range<isize> = -5..1024;
const TYPE_NAME: &'static str = "int";


#[derive(Clone)]
pub struct PyIntegerType {
    static_integers: Vec<RtObject>,
}


impl typing::BuiltinType for PyIntegerType {
    type T = PyInteger;
    type V = rs::Integer;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: rs::Integer) -> RtObject {
        match value.to_isize() {
            Some(idx @ -5..1024) => self.static_integers[(idx + 5) as usize].clone(),
            Some(_) |
            None => PyIntegerType::inject_selfref(PyIntegerType::alloc(value))
        }
    }


    fn init_type() -> Self {
        let range: Vec<RtObject> = STATIC_INT_RANGE
            .map(rs::Integer::from)
            .map(PyIntegerType::alloc)
            .map(PyIntegerType::inject_selfref)
            .collect();
        PyIntegerType {
            static_integers: range,
        }
    }

    fn inject_selfref(value: PyInteger) -> RtObject {
        let object = RtObject::new(Type::Int(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::Int(ref int) => {
                int.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyInteger {
            value: IntValue(value),
            rc: selfref::RefCount::default(),
        }
    }

}


pub struct IntValue(pub rs::Integer);
pub type PyInteger = RtValue<IntValue>;

impl PyInteger {
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
            "__abs__"       => {PyInteger::op_abs},
            "__bool__"      => {PyInteger::op_bool},
            "__float__"     => {PyInteger::op_float},
            "__hash__"      => {PyInteger::op_hash},
            "__index__"     => {PyInteger::op_index},
            "__int__"       => {PyInteger::op_int},
            "__invert__"    => {PyInteger::op_invert},
            "__neg__"       => {PyInteger::op_neg},
            "__pos__"       => {PyInteger::op_pos},
            "__repr__"      => {PyInteger::op_repr},
            "__str__"       => {PyInteger::op_str},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        unary_method_wrapper!(self, TYPE_NAME, name, rt, Type::Int, func)
    }

    fn try_get_binary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__add__"          => {PyInteger::op_add},
            "__and__"          => {PyInteger::op_and},
            "__delattr__"      => {PyInteger::op_delattr},
            "__divmod__"       => {PyInteger::op_divmod},
            "__eq__"           => {PyInteger::op_eq},
            "__floordiv__"     => {PyInteger::op_floordiv},
            "__ge__"           => {PyInteger::op_ge},
            "__getattribute__" => {PyInteger::op_getattribute},
            "__gt__"           => {PyInteger::op_gt},
            "__le__"           => {PyInteger::op_le},
            "__lshift__"       => {PyInteger::op_lshift},
            "__lt__"           => {PyInteger::op_lt},
            "__mod__"          => {PyInteger::op_mod},
            "__mul__"          => {PyInteger::op_mul},
            "__ne__"           => {PyInteger::op_ne},
            "__or__"           => {PyInteger::op_or},
            "__radd__"         => {PyInteger::op_radd},
            "__rand__"         => {PyInteger::op_rand},
            "__rdivmod__"      => {PyInteger::op_rdivmod},
            "__rfloordiv__"    => {PyInteger::op_rfloordiv},
            "__rlshift__"      => {PyInteger::op_rlshift},
            "__rmod__"         => {PyInteger::op_rmod},
            "__rmul__"         => {PyInteger::op_rmul},
            "__ror__"          => {PyInteger::op_ror},
            "__rrshift__"      => {PyInteger::op_rrshift},
            "__rshift__"       => {PyInteger::op_rshift},
            "__rsub__"         => {PyInteger::op_rsub},
            "__rtruediv__"     => {PyInteger::op_rtruediv},
            "__rxor__"         => {PyInteger::op_rxor},
            "__sub__"          => {PyInteger::op_sub},
            "__truediv__"      => {PyInteger::op_truediv},
            "__xor__"          => {PyInteger::op_xor},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        binary_method_wrapper!(self, TYPE_NAME, name, rt, Type::Int, func)
    }

    fn try_get_ternary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__pow__"          => {PyInteger::op_pow},
            //"__rpow__"         => {PyInteger::op_rpow},
            "__setattr__"      => {PyInteger::op_setattr},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(TYPE_NAME, missing)))
        };

        ternary_method_wrapper!(self, TYPE_NAME, name, rt, Type::Int, func)
    }

}


impl fmt::Display for PyInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}

impl fmt::Debug for PyInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl api::PyAPI for PyInteger {}


/// `self.rhs`
impl method::GetAttr for PyInteger {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {

        match name.as_ref() {
            &Type::Str(ref pystring) => {
                let string = pystring.value.0.clone();
                self.get_attribute(&rt, &string)
            }
            other => Err(Error::typerr(&format!(
                "getattr <{}>' requires string for attribute names, not {}",
                TYPE_NAME,
                other.debug_name())))
        }
    }
}


/// `hash(self)`
impl method::Hashed for PyInteger {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        let hash = self.native_hash()?;
        Ok(rt.int(hash))
    }

    fn native_hash(&self) -> RtResult<HashId> {
        Ok(number::hash_int(&self.value.0))
    }
}


/// `str(self)`
impl method::StringCast for PyInteger {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        let string = self.native_str()?;
        Ok(rt.str(string))
    }

    fn native_str(&self) -> RtResult<rs::String> {
        Ok(format_int(&self.value.0))
    }
}


/// `repr(self)`
impl method::StringRepresentation for PyInteger {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        let string = self.native_repr()?;
        Ok(rt.str(string))
    }

    fn native_repr(&self) -> RtResult<rs::String> {
        Ok(format_int(&self.value.0))
    }
}


/// `self == rhs`
impl method::Equal for PyInteger {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let value = self.native_eq(rhs.as_ref())?;
        Ok(rt.bool(value))
    }

    fn native_eq(&self, other: &Type) -> RtResult<rs::Boolean> {
        let lhs = IntAdapter(&self.value.0);

        match *other {
            Type::Bool(ref obj) => Ok(self.value.0 == obj.value.0),
            Type::Int(ref obj) => Ok(self.value.0 == obj.value.0),
            Type::Float(ref obj) => Ok(lhs == FloatAdapter(&obj.value.0)),
            _ => Ok(false),
        }
    }
}


/// `self != rhs`
impl method::NotEqual for PyInteger {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_ne(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_ne(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        let truth = !self.native_eq(rhs)?;
        Ok(truth)
    }
}

/// `bool(self)`
impl method::BooleanCast for PyInteger {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        if self.native_bool().unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_bool(&self) -> RtResult<rs::Boolean> {
        return Ok(!self.value.0.is_zero());
    }
}

/// `int(self)`
impl method::IntegerCast for PyInteger {
    #[allow(unused_variables)]
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        self.rc.upgrade()
    }

    fn native_int(&self) -> RtResult<rs::Integer> {
        return Ok(self.value.0.clone());
    }
}

/// `-self`
impl method::NegateValue for PyInteger {
    fn op_neg(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(- self.value.0.clone()))
    }

    fn native_neg(&self) -> RtResult<rs::Number> {
        Ok(rs::Number::Int(- self.value.0.clone()))
    }
}


/// `self + rhs`
impl method::Add for PyInteger {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match self.native_add(rhs.as_ref())? {
            Native::Int(int) => Ok(rt.int(int)),
            Native::Float(float) => Ok(rt.float(float)),
            _ => unreachable!()
        }
    }

    fn native_add(&self, rhs: &Type) -> RtResult<Native> {
        match rhs {
            &Type::Int(ref rhs) =>  Ok(Native::Int(&self.value.0 + &rhs.value.0)),
            &Type::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(Native::Float(lhs + rhs.value.0)),
                    None => Err(Error::overflow(
                        &format!("{:?} + {} overflows float", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("+", TYPE_NAME, other.debug_name())))
        }
    }

}

/// `self << rhs`
impl method::LeftShift for PyInteger {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {

        match rhs.as_ref() {
            &Type::Int(ref rhs) =>  {
                #[allow(unused_comparisons)]
                match rhs.value.0.to_usize() {
                    Some(int) if int < 0    => Err(Error::value(strings::ERROR_NEG_BIT_SHIFT)),
                    Some(int) if int == 0   => self.rc.upgrade(),
                    Some(int) if int > 0    => Ok(rt.int(&self.value.0 << int)),
                    Some(_)                 => unreachable!(),
                    None                    => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            },
            other => Err(Error::typerr(
                &strings_error_bad_operand!("<<", TYPE_NAME, other.debug_name())))
        }
    }
}

/// `self * rhs`
impl method::Multiply for PyInteger {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match rhs.as_ref() {
            &Type::Int(ref rhs) =>  Ok(rt.int(&self.value.0 * &rhs.value.0)),
            &Type::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(rt.float(lhs * rhs.value.0)),
                    None => Err(Error::overflow(
                        &format!("{:?} + {} overflows float", self.value.0, rhs.value.0))),
                }
            }

            other => Err(Error::typerr(
                &strings_error_bad_operand!("*", TYPE_NAME, other.debug_name()))),
        }
    }
}

/// `self ** rhs`
impl method::Pow for PyInteger {

    // TODO: modulus not currently used
    #[allow(unused_variables)]
    fn op_pow(&self, rt: &Runtime, exponent: &RtObject, modulus: &RtObject) -> ObjectResult {
        match exponent.as_ref() {
            &Type::Int(ref power) =>  {
                let base = self.value.0.clone();

                match power.value.0.to_usize() {
                    Some(int)  => Ok(rt.int(num::pow::pow(base, int))),
                    None  => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            },
            other => Err(Error::typerr(
                &strings_error_bad_operand!("**", TYPE_NAME, other.debug_name()))),
        }
    }
}

/// `self >> rhs`
impl method::RightShift for PyInteger {

    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {

        match rhs.as_ref() {
            &Type::Int(ref rhs) =>  {
                #[allow(unused_comparisons)]
                match rhs.value.0.to_usize() {
                    Some(int) if int > 0    => Ok(rt.int(&self.value.0 >> int)),
                    Some(int) if int == 0   => self.rc.upgrade(),
                    Some(int) if int < 0    => Err(Error::value(strings::ERROR_NEG_BIT_SHIFT)),
                    Some(_)                 => unreachable!(),
                    None                    => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            },
            other => Err(Error::typerr(
                &strings_error_bad_operand!(">>", TYPE_NAME, other.debug_name()))),
        }
    }
}

/// `self - rhs`
impl method::Subtract for PyInteger {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match rhs.as_ref() {
            &Type::Int(ref rhs) =>  Ok(rt.int(&self.value.0 - &rhs.value.0)),
            &Type::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(rt.float(lhs - rhs.value.0)),
                    None => Err(Error::overflow(&format!(
                        "{:?} + {} overflows", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("-", TYPE_NAME, other.debug_name())))
        }
    }

}

method_not_implemented!(PyInteger,
    New   Init   Delete   GetAttribute   
    SetAttr   DelAttr   Id   Is   
    IsNot   BytesCast   StringFormat   LessThan   
    LessOrEqual   GreaterOrEqual   GreaterThan   FloatCast   
    ComplexCast   Rounding   Index   AbsValue   
    PositiveValue   InvertValue   BitwiseAnd   DivMod   
    FloorDivision   Modulus   BitwiseOr   MatrixMultiply   
    TrueDivision   XOr   ReflectedAdd   ReflectedBitwiseAnd   
    ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   ReflectedModulus   
    ReflectedMultiply   ReflectedMatrixMultiply   ReflectedBitwiseOr   ReflectedPow   
    ReflectedRightShift   ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   
    InPlaceAdd   InPlaceBitwiseAnd   InPlaceDivMod   InPlaceFloorDivision   
    InPlaceLeftShift   InPlaceModulus   InPlaceMultiply   InPlaceMatrixMultiply   
    InPlaceBitwiseOr   InPlacePow   InPlaceRightShift   InPlaceSubtract   
    InPlaceTrueDivision   InPlaceXOr   Contains   Iter   
    Call   Length   LengthHint   Next   
    Reversed   GetItem   SetItem   DeleteItem   
    Count   Append   Extend   Pop   
    Remove   IsDisjoint   AddItem   Discard   
    Clear   Get   Keys   Values   
    Items   PopItem   Update   SetDefault   
    Await   Send   Throw   Close   
    Exit   Enter   DescriptorGet   DescriptorSet   
    DescriptorSetName
);


#[cfg(test)]
#[allow(non_snake_case)]
mod benches {
    #[allow(unused_imports)]
    use ::runtime::traits::{IteratorProvider, BooleanProvider, IntegerProvider,
                 StringProvider, NoneProvider, TupleProvider};
    use api::method::*;
    use test::Bencher;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    mod __add__ {
        use super::*;

        #[bench]
        fn static_static(b: &mut Bencher) {
            let rt = setup_test();

            let one = rt.int(1);
            let two = rt.int(2);

            b.iter(|| {
                one.op_add(&rt, &two).unwrap();
            });
        }

        #[bench]
        fn dynamic_static(b: &mut Bencher) {
            let rt = setup_test();

            let big_dynamic = rt.int(1000000);
            let two = rt.int(2);

            b.iter(|| {
                big_dynamic.op_add(&rt, &two).unwrap();
            });
        }

        #[bench]
        fn dynamic_dynamic(b: &mut Bencher) {
            let rt = setup_test();

            let lhs = rt.int(12345);
            let rhs = rt.int(67890);

            b.iter(|| {
                lhs.op_add(&rt, &rhs).unwrap();
            });
        }
    }

    mod native__add__ {
        use super::*;

        #[bench]
        fn static_static(b: &mut Bencher) {
            let rt = setup_test();

            let one = rt.int(1);
            let two = rt.int(2);

            let rhs = two.as_ref();
            b.iter(|| {
                one.native_add(rhs).unwrap();
            });
        }

        #[bench]
        fn dynamic_static(b: &mut Bencher) {
            let rt = setup_test();

            let big_dynamic = rt.int(1000000);
            let two = rt.int(2);
            let rhs = two.as_ref();

            b.iter(|| {
                big_dynamic.native_add(rhs).unwrap();
            });
        }

        #[bench]
        fn dynamic_dynamic(b: &mut Bencher) {
            let rt = setup_test();

            let lhs = rt.int(12345);
            let big = rt.int(67890);
            let rhs = big.as_ref();

            b.iter(|| {
                lhs.native_add(rhs).unwrap();
            });
        }

    }
}