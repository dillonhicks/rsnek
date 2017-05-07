use std;
use std::fmt;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

use runtime::Runtime;
use traits::{IntegerProvider, BooleanProvider};
use result::{RtResult, ObjectResult};

use ::api::PyAPI;
use ::api::RtObject;
use ::api::WeakRtObject as WeakRtObject;
use ::api::method::{self, Id, StringRepresentation, Equal, Hashed};
use ::api::selfref::SelfRef;

use ::objects::native::{self, Native};
use ::objects::dictionary::PyDict;
use ::objects::object::PyObject;
use ::objects::boolean::PyBoolean;
use ::objects::integer::PyInteger;
use ::objects::float::PyFloat;
use ::objects::iterator::PyIterator;
use ::objects::string::PyString;
use ::objects::bytes::PyBytes;
use ::objects::complex::PyComplex;
use ::objects::none::PyNone;
use ::objects::tuple::PyTuple;
use ::objects::list::PyList;
use ::objects::pytype::PyType;
use ::objects::method::PyFunction;
use ::objects::code::PyCode;
use ::objects::frame::PyFrame;
use ::objects::set::PySet;
use ::objects::frozenset::PyFrozenSet;


#[allow(dead_code)]
pub enum Builtin {
    Object(PyObject),
    None(PyNone),
    Bool(PyBoolean),
    Int(PyInteger),
    Float(PyFloat),
    Iter(PyIterator),
    Complex(PyComplex),
    Str(PyString),
    Bytes(PyBytes),
    Dict(PyDict),
    Tuple(PyTuple),
    List(PyList),
    Type(PyType),
    Function(PyFunction),
    Module(PyObject),
    Code(PyCode),
    Frame(PyFrame),
    Set(PySet),
    FrozenSet(PyFrozenSet),

    // Utility Types
    DictKey(native::DictKey),
}

impl Builtin {
    pub fn debug_name(&self) -> &str {

        match *self {
            Builtin::Object(_) => "object",
            Builtin::None(_) => "NoneType",
            Builtin::Bool(_) => "bool",
            Builtin::Int(_) => "int",
            Builtin::Float(_) => "float",
            Builtin::Iter(_) => "iterator",
            Builtin::Complex(_) => "complex",
            Builtin::Str(_) => "str",
            Builtin::Bytes(_) => "bytes",
            Builtin::Dict(_) => "dict",
            Builtin::Tuple(_) => "tuple",
            Builtin::List(_) => "list",
            Builtin::Type(_) => "type",
            Builtin::Function(_) => "function",
            Builtin::Module(_) => "module",
            Builtin::Code(_) => "code",
            Builtin::Frame(_) => "frame",
            Builtin::Set(_) => "set",
            Builtin::FrozenSet(_) => "frozenset",
            Builtin::DictKey(_) => "dictkey",
        }
    }
}

impl std::cmp::PartialEq for Builtin {
    fn eq(&self, rhs: &Builtin) -> bool {
        self.native_eq(rhs).unwrap_or(false)
    }
}

impl std::cmp::Eq for Builtin {}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        expr_foreach_builtin!(self, obj, {
            write!(f, "{:?}", obj)
        })
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.native_repr() {
            Ok(string) => write!(f, "{}", string),
            _ => write!(f, "BuiltinType(repr_error=True)"),
        }
    }
}

impl Iterator for Builtin {
    type Item = RtObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Builtin::Iter(ref mut iterator) => {
                iterator.next()
            }
            _ => None
        }
    }
}

impl Hash for Builtin {
    fn hash<H: Hasher>(&self, state: &mut H) where H: Hasher{
        self.native_hash().unwrap().hash(state);
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//     New Object Traits
//
// For the BuiltinObject this should mean just proxy dispatching the
// underlying associated function using the foreach macros.
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl SelfRef for Builtin {
    fn strong_count(&self) -> native::Integer {
        expr_foreach_builtin!(self, obj, {
            obj.rc.strong_count()
        })
    }

    fn weak_count(&self) -> native::Integer {
        expr_foreach_builtin!(self, obj, {
            obj.rc.weak_count()
        })
    }

    fn set(&self, objref: &RtObject) {
        expr_foreach_builtin!(self, obj, {
            obj.rc.set(objref)
        })
    }

    fn get(&self) -> WeakRtObject {
        expr_foreach_builtin!(self, obj, {
            obj.rc.get()
        })
    }

    fn upgrade(&self) -> ObjectResult {
        expr_foreach_builtin!(self, obj, {
            obj.rc.upgrade()
        })
    }
}


impl PyAPI for Builtin {}

impl method::GetAttr for Builtin {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Builtin) -> RtResult<RtObject> {
        native_foreach_builtin!(self, native_getattr, lhs, name)
    }
}


impl method::SetAttr for Builtin {
    fn op_setattr(&self, rt: &Runtime, name: &RtObject, value: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> RtResult<native::None> {
        native_foreach_builtin!(self, native_setattr, lhs, name, value)
    }
}

impl method::Id for Builtin {
    fn op_id(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> native::ObjectId {
        expr_foreach_builtin!(self, obj, {
            (obj as *const _) as native::ObjectId
        })
    }
}


impl method::Is for Builtin {
    fn op_is(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        Ok(self.native_id() == rhs.native_id())
    }
}

impl method::IsNot for Builtin {
    fn op_is_not(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is_not(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is_not(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        Ok(self.native_id() != rhs.native_id())
    }
}

impl method::Hashed for Builtin {
    //
    // Hash
    //
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> RtResult<native::HashId> {
        native_foreach_builtin!(self, native_hash, obj)
    }
}

impl method::StringCast for Builtin {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_str, obj)
    }

    fn native_str(&self) -> RtResult<native::String> {
        native_foreach_builtin!(self, native_str, obj)
    }
}
impl method::BytesCast for Builtin {
    fn op_bytes(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> RtResult<native::Bytes> {
        native_foreach_builtin!(self, native_bytes, obj)
    }
}
impl method::StringFormat for Builtin {
    fn op_format(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_format, obj)
    }

    fn native_format(&self) -> RtResult<native::String> {
        native_foreach_builtin!(self, native_format, obj)
    }
}
impl method::StringRepresentation for Builtin {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_repr, obj)
    }

    fn native_repr(&self) -> RtResult<native::String> {
        native_foreach_builtin!(self, native_repr, obj)
    }
}

impl method::Equal for Builtin {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_eq, lhs, rhs)
    }
}
impl method::NotEqual for Builtin {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_ne, lhs, rhs)
    }
}

impl method::LessThan for Builtin {
    fn op_lt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_lt, lhs, rhs)
    }
}
impl method::LessOrEqual for Builtin {
    fn op_le(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_le, lhs, rhs)
    }
}
impl method::GreaterOrEqual for Builtin {
    fn op_ge(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_ge, lhs, rhs)
    }
}
impl method::GreaterThan for Builtin {
    fn op_gt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_gt, lhs, rhs)
    }
}
impl method::BooleanCast for Builtin {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_bool, obj)
    }

    fn native_bool(&self) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_bool, obj)
    }
}

impl method::IntegerCast for Builtin {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_int, obj)
    }

    fn native_int(&self) -> RtResult<native::Integer> {
        native_foreach_builtin!(self, native_int, obj)
    }
}

impl method::FloatCast for Builtin {
    fn op_float(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_float, obj)
    }

    fn native_float(&self) -> RtResult<native::Float> {
        native_foreach_builtin!(self, native_float, obj)
    }
}

impl method::ComplexCast for Builtin {
    fn op_complex(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_complex, obj)
    }

    fn native_complex(&self) -> RtResult<native::Complex> {
        native_foreach_builtin!(self, native_complex, obj)
    }
}

impl method::Index for Builtin {
    fn op_index(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_index, obj)
    }

    fn native_index(&self) -> RtResult<native::Integer> {
        native_foreach_builtin!(self, native_index, obj)
    }
}
impl method::NegateValue for Builtin {
    fn op_neg(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_neg, obj)
    }

    fn native_neg(&self) -> RtResult<native::Number> {
        native_foreach_builtin!(self, native_neg, obj)
    }
}
impl method::AbsValue for Builtin {
    fn op_abs(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_abs, obj)
    }

    fn native_abs(&self) -> RtResult<native::Number> {
        native_foreach_builtin!(self, native_abs, obj)
    }
}
impl method::PositiveValue for Builtin {
    fn op_pos(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_pos, obj)
    }

    fn native_pos(&self) -> RtResult<native::Number> {
        native_foreach_builtin!(self, native_pos, obj)
    }
}
impl method::InvertValue for Builtin {
    fn op_invert(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_invert, obj)
    }

    fn native_invert(&self) -> RtResult<native::Number> {
        native_foreach_builtin!(self, native_invert, obj)
    }
}
impl method::Add for Builtin {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> RtResult<Native> {
        native_foreach_builtin!(self, native_add, lhs, rhs)
    }
}
impl method::BitwiseAnd for Builtin {
    fn op_and(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_and, lhs, rhs)
    }
}
impl method::DivMod for Builtin {
    fn op_divmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_divmod, lhs, rhs)
    }
}
impl method::FloorDivision for Builtin {
    fn op_floordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_floordiv, lhs, rhs)
    }
}
impl method::LeftShift for Builtin {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_lshift, lhs, rhs)
    }
}
impl method::Modulus for Builtin {
    fn op_mod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_mod, lhs, rhs)
    }
}
impl method::Multiply for Builtin {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Builtin) -> RtResult<native::Native> {
        native_foreach_builtin!(self, native_mul, lhs, rhs)
    }
}
impl method::MatrixMultiply for Builtin {
    fn op_matmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_matmul, lhs, rhs)
    }
}
impl method::BitwiseOr for Builtin {
    fn op_or(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_or, lhs, rhs)
    }
}
impl method::Pow for Builtin {
    fn op_pow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Builtin, modulus: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_pow, base, power, modulus)
    }
}
impl method::RightShift for Builtin {
    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_rshift, lhs, rhs)
    }
}
impl method::Subtract for Builtin {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_sub, lhs, rhs)
    }
}
impl method::TrueDivision for Builtin {
    fn op_truediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_truediv, lhs, rhs)
    }
}
impl method::XOr for Builtin {
    fn op_xor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_xor, lhs, rhs)
    }
}

impl method::InPlaceAdd for Builtin {
    fn op_iadd(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_iadd, lhs, rhs)
    }
}
impl method::InPlaceBitwiseAnd for Builtin {
    fn op_iand(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_iand, lhs, rhs)
    }
}
impl method::InPlaceDivMod for Builtin {
    fn op_idivmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_idivmod, lhs, rhs)
    }
}
impl method::InPlaceFloorDivision for Builtin {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_ifloordiv, lhs, rhs)
    }
}
impl method::InPlaceLeftShift for Builtin {
    fn op_ilshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_ilshift, lhs, rhs)
    }
}
impl method::InPlaceModulus for Builtin {
    fn op_imod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_imod, lhs, rhs)
    }
}
impl method::InPlaceMultiply for Builtin {
    fn op_imul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_imul, lhs, rhs)
    }
}
impl method::InPlaceMatrixMultiply for Builtin {
    fn op_imatmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_imatmul, lhs, rhs)
    }
}
impl method::InPlaceBitwiseOr for Builtin {
    fn op_ior(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_ior, lhs, rhs)
    }
}
impl method::InPlacePow for Builtin {
    fn op_ipow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Builtin, modulus: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_ipow, base, power, modulus)
    }
}
impl method::InPlaceRightShift for Builtin {
    fn op_irshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_irshift, lhs, rhs)
    }
}
impl method::InPlaceSubtract for Builtin {
    fn op_isub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_isub, lhs, rhs)
    }
}
impl method::InPlaceTrueDivision for Builtin {
    fn op_itruediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_itruediv, lhs, rhs)
    }
}
impl method::InPlaceXOr for Builtin {
    fn op_ixor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_ixor, lhs, rhs)
    }
}
impl method::Contains for Builtin {
    fn op_contains(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_contains, lhs, rhs)
    }
}
impl method::Iter for Builtin {
    fn op_iter(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_iter, lhs)
    }

    fn native_iter(&self) -> RtResult<native::Iterator> {
        native_foreach_builtin!(self, native_iter, lhs)
    }
}
impl method::Call for Builtin {
    fn op_call(&self, rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Builtin, starargs: &Builtin, kwargs: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_call, method, pos_args, starargs, kwargs)
    }
}
impl method::Length for Builtin {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_len, lhs)
    }

    fn native_len(&self) -> RtResult<native::Integer> {
        native_foreach_builtin!(self, native_len, lhs)
    }
}

impl method::Next for Builtin {
    fn op_next(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, op_next, lhs)
    }

    fn native_next(&self) -> RtResult<RtObject> {
        native_foreach_builtin!(self, native_next, lhs)
    }    
}

impl method::GetItem for Builtin {
    #[inline]
    fn op_getitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_getitem, object, name)
    }

    #[inline]
    fn native_getitem(&self, name: &Builtin) -> RtResult<RtObject> {
        native_foreach_builtin!(self, native_getitem, object, name)
    }
}

impl method::SetItem for Builtin {
    fn op_setitem(&self, rt: &Runtime, name: &RtObject, item: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Builtin, item: &Builtin) -> RtResult<native::None> {
        native_foreach_builtin!(self, native_setitem, object, name, item)
    }
}

impl method::DeleteItem for Builtin {
    fn op_delitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_delitem, object, name)
    }
}

impl method::Count for Builtin {
    fn meth_count(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Builtin) -> RtResult<native::Integer> {
        native_foreach_builtin!(self, native_meth_count, object, name)
    }
}

impl method::Append for Builtin {
    fn meth_append(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Builtin) -> RtResult<native::None> {
        native_foreach_builtin!(self, native_meth_append, object, name)
    }
}

impl method::Extend for Builtin {
    fn meth_extend(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Builtin) -> RtResult<native::None> {
        native_foreach_builtin!(self, native_meth_extend, object, name)
    }
}

impl method::Pop for Builtin {
    fn meth_pop(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_meth_pop, object, name)
    }
}

impl method::Remove for Builtin {
    fn meth_remove(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_meth_remove, object, name)
    }
}

impl method::IsDisjoint for Builtin {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Builtin) -> RtResult<native::Boolean> {
        native_foreach_builtin!(self, native_meth_isdisjoint, object, name)
    }
}

impl method::AddItem for Builtin {
    fn meth_add(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_builtin!(self, rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Builtin) -> RtResult<Builtin> {
        native_foreach_builtin!(self, native_meth_add, object, name)
    }
}


impl method::Keys for Builtin {
    fn meth_keys(&self, rt: &Runtime) -> ObjectResult {
        foreach_builtin!(self, rt, meth_keys, object)
    }

    fn native_meth_keys(&self) -> RtResult<native::Tuple> {
        native_foreach_builtin!(self, native_meth_keys, object)
    }    
}


method_not_implemented!(Builtin,
    Await   Clear   Close   DelAttr   Delete   
    DescriptorGet   DescriptorSet   DescriptorSetName   Discard   Enter   
    Exit   Get   GetAttribute   Init   Items   
    LengthHint   New   PopItem   ReflectedAdd   ReflectedBitwiseAnd   
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift
    ReflectedMatrixMultiply  ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift   ReflectedSubtract  ReflectedTrueDivision   ReflectedXOr   Reversed
    Rounding   Send   SetDefault   Throw   Update   Values
);
