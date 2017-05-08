//! `Type` is an enum wrapper for all known builtin types to act as a generic proxy
//! for `PyAPI` trait methods.
//!
//! Note that `Type` contains both internal and external types. It is not necessary to expose all
//! well known types as externally accessible types.
//!
use std;
use std::fmt;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

use ::api::method::{self, Id, StringRepresentation, Equal, Hashed};
use ::api::PyAPI;
use ::api::result::{RtResult, ObjectResult};
use ::api::RtObject;
use ::api::selfref::SelfRef;
use ::api::WeakRtObject as WeakRtObject;
use ::objects::boolean::PyBoolean;
use ::objects::bytes::PyBytes;
use ::objects::code::PyCode;
use ::objects::complex::PyComplex;
use ::objects::dictionary::PyDict;
use ::objects::float::PyFloat;
use ::objects::frame::PyFrame;
use ::objects::frozenset::PyFrozenSet;
use ::objects::integer::PyInteger;
use ::objects::iterator::PyIterator;
use ::objects::list::PyList;
use ::objects::method::PyFunction;
use ::objects::none::PyNone;
use ::objects::object::PyObject;
use ::objects::pytype::PyType;
use ::objects::set::PySet;
use ::objects::string::PyString;
use ::objects::tuple::PyTuple;
use ::runtime::Runtime;
use ::runtime::traits::{IntegerProvider, BooleanProvider};
use ::system::primitives as rs;
use ::system::primitives::{Native};


#[allow(dead_code)]
pub enum Type {
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
    DictKey(rs::DictKey),
}


impl Type {
    /// Since we do not have actual type objects just statically map the names of
    /// all of the types to a constant string.
    pub fn debug_name(&self) -> &str {

        match *self {
            Type::Object(_) => "object",
            Type::None(_) => "NoneType",
            Type::Bool(_) => "bool",
            Type::Int(_) => "int",
            Type::Float(_) => "float",
            Type::Iter(_) => "iterator",
            Type::Complex(_) => "complex",
            Type::Str(_) => "str",
            Type::Bytes(_) => "bytes",
            Type::Dict(_) => "dict",
            Type::Tuple(_) => "tuple",
            Type::List(_) => "list",
            Type::Type(_) => "type",
            Type::Function(_) => "function",
            Type::Module(_) => "module",
            Type::Code(_) => "code",
            Type::Frame(_) => "frame",
            Type::Set(_) => "set",
            Type::FrozenSet(_) => "frozenset",
            Type::DictKey(_) => "dictkey",
        }
    }
}

impl std::cmp::PartialEq for Type {
    fn eq(&self, rhs: &Type) -> bool {
        self.native_eq(rhs).unwrap_or(false)
    }
}

impl std::cmp::Eq for Type {}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        expr_foreach_type!(self, obj, {
            write!(f, "{:?}", obj)
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.native_repr() {
            Ok(string) => write!(f, "{}", string),
            _ => write!(f, "BuiltinType(repr_error=True)"),
        }
    }
}

impl Iterator for Type {
    type Item = RtObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Type::Iter(ref mut iterator) => {
                iterator.next()
            }
            _ => None
        }
    }
}

impl Hash for Type {
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


impl SelfRef for Type {
    fn strong_count(&self) -> rs::Integer {
        expr_foreach_type!(self, obj, {
            obj.rc.strong_count()
        })
    }

    fn weak_count(&self) -> rs::Integer {
        expr_foreach_type!(self, obj, {
            obj.rc.weak_count()
        })
    }

    fn set(&self, objref: &RtObject) {
        expr_foreach_type!(self, obj, {
            obj.rc.set(objref)
        })
    }

    fn get(&self) -> WeakRtObject {
        expr_foreach_type!(self, obj, {
            obj.rc.get()
        })
    }

    fn upgrade(&self) -> ObjectResult {
        expr_foreach_type!(self, obj, {
            obj.rc.upgrade()
        })
    }
}


impl PyAPI for Type {}

impl method::GetAttr for Type {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Type) -> RtResult<RtObject> {
        native_foreach_type!(self, native_getattr, lhs, name)
    }
}


impl method::SetAttr for Type {
    fn op_setattr(&self, rt: &Runtime, name: &RtObject, value: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Type, value: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self, native_setattr, lhs, name, value)
    }
}

impl method::Id for Type {
    fn op_id(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> rs::ObjectId {
        expr_foreach_type!(self, obj, {
            (obj as *const _) as rs::ObjectId
        })
    }
}


impl method::Is for Type {
    fn op_is(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() == rhs.native_id())
    }
}

impl method::IsNot for Type {
    fn op_is_not(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is_not(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is_not(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() != rhs.native_id())
    }
}

impl method::Hashed for Type {
    //
    // Hash
    //
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> RtResult<rs::HashId> {
        native_foreach_type!(self, native_hash, obj)
    }
}

impl method::StringCast for Type {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_str, obj)
    }

    fn native_str(&self) -> RtResult<rs::String> {
        native_foreach_type!(self, native_str, obj)
    }
}
impl method::BytesCast for Type {
    fn op_bytes(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> RtResult<rs::Bytes> {
        native_foreach_type!(self, native_bytes, obj)
    }
}
impl method::StringFormat for Type {
    fn op_format(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_format, obj)
    }

    fn native_format(&self) -> RtResult<rs::String> {
        native_foreach_type!(self, native_format, obj)
    }
}
impl method::StringRepresentation for Type {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_repr, obj)
    }

    fn native_repr(&self) -> RtResult<rs::String> {
        native_foreach_type!(self, native_repr, obj)
    }
}

impl method::Equal for Type {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_eq, lhs, rhs)
    }
}
impl method::NotEqual for Type {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_ne, lhs, rhs)
    }
}

impl method::LessThan for Type {
    fn op_lt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_lt, lhs, rhs)
    }
}
impl method::LessOrEqual for Type {
    fn op_le(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_le, lhs, rhs)
    }
}
impl method::GreaterOrEqual for Type {
    fn op_ge(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_ge, lhs, rhs)
    }
}
impl method::GreaterThan for Type {
    fn op_gt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_gt, lhs, rhs)
    }
}
impl method::BooleanCast for Type {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_bool, obj)
    }

    fn native_bool(&self) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_bool, obj)
    }
}

impl method::IntegerCast for Type {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_int, obj)
    }

    fn native_int(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self, native_int, obj)
    }
}

impl method::FloatCast for Type {
    fn op_float(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_float, obj)
    }

    fn native_float(&self) -> RtResult<rs::Float> {
        native_foreach_type!(self, native_float, obj)
    }
}

impl method::ComplexCast for Type {
    fn op_complex(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_complex, obj)
    }

    fn native_complex(&self) -> RtResult<rs::Complex> {
        native_foreach_type!(self, native_complex, obj)
    }
}

impl method::Index for Type {
    fn op_index(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_index, obj)
    }

    fn native_index(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self, native_index, obj)
    }
}
impl method::NegateValue for Type {
    fn op_neg(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_neg, obj)
    }

    fn native_neg(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self, native_neg, obj)
    }
}
impl method::AbsValue for Type {
    fn op_abs(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_abs, obj)
    }

    fn native_abs(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self, native_abs, obj)
    }
}
impl method::PositiveValue for Type {
    fn op_pos(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_pos, obj)
    }

    fn native_pos(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self, native_pos, obj)
    }
}
impl method::InvertValue for Type {
    fn op_invert(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_invert, obj)
    }

    fn native_invert(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self, native_invert, obj)
    }
}
impl method::Add for Type {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Type) -> RtResult<Native> {
        native_foreach_type!(self, native_add, lhs, rhs)
    }
}
impl method::BitwiseAnd for Type {
    fn op_and(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_and, lhs, rhs)
    }
}
impl method::DivMod for Type {
    fn op_divmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_divmod, lhs, rhs)
    }
}
impl method::FloorDivision for Type {
    fn op_floordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_floordiv, lhs, rhs)
    }
}
impl method::LeftShift for Type {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_lshift, lhs, rhs)
    }
}
impl method::Modulus for Type {
    fn op_mod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_mod, lhs, rhs)
    }
}
impl method::Multiply for Type {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Type) -> RtResult<rs::Native> {
        native_foreach_type!(self, native_mul, lhs, rhs)
    }
}
impl method::MatrixMultiply for Type {
    fn op_matmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_matmul, lhs, rhs)
    }
}
impl method::BitwiseOr for Type {
    fn op_or(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_or, lhs, rhs)
    }
}
impl method::Pow for Type {
    fn op_pow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Type, modulus: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_pow, base, power, modulus)
    }
}
impl method::RightShift for Type {
    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_rshift, lhs, rhs)
    }
}
impl method::Subtract for Type {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_sub, lhs, rhs)
    }
}
impl method::TrueDivision for Type {
    fn op_truediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_truediv, lhs, rhs)
    }
}
impl method::XOr for Type {
    fn op_xor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_xor, lhs, rhs)
    }
}

impl method::InPlaceAdd for Type {
    fn op_iadd(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_iadd, lhs, rhs)
    }
}
impl method::InPlaceBitwiseAnd for Type {
    fn op_iand(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_iand, lhs, rhs)
    }
}
impl method::InPlaceDivMod for Type {
    fn op_idivmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_idivmod, lhs, rhs)
    }
}
impl method::InPlaceFloorDivision for Type {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_ifloordiv, lhs, rhs)
    }
}
impl method::InPlaceLeftShift for Type {
    fn op_ilshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_ilshift, lhs, rhs)
    }
}
impl method::InPlaceModulus for Type {
    fn op_imod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_imod, lhs, rhs)
    }
}
impl method::InPlaceMultiply for Type {
    fn op_imul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_imul, lhs, rhs)
    }
}
impl method::InPlaceMatrixMultiply for Type {
    fn op_imatmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_imatmul, lhs, rhs)
    }
}
impl method::InPlaceBitwiseOr for Type {
    fn op_ior(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_ior, lhs, rhs)
    }
}
impl method::InPlacePow for Type {
    fn op_ipow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Type, modulus: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_ipow, base, power, modulus)
    }
}
impl method::InPlaceRightShift for Type {
    fn op_irshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_irshift, lhs, rhs)
    }
}
impl method::InPlaceSubtract for Type {
    fn op_isub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_isub, lhs, rhs)
    }
}
impl method::InPlaceTrueDivision for Type {
    fn op_itruediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_itruediv, lhs, rhs)
    }
}
impl method::InPlaceXOr for Type {
    fn op_ixor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_ixor, lhs, rhs)
    }
}
impl method::Contains for Type {
    fn op_contains(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_contains, lhs, rhs)
    }
}
impl method::Iter for Type {
    fn op_iter(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_iter, lhs)
    }

    fn native_iter(&self) -> RtResult<rs::Iterator> {
        native_foreach_type!(self, native_iter, lhs)
    }
}
impl method::Call for Type {
    fn op_call(&self, rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Type, starargs: &Type, kwargs: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_call, method, pos_args, starargs, kwargs)
    }
}
impl method::Length for Type {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_len, lhs)
    }

    fn native_len(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self, native_len, lhs)
    }
}

impl method::Next for Type {
    fn op_next(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, op_next, lhs)
    }

    fn native_next(&self) -> RtResult<RtObject> {
        native_foreach_type!(self, native_next, lhs)
    }    
}

impl method::GetItem for Type {
    #[inline]
    fn op_getitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_getitem, object, name)
    }

    #[inline]
    fn native_getitem(&self, name: &Type) -> RtResult<RtObject> {
        native_foreach_type!(self, native_getitem, object, name)
    }
}

impl method::SetItem for Type {
    fn op_setitem(&self, rt: &Runtime, name: &RtObject, item: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Type, item: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self, native_setitem, object, name, item)
    }
}

impl method::DeleteItem for Type {
    fn op_delitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_delitem, object, name)
    }
}

impl method::Count for Type {
    fn meth_count(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Type) -> RtResult<rs::Integer> {
        native_foreach_type!(self, native_meth_count, object, name)
    }
}

impl method::Append for Type {
    fn meth_append(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self, native_meth_append, object, name)
    }
}

impl method::Extend for Type {
    fn meth_extend(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self, native_meth_extend, object, name)
    }
}

impl method::Pop for Type {
    fn meth_pop(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_meth_pop, object, name)
    }
}

impl method::Remove for Type {
    fn meth_remove(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_meth_remove, object, name)
    }
}

impl method::IsDisjoint for Type {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self, native_meth_isdisjoint, object, name)
    }
}

impl method::AddItem for Type {
    fn meth_add(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self, rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self, native_meth_add, object, name)
    }
}


impl method::Keys for Type {
    fn meth_keys(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self, rt, meth_keys, object)
    }

    fn native_meth_keys(&self) -> RtResult<rs::Tuple> {
        native_foreach_type!(self, native_meth_keys, object)
    }    
}


method_not_implemented!(Type,
    Await   Clear   Close   DelAttr   Delete   
    DescriptorGet   DescriptorSet   DescriptorSetName   Discard   Enter   
    Exit   Get   GetAttribute   Init   Items   
    LengthHint   New   PopItem   ReflectedAdd   ReflectedBitwiseAnd   
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift
    ReflectedMatrixMultiply  ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift   ReflectedSubtract  ReflectedTrueDivision   ReflectedXOr   Reversed
    Rounding   Send   SetDefault   Throw   Update   Values
);
