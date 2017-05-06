use std;
use std::fmt;
use std::borrow::Borrow;
use std::ops::Deref;


use num::{self, Zero, ToPrimitive};

use runtime::Runtime;
use traits::{BooleanProvider, StringProvider, FunctionProvider, IntegerProvider, FloatProvider};
use resource::strings;
use error::Error;
use result::{NativeResult, RuntimeResult};
use object::{self, RtValue, method, typing};
use object::selfref::{self, SelfRef};
use object::method::{Equal, Hashed, IntegerCast, StringCast, BooleanCast, NegateValue};
use object::method::*;

use typedef::native::{self, HashId, SignatureBuilder};
use ::object::RtObject;
use typedef::builtin::Builtin;
use ::typedef::number::{self, FloatAdapter, IntAdapter, format_int};
use ::builtin::precondition::check_args;
use ::builtin::precondition::check_kwargs;

const STATIC_INT_RANGE: std::ops::Range<isize> = -5..1024;
const DOCSTRING: &'static str = r#"int(x=0) -> integer
int(x, base=10) -> integer

Convert a number or string to an integer, or return 0 if no arguments
are given.  If x is a number, return x.__int__().  For floating point
numbers, this truncates towards zero.

If x is not a number or if base is given, then x must be a string,
bytes, or bytearray instance representing an integer literal in the
given base.  The literal can be preceded by '+' or '-' and be surrounded
by whitespace.  The base defaults to 10.  Valid bases are 0 and 2-36.
Base 0 means to interpret the base from the string as an integer literal.
>>> int('0b100', base=0)
4
"#;

#[derive(Clone)]
pub struct PyIntegerType {
    pub static_integers: Vec<RtObject>,
}


impl typing::BuiltinType for PyIntegerType {
    type T = PyInteger;
    type V = native::Integer;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: native::Integer) -> RtObject {
        match value.to_isize() {
            Some(idx @ -5..1024) => self.static_integers[(idx + 5) as usize].clone(),
            Some(_) |
            None => PyIntegerType::inject_selfref(PyIntegerType::alloc(value))
        }
    }


    fn init_type() -> Self {
        let range: Vec<RtObject> = STATIC_INT_RANGE
            .map(native::Integer::from)
            .map(PyIntegerType::alloc)
            .map(PyIntegerType::inject_selfref)
            .collect();
        PyIntegerType { static_integers: range }
    }

    fn inject_selfref(value: PyInteger) -> RtObject {
        let objref = RtObject::new(Builtin::Int(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Int(ref int) => {
                int.rc.set(&objref.clone());
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


impl method::New for PyIntegerType {}
impl method::Init for PyIntegerType {}
impl method::Delete for PyIntegerType {}


pub struct IntValue(pub native::Integer);
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
    pub fn get_attribute(&self, rt: &Runtime, name: &str) -> RuntimeResult {
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
                &strings_error_no_attribute!("int", missing)))
        }
    }

    fn try_get_name(&self, rt: &Runtime, name: &str) -> RuntimeResult {
        match name {
            "__doc__" => Ok(rt.str(DOCSTRING)),
            missing => Err(Error::attribute(
                &strings_error_no_attribute!("int", missing)))
        }
    }

    fn try_get_unary_method(&self, rt: &Runtime, name: &str) -> RuntimeResult {
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
                &strings_error_no_attribute!("int", missing)))
        };

        unary_method_wrapper!(self, "int", name, rt, Builtin::Int, func)
    }

    fn try_get_binary_method(&self, rt: &Runtime, name: &str) -> RuntimeResult {
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
                &strings_error_no_attribute!("int", missing)))
        };

        binary_method_wrapper!(self, "int", name, rt, Builtin::Int, func)
    }

    fn try_get_ternary_method(&self, rt: &Runtime, name: &str) -> RuntimeResult {
        let func = match name {
            "__pow__"          => {PyInteger::op_pow},
            //"__rpow__"         => {PyInteger::op_rpow},
            "__setattr__"      => {PyInteger::op_setattr},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!("int", missing)))
        };

        ternary_method_wrapper!(self, "int", name, rt, Builtin::Int, func)
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
impl object::PyAPI for PyInteger {}


/// `self.rhs`
impl method::GetAttr for PyInteger {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {

        let boxed: &Box<Builtin> = name.0.borrow();
        match boxed.deref() {
            &Builtin::Str(ref pystring) => {
                let selfref = self.rc.upgrade()?;
                let string = pystring.value.0.clone();
                self.get_attribute(&rt, &string)
            }
            other => Err(Error::typerr(&format!(
                "getattr <int>' requires string for attribute names, not {}",
                other.debug_name())))
        }
    }
}


/// `hash(self)`
impl method::Hashed for PyInteger {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        let hash = self.native_hash()?;
        Ok(rt.int(hash))
    }

    fn native_hash(&self) -> NativeResult<HashId> {
        Ok(number::hash_int(&self.value.0))
    }
}


/// `str(self)`
impl method::StringCast for PyInteger {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        let string = self.native_str()?;
        Ok(rt.str(string))
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(format_int(&self.value.0))
    }
}


/// `repr(self)`
impl method::StringRepresentation for PyInteger {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        let string = self.native_repr()?;
        Ok(rt.str(string))
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        Ok(format_int(&self.value.0))
    }
}


/// `self == rhs`
impl method::Equal for PyInteger {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        let value = self.native_eq(builtin.deref())?;
        Ok(rt.bool(value))
    }

    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        let lhs = IntAdapter(&self.value.0);

        match *other {
            Builtin::Bool(ref obj) => Ok(self.value.0 == obj.value.0),
            Builtin::Int(ref obj) => Ok(self.value.0 == obj.value.0),
            Builtin::Float(ref obj) => Ok(lhs == FloatAdapter(&obj.value.0)),
            _ => Ok(false),
        }
    }
}


/// `self != rhs`
impl method::NotEqual for PyInteger {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        let truth = self.native_ne(builtin.deref())?;
        Ok(rt.bool(truth))
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let truth = !self.native_eq(rhs)?;
        Ok(truth)
    }
}

/// `bool(self)`
impl method::BooleanCast for PyInteger {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        if self.native_bool().unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.0.is_zero());
    }
}

/// `int(self)`
impl method::IntegerCast for PyInteger {
    #[allow(unused_variables)]
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        self.rc.upgrade()
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        return Ok(self.value.0.clone());
    }
}

/// `-self`
impl method::NegateValue for PyInteger {
    fn op_neg(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(- self.value.0.clone()))
    }

    fn native_neg(&self) -> NativeResult<native::Number> {
        Ok(native::Number::Int(- self.value.0.clone()))
    }
}


/// `self + rhs`
impl method::Add for PyInteger {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();


        match builtin.deref() {
            &Builtin::Int(ref rhs) =>  Ok(rt.int(&self.value.0 + &rhs.value.0)),
            &Builtin::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(rt.float(lhs + rhs.value.0)),
                    None => Err(Error::overflow(
                        &format!("{:?} + {} overflows float", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("+", "int", other.debug_name())))
        }
    }

}

/// `self << rhs`
impl method::LeftShift for PyInteger {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref rhs) =>  {
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
                &strings_error_bad_operand!("<<", "int", other.debug_name())))
        }
    }
}

/// `self * rhs`
impl method::Multiply for PyInteger {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref rhs) =>  Ok(rt.int(&self.value.0 * &rhs.value.0)),
            &Builtin::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(rt.float(lhs * rhs.value.0)),
                    None => Err(Error::overflow(
                        &format!("{:?} + {} overflows float", self.value.0, rhs.value.0))),
                }
            }

            other => Err(Error::typerr(
                &strings_error_bad_operand!("*", "int", other.debug_name()))),
        }
    }
}

/// `self ** rhs`
impl method::Pow for PyInteger {

    // TODO: modulus not currently used
    #[allow(unused_variables)]
    fn op_pow(&self, rt: &Runtime, exponent: &RtObject, modulus: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = exponent.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref power) =>  {
                let base = self.value.0.clone();

                match power.value.0.to_usize() {
                    Some(int)  => Ok(rt.int(num::pow::pow(base, int))),
                    None  => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            },
            other => Err(Error::typerr(
                &strings_error_bad_operand!("**", "int", other.debug_name()))),
        }
    }
}

/// `self >> rhs`
impl method::RightShift for PyInteger {

    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref rhs) =>  {
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
                &strings_error_bad_operand!(">>", "int", other.debug_name()))),
        }
    }
}

/// `self - rhs`
impl method::Subtract for PyInteger {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref rhs) =>  Ok(rt.int(&self.value.0 - &rhs.value.0)),
            &Builtin::Float(ref rhs) => {
                match self.value.0.to_f64() {
                    Some(lhs) => Ok(rt.float(lhs - rhs.value.0)),
                    None => Err(Error::overflow(&format!(
                        "{:?} + {} overflows", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("-", "int", other.debug_name())))
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
