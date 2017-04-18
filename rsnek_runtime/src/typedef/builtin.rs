use std;
use std::fmt;
use std::borrow::Borrow;

use runtime::{Runtime, IntegerProvider, BooleanProvider};
use result::{NativeResult, RuntimeResult};

use object;
use object::method::{self, Id, StringRepresentation, Equal};
use object::selfref::SelfRef;

use typedef::native;
use typedef::dictionary::PyDict;
use typedef::object::PyObject;
use typedef::boolean::PyBoolean;
use typedef::integer::PyInteger;
use typedef::float::PyFloat;
use typedef::iterator::PyIterator;
use typedef::string::PyString;
use typedef::bytes::PyBytes;
use typedef::complex::PyComplex;
use typedef::none::PyNone;
use typedef::tuple::PyTuple;
use typedef::objectref::{ObjectRef, WeakObjectRef};
use typedef::pytype::PyType;
use typedef::method::PyFunction;
use typedef::code::PyCode;


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
    Type(PyType),
    Function(PyFunction),
    Module(PyObject),
    Code(PyCode),

    // Utility Types
    DictKey(native::DictKey),
}

impl Builtin {
    pub fn debug_name(&self) -> &str{
        match *self {
            Builtin::Object(_) => "object",
            Builtin::None(_) => "none",
            Builtin::Bool(_) => "bool",
            Builtin::Int(_) => "int",
            Builtin::Float(_) => "float",
            Builtin::Iter(_) => "iter",
            Builtin::Complex(_) => "complex",
            Builtin::Str(_) => "str",
            Builtin::Bytes(_) => "bytes",
            Builtin::Dict(_) => "dict",
            Builtin::Tuple(_) => "tuple",
            Builtin::Type(_) => "type",
            Builtin::Function(_) => "function",
            Builtin::Module(_) => "module",
            Builtin::Code(_) => "code",
            Builtin::DictKey(_) => "dictkey",
        } 
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//     Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


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

    fn set(&self, objref: &ObjectRef) {
        expr_foreach_builtin!(self, obj, {
            obj.rc.set(objref)
        })
    }

    fn get(&self) -> WeakObjectRef {
        expr_foreach_builtin!(self, obj, {
            obj.rc.get()
        })
    }

    fn upgrade(&self) -> RuntimeResult {
        expr_foreach_builtin!(self, obj, {
            obj.rc.upgrade()
        })
    }
}


impl object::PyAPI for Builtin {}
impl method::New for Builtin {}
impl method::Init for Builtin {}
impl method::Delete for Builtin {}
impl method::GetAttr for Builtin {
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        native_foreach_builtin!(self, native_getattr, lhs, name)
    }
}

impl method::GetAttribute for Builtin {}
impl method::SetAttr for Builtin {
    fn op_setattr(&self, rt: &Runtime, name: &ObjectRef, value: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {
        native_foreach_builtin!(self, native_setattr, lhs, name, value)
    }
}
impl method::DelAttr for Builtin {}
impl method::Id for Builtin {
    fn op_id(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> native::ObjectId {
        expr_foreach_builtin!(self, obj, {
            (obj as *const _) as native::ObjectId
        })
    }
}


impl method::Is for Builtin {
    fn op_is(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is(rhs_builtin).unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }


    fn native_is(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_id() == rhs.native_id())
    }
}

impl method::IsNot for Builtin {
    fn op_is_not(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is_not(rhs_builtin).unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_is_not(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_id() != rhs.native_id())
    }
}

impl method::Hashed for Builtin {
    //
    // Hash
    //
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        native_foreach_builtin!(self, native_hash, obj)
    }
}

impl method::StringCast for Builtin {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_str, obj)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        native_foreach_builtin!(self, native_str, obj)
    }
}
impl method::BytesCast for Builtin {
    fn op_bytes(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> NativeResult<native::Bytes> {
        native_foreach_builtin!(self, native_bytes, obj)
    }
}
impl method::StringFormat for Builtin {
    fn op_format(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_format, obj)
    }

    fn native_format(&self) -> NativeResult<native::String> {
        native_foreach_builtin!(self, native_format, obj)
    }
}
impl method::StringRepresentation for Builtin {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_repr, obj)
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        native_foreach_builtin!(self, native_repr, obj)
    }
}
impl method::Equal for Builtin {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_eq, lhs, rhs)
    }
}
impl method::NotEqual for Builtin {
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_ne, lhs, rhs)
    }
}

impl method::LessThan for Builtin {
    fn op_lt(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_lt, lhs, rhs)
    }
}
impl method::LessOrEqual for Builtin {
    fn op_le(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_le, lhs, rhs)
    }
}
impl method::GreaterOrEqual for Builtin {
    fn op_ge(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_ge, lhs, rhs)
    }
}
impl method::GreaterThan for Builtin {
    fn op_gt(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_gt, lhs, rhs)
    }
}
impl method::BooleanCast for Builtin {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_bool, obj)
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_bool, obj)
    }
}

impl method::IntegerCast for Builtin {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_int, obj)
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        native_foreach_builtin!(self, native_int, obj)
    }
}

impl method::FloatCast for Builtin {
    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_float, obj)
    }

    fn native_float(&self) -> NativeResult<native::Float> {
        native_foreach_builtin!(self, native_float, obj)
    }
}

impl method::ComplexCast for Builtin {
    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_complex, obj)
    }

    fn native_complex(&self) -> NativeResult<native::Complex> {
        native_foreach_builtin!(self, native_complex, obj)
    }
}

impl method::Rounding for Builtin {}
impl method::Index for Builtin {
    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_index, obj)
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        native_foreach_builtin!(self, native_index, obj)
    }
}
impl method::NegateValue for Builtin {}
impl method::AbsValue for Builtin {}
impl method::PositiveValue for Builtin {}
impl method::InvertValue for Builtin {}
impl method::Add for Builtin {
    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_add, lhs, rhs)
    }
}
impl method::BitwiseAnd for Builtin {
    fn op_and(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_and, lhs, rhs)
    }
}
impl method::DivMod for Builtin {
    fn op_divmod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_divmod, lhs, rhs)
    }
}
impl method::FloorDivision for Builtin {
    fn op_floordiv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_floordiv, lhs, rhs)
    }
}
impl method::LeftShift for Builtin {
    fn op_lshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_lshift, lhs, rhs)
    }
}
impl method::Modulus for Builtin {
    fn op_mod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_mod, lhs, rhs)
    }
}
impl method::Multiply for Builtin {
    fn op_mul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_mul, lhs, rhs)
    }
}
impl method::MatrixMultiply for Builtin {
    fn op_matmul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_matmul, lhs, rhs)
    }
}
impl method::BitwiseOr for Builtin {
    fn op_or(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_or, lhs, rhs)
    }
}
impl method::Pow for Builtin {
    fn op_pow(&self, rt: &Runtime, power: &ObjectRef, modulus: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_pow, base, power, modulus)
    }
}
impl method::RightShift for Builtin {
    fn op_rshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_rshift, lhs, rhs)
    }
}
impl method::Subtract for Builtin {
    fn op_sub(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_sub, lhs, rhs)
    }
}
impl method::TrueDivision for Builtin {
    fn op_truediv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_truediv, lhs, rhs)
    }
}
impl method::XOr for Builtin {
    fn op_xor(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_xor, lhs, rhs)
    }
}
impl method::ReflectedAdd for Builtin {}
impl method::ReflectedBitwiseAnd for Builtin {}
impl method::ReflectedDivMod for Builtin {}
impl method::ReflectedFloorDivision for Builtin {}
impl method::ReflectedLeftShift for Builtin {}
impl method::ReflectedModulus for Builtin {}
impl method::ReflectedMultiply for Builtin {}
impl method::ReflectedMatrixMultiply for Builtin {}
impl method::ReflectedBitwiseOr for Builtin {}
impl method::ReflectedPow for Builtin {}
impl method::ReflectedRightShift for Builtin {}
impl method::ReflectedSubtract for Builtin {}
impl method::ReflectedTrueDivision for Builtin {}
impl method::ReflectedXOr for Builtin {}
impl method::InPlaceAdd for Builtin {
    fn op_iadd(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_iadd, lhs, rhs)
    }
}
impl method::InPlaceBitwiseAnd for Builtin {
    fn op_iand(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_iand, lhs, rhs)
    }
}
impl method::InPlaceDivMod for Builtin {
    fn op_idivmod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_idivmod, lhs, rhs)
    }
}
impl method::InPlaceFloorDivision for Builtin {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_ifloordiv, lhs, rhs)
    }
}
impl method::InPlaceLeftShift for Builtin {
    fn op_ilshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_ilshift, lhs, rhs)
    }
}
impl method::InPlaceModulus for Builtin {
    fn op_imod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_imod, lhs, rhs)
    }
}
impl method::InPlaceMultiply for Builtin {
    fn op_imul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_imul, lhs, rhs)
    }
}
impl method::InPlaceMatrixMultiply for Builtin {
    fn op_imatmul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_imatmul, lhs, rhs)
    }
}
impl method::InPlaceBitwiseOr for Builtin {
    fn op_ior(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_ior, lhs, rhs)
    }
}
impl method::InPlacePow for Builtin {
    fn op_ipow(&self, rt: &Runtime, power: &ObjectRef, modulus: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_ipow, base, power, modulus)
    }
}
impl method::InPlaceRightShift for Builtin {
    fn op_irshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_irshift, lhs, rhs)
    }
}
impl method::InPlaceSubtract for Builtin {
    fn op_isub(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_isub, lhs, rhs)
    }
}
impl method::InPlaceTrueDivision for Builtin {
    fn op_itruediv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_itruediv, lhs, rhs)
    }
}
impl method::InPlaceXOr for Builtin {
    fn op_ixor(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_ixor, lhs, rhs)
    }
}
impl method::Contains for Builtin {
    fn op_contains(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_contains, lhs, rhs)
    }
}
impl method::Iter for Builtin {}
impl method::Call for Builtin {
    fn op_call(&self, rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Builtin, starargs: &Builtin, kwargs: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_call, method, pos_args, starargs, kwargs)
    }
}
impl method::Length for Builtin {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_len, lhs)
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        native_foreach_builtin!(self, native_len, lhs)
    }
}
impl method::LengthHint for Builtin {}
impl method::Next for Builtin {
    fn op_next(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_next, lhs)
    }

    fn native_next(&self) -> NativeResult<ObjectRef> {
        native_foreach_builtin!(self, native_next, lhs)
    }    
}
impl method::Reversed for Builtin {}
impl method::GetItem for Builtin {
    fn op_getitem(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_getitem, object, name)
    }

    fn native_getitem(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        native_foreach_builtin!(self, native_getitem, object, name)
    }
}

impl method::SetItem for Builtin {
    fn op_setitem(&self, rt: &Runtime, name: &ObjectRef, item: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Builtin, item: &Builtin) -> NativeResult<native::None> {
        native_foreach_builtin!(self, native_setitem, object, name, item)
    }
}

impl method::DeleteItem for Builtin {
    fn op_delitem(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_delitem, object, name)
    }
}

impl method::Count for Builtin {
    fn meth_count(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Builtin) -> NativeResult<native::Integer> {
        native_foreach_builtin!(self, native_meth_count, object, name)
    }
}

impl method::Append for Builtin {
    fn meth_append(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Builtin) -> NativeResult<native::None> {
        native_foreach_builtin!(self, native_meth_append, object, name)
    }
}

impl method::Extend for Builtin {
    fn meth_extend(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Builtin) -> NativeResult<native::None> {
        native_foreach_builtin!(self, native_meth_extend, object, name)
    }
}

impl method::Pop for Builtin {
    fn meth_pop(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_meth_pop, object, name)
    }
}

impl method::Remove for Builtin {
    fn meth_remove(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_meth_remove, object, name)
    }
}

impl method::IsDisjoint for Builtin {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_meth_isdisjoint, object, name)
    }
}

impl method::AddItem for Builtin {
    fn meth_add(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_meth_add, object, name)
    }
}

impl method::Discard for Builtin {}
impl method::Clear for Builtin {}
impl method::Get for Builtin {}
impl method::Keys for Builtin {}
impl method::Values for Builtin {}
impl method::Items for Builtin {}
impl method::PopItem for Builtin {}
impl method::Update for Builtin {}
impl method::SetDefault for Builtin {}
impl method::Await for Builtin {}
impl method::Send for Builtin {}
impl method::Throw for Builtin {}
impl method::Close for Builtin {}
impl method::Exit for Builtin {}
impl method::Enter for Builtin {}
impl method::DescriptorGet for Builtin {}
impl method::DescriptorSet for Builtin {}
impl method::DescriptorSetName for Builtin {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//     stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl std::cmp::PartialEq for Builtin {
    fn eq(&self, rhs: &Builtin) -> bool {
        self.native_eq(rhs).unwrap()
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
    type Item = ObjectRef;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Builtin::Iter(ref mut iterator) => {
                iterator.next()
            }
            _ => None
        }
    }
}

#[cfg(all(feature="old", test))]
mod impl_pybehavior {
    api_test_stub!(unary, self, __del__, Delete, op_del, native_del);
    api_test_stub!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
    api_test_stub!(unary, self, __str__, ToString, op_str, native_str);

    /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
    api_test_stub!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
    api_test_stub!(binary, self, __format__, Format, op_format, native_format);


    /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
    api_test_stub!(binary, self, __lt__, LessThan, op_lt, native_lt);
    api_test_stub!(binary, self, __le__, LessOrEqual, op_le, native_le);
    api_test_stub!(binary, self, __eq__, Equal, op_eq, native_eq, native::Boolean);
    api_test_stub!(binary, self, __ne__, NotEqual, op_ne, native_ne, native::Boolean);
    api_test_stub!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
    api_test_stub!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

    // Called by built-in function hash() and for operations on members of hashed collections including
    // set, frozenset, and dict. __hash__() should return an integer. The only required property is
    // that objects which compare equal have the same hash value; it is advised to mix together
    // the hash values of the components of the object that also play a part in comparison
    // of objects by packing them into a tuple and hashing the tuple. Example:
    api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);

    // Identity operators
    api_test_stub!(unary, self, identity, Identity, identity, native_identity, native::Boolean);
    api_test_stub!(unary, self, __bool__, Truth, op_bool, native_bool, native::Boolean);
    api_test_stub!(unary, self, __not__, Not, op_not, native_not, native::Boolean);
    api_test_stub!(binary, self, is_, Is, op_is, native_is, native::Boolean);
    api_test_stub!(binary, self, is_not, IsNot, op_is_not, native_is_not, native::Boolean);

    // 3.3.6. Emulating container types
    api_test_stub!(unary, self, __len__, Length, op_len, native_len);
    api_test_stub!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint);
    api_test_stub!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
    api_test_stub!(binary, self, __missing__, MissingItem, op_missing, native_missing);
    api_test_stub!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem);
    api_test_stub!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
    api_test_stub!(unary, self, __iter__, Iterator, op_iter, native_iter);
    api_test_stub!(unary, self, __reversed__, Reverse, op_reverse, native_reverse);
    api_test_stub!(binary, self, __contains__, Contains, op_contains, native_contains);

    // 3.3.7. Emulating numeric types
    //
    // The following methods can be defined to emulate numeric objects. Methods corresponding to
    // operations that are not supported by the particular kind of number implemented
    // (e.g., bitwise operations for non-integral numbers) should be left undefined.
    api_test_stub!(binary, self, __add__, Add, op_add, native_add);
    api_test_stub!(binary, self, __and__, And, op_and, native_and);
    api_test_stub!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
    api_test_stub!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
    api_test_stub!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
    api_test_stub!(binary, self, __mod__, Modulus, op_mod, native_mod);
    api_test_stub!(binary, self, __mul__, Multiply, op_mul, native_mul);
    api_test_stub!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);
    api_test_stub!(binary, self, __or__, Or, op_or, native_or);
    api_test_stub!(ternary, self, __pow__, Pow, op_pow, native_pow);
    api_test_stub!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);
    api_test_stub!(binary, self, __sub__, Subtract, op_sub, native_sub);
    api_test_stub!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
    api_test_stub!(binary, self, __xor__, XOr, op_xor, native_xor);

    // Reflected Operators
    api_test_stub!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);
    api_test_stub!(binary, self, __rand__, ReflectedAnd, op_rand, native_rand);
    api_test_stub!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);
    api_test_stub!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);
    api_test_stub!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);
    api_test_stub!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);
    api_test_stub!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);
    api_test_stub!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);
    api_test_stub!(binary, self, __ror__, ReflectedOr, op_ror, native_ror);
    api_test_stub!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);
    api_test_stub!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);
    api_test_stub!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);
    api_test_stub!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);
    api_test_stub!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

    // In place operators
    api_test_stub!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);
    api_test_stub!(binary, self, __iand__, InPlaceAnd, op_iand, native_iand);
    api_test_stub!(binary, self, __idivmod__, InPlaceDivMod, op_idivmod, native_idivmod);
    api_test_stub!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);
    api_test_stub!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);
    api_test_stub!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);
    api_test_stub!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);
    api_test_stub!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);
    api_test_stub!(binary, self, __ior__, InPlaceOr, op_ior, native_ior);
    api_test_stub!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);
    api_test_stub!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);
    api_test_stub!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);
    api_test_stub!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);
    api_test_stub!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);

    // Standard unary operators
    api_test_stub!(unary, self, __neg__, Negate, op_neg, native_neg);
    api_test_stub!(unary, self, __abs__, Abs, op_abs, native_abs);
    api_test_stub!(unary, self, __pos__, Positive, op_pos, native_pos);
    api_test_stub!(unary, self, __invert__, Invert, op_invert, native_invert);

    // Standard numeric conversions
    api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
    api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
    api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
    api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
    api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);
}
