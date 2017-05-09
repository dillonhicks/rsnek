//! Wrapper around the reference counted pointed to all
//! runtime objects. In CPython, the StrongRc is as a field in the
//! PyObject struct. Due to the design of rust, all access to the underlying
//! structs must be proxied through the rc for ownership and lifetime analysis.
//!
use std;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use num::Zero;
use serde::ser::{Serialize, Serializer};

use ::api::result::{Error, ErrorType, ObjectResult, RtResult};
use ::api::method::{self, Id, Next, StringCast, StringRepresentation, Equal};
use ::api;
use ::runtime::Runtime;
use ::system::{StrongRc, WeakRc};
use ::runtime::traits::{IntegerProvider, BooleanProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Native, ObjectId};
use ::system::primitives as rs;

type RuntimeRef = StrongRc<Type>;
type RuntimeWeakRef = WeakRc<Type>;


/// The wrapper and interface around any rust native structure
/// that wants to be represented as a Python object in the runtime.
///
pub struct RtObject(RuntimeRef);


impl RtObject {
    #[inline]
    pub fn new(value: Type) -> RtObject {
        RtObject(StrongRc::new(value))
    }

    /// Downgrade the RtObject to a WeakRtObject
    pub fn downgrade(&self) -> WeakRtObject {
        WeakRtObject(StrongRc::downgrade(&self.0))
    }

    pub fn strong_count(&self) -> rs::Integer {
        rs::Integer::from(StrongRc::strong_count(&self.0))
    }

    pub fn weak_count(&self) -> rs::Integer {
        rs::Integer::from(StrongRc::weak_count(&self.0))
    }

    pub fn id(&self) -> ObjectId {
        self.native_id()
    }

    pub fn debug_name(&self) -> &str {
        self.as_ref().debug_name()
    }

    pub fn to_string(&self) -> rs::String {
        self.native_str().unwrap_or(format!("{}", self))
    }
}


impl Serialize for RtObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {

        serializer.serialize_str(&self.to_string())
    }
}


impl std::cmp::PartialEq for RtObject {
    fn eq(&self, rhs: &RtObject) -> bool {
        *self.as_ref() == *rhs.as_ref()
    }
}


impl std::cmp::Eq for RtObject {}


impl Clone for RtObject {
    fn clone(&self) -> Self {
        RtObject((self.0).clone())
    }
}

impl Hash for RtObject {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, s: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


impl std::fmt::Display for RtObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let builtin = self.as_ref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl std::fmt::Debug for RtObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let builtin = self.as_ref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl AsRef<Type> for RtObject {
    #[inline]
    fn as_ref(&self) -> &Type {
        self.0.borrow()
    }
}

/// While it is cool to be able to directly iterate over an `RtObject`
/// it is impractical and likely impossible to debug if the critical
/// case is hit.
impl Iterator for RtObject {
    type Item = RtObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self.as_ref() {
            &Type::Iter(ref iterator) => {
                match iterator.native_next() {
                    Ok(objref) => Some(objref),
                    Err(Error(ErrorType::StopIteration, _)) => None,
                    Err(err) => {
                        crit!("Iterator logic fault"; "cause" => format!("{:?}", err));
                        None
                    }
                }
            }
            _ => None
        }
    }

}


impl api::PyAPI for RtObject {}

impl method::GetAttr for RtObject {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Type) -> ObjectResult {
        native_foreach_type!(self.as_ref(), native_getattr, lhs, name)
    }
}


impl method::SetAttr for RtObject {
    fn op_setattr(&self, rt: &Runtime, name: &RtObject, value: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Type, value: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self.as_ref(), native_setattr, lhs, name, value)
    }
}


impl method::Id for RtObject {
    fn op_id(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> rs::ObjectId {
        expr_foreach_type!(self.as_ref(), obj, {
            (obj as *const _) as rs::ObjectId
        })
    }
}



impl method::Is for RtObject {
    fn op_is(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() == rhs.native_id())
    }
}

impl method::IsNot for RtObject {
    fn op_is_not(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is_not(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is_not(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() != rhs.native_id())
    }
}


impl method::Hashed for RtObject {

    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_hash, obj)
    }

    fn native_hash(&self) -> RtResult<rs::HashId> {
        native_foreach_type!(self.as_ref(), native_hash, obj)
    }
}


impl method::StringCast for RtObject {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_str, obj)
    }

    fn native_str(&self) -> RtResult<rs::String> {
        native_foreach_type!(self.as_ref(), native_str, obj)
    }
}


impl method::BytesCast for RtObject {
    fn op_bytes(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> RtResult<rs::Bytes> {
        native_foreach_type!(self.as_ref(), native_bytes, obj)
    }
}


impl method::StringFormat for RtObject {
    fn op_format(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_format, obj)
    }

    fn native_format(&self) -> RtResult<rs::String> {
        native_foreach_type!(self.as_ref(), native_format, obj)
    }
}


impl method::StringRepresentation for RtObject {
    fn op_repr(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_repr, obj)
    }

    fn native_repr(&self) -> RtResult<rs::String> {
        native_foreach_type!(self.as_ref(), native_repr, obj)
    }
}

impl method::Equal for RtObject {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_eq, lhs, rhs)
    }
}


impl method::NotEqual for RtObject {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_ne, lhs, rhs)
    }
}

impl method::LessThan for RtObject {
    fn op_lt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_lt, lhs, rhs)
    }
}


impl method::LessOrEqual for RtObject {
    fn op_le(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_le, lhs, rhs)
    }
}


impl method::GreaterOrEqual for RtObject {
    fn op_ge(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_ge, lhs, rhs)
    }
}


impl method::GreaterThan for RtObject {
    fn op_gt(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_gt, lhs, rhs)
    }
}


impl method::BooleanCast for RtObject {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_bool, obj)
    }

    fn native_bool(&self) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_bool, obj)
    }
}

impl method::IntegerCast for RtObject {
    fn op_int(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_int, obj)
    }

    fn native_int(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self.as_ref(), native_int, obj)
    }
}

impl method::FloatCast for RtObject {
    fn op_float(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_float, obj)
    }

    fn native_float(&self) -> RtResult<rs::Float> {
        native_foreach_type!(self.as_ref(), native_float, obj)
    }
}

impl method::ComplexCast for RtObject {
    fn op_complex(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_complex, obj)
    }

    fn native_complex(&self) -> RtResult<rs::Complex> {
        native_foreach_type!(self.as_ref(), native_complex, obj)
    }
}


impl method::Index for RtObject {
    fn op_index(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_index, obj)
    }

    fn native_index(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self.as_ref(), native_index, obj)
    }
}


impl method::NegateValue for RtObject {
    fn op_neg(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_neg, obj)
    }

    fn native_neg(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self.as_ref(), native_neg, obj)
    }
}


impl method::AbsValue for RtObject {
    fn op_abs(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_abs, obj)
    }

    fn native_abs(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self.as_ref(), native_abs, obj)
    }
}


impl method::PositiveValue for RtObject {
    fn op_pos(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_pos, obj)
    }

    fn native_pos(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self.as_ref(), native_pos, obj)
    }
}


impl method::InvertValue for RtObject {
    fn op_invert(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_invert, obj)
    }

    fn native_invert(&self) -> RtResult<rs::Number> {
        native_foreach_type!(self.as_ref(), native_invert, obj)
    }
}


impl method::Add for RtObject {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Type) -> RtResult<Native> {
        native_foreach_type!(self.as_ref(), native_add, lhs, rhs)
    }
}


impl method::BitwiseAnd for RtObject {
    fn op_and(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_and, lhs, rhs)
    }
}


impl method::DivMod for RtObject {
    fn op_divmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_divmod, lhs, rhs)
    }
}


impl method::FloorDivision for RtObject {
    fn op_floordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_floordiv, lhs, rhs)
    }
}


impl method::LeftShift for RtObject {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_lshift, lhs, rhs)
    }
}


impl method::Modulus for RtObject {
    fn op_mod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_mod, lhs, rhs)
    }
}


impl method::Multiply for RtObject {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Type) -> RtResult<rs::Native> {
        native_foreach_type!(self.as_ref(), native_mul, lhs, rhs)
    }
}


impl method::MatrixMultiply for RtObject {
    fn op_matmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_matmul, lhs, rhs)
    }
}


impl method::BitwiseOr for RtObject {
    fn op_or(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_or, lhs, rhs)
    }
}


impl method::Pow for RtObject {
    fn op_pow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Type, modulus: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_pow, base, power, modulus)
    }
}


impl method::RightShift for RtObject {
    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_rshift, lhs, rhs)
    }
}


impl method::Subtract for RtObject {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_sub, lhs, rhs)
    }
}


impl method::TrueDivision for RtObject {
    fn op_truediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_truediv, lhs, rhs)
    }
}


impl method::XOr for RtObject {
    fn op_xor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_xor, lhs, rhs)
    }
}


impl method::InPlaceAdd for RtObject {
    fn op_iadd(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_iadd, lhs, rhs)
    }
}


impl method::InPlaceBitwiseAnd for RtObject {
    fn op_iand(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_iand, lhs, rhs)
    }
}


impl method::InPlaceDivMod for RtObject {
    fn op_idivmod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_idivmod, lhs, rhs)
    }
}


impl method::InPlaceFloorDivision for RtObject {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_ifloordiv, lhs, rhs)
    }
}


impl method::InPlaceLeftShift for RtObject {
    fn op_ilshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_ilshift, lhs, rhs)
    }
}


impl method::InPlaceModulus for RtObject {
    fn op_imod(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_imod, lhs, rhs)
    }
}


impl method::InPlaceMultiply for RtObject {
    fn op_imul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_imul, lhs, rhs)
    }
}


impl method::InPlaceMatrixMultiply for RtObject {
    fn op_imatmul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_imatmul, lhs, rhs)
    }
}


impl method::InPlaceBitwiseOr for RtObject {
    fn op_ior(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_ior, lhs, rhs)
    }
}


impl method::InPlacePow for RtObject {
    fn op_ipow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Type, modulus: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_ipow, base, power, modulus)
    }
}


impl method::InPlaceRightShift for RtObject {
    fn op_irshift(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_irshift, lhs, rhs)
    }
}


impl method::InPlaceSubtract for RtObject {
    fn op_isub(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_isub, lhs, rhs)
    }
}


impl method::InPlaceTrueDivision for RtObject {
    fn op_itruediv(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_itruediv, lhs, rhs)
    }
}


impl method::InPlaceXOr for RtObject {
    fn op_ixor(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_ixor, lhs, rhs)
    }
}


impl method::Contains for RtObject {
    fn op_contains(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_contains, lhs, rhs)
    }
}


impl method::Iter for RtObject {
    fn op_iter(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_iter, lhs)
    }

    fn native_iter(&self) -> RtResult<rs::Iterator> {
        native_foreach_type!(self.as_ref(), native_iter, lhs)
    }
}


impl method::Call for RtObject {
    fn op_call(&self, rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Type, starargs: &Type, kwargs: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_call, method, pos_args, starargs, kwargs)
    }
}


impl method::Length for RtObject {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_len, lhs)
    }

    fn native_len(&self) -> RtResult<rs::Integer> {
        native_foreach_type!(self.as_ref(), native_len, lhs)
    }
}


impl method::Next for RtObject {
    fn op_next(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_next, lhs)
    }

    fn native_next(&self) -> ObjectResult {
        native_foreach_type!(self.as_ref(), native_next, lhs)
    }
}


impl method::GetItem for RtObject {
    #[inline]
    fn op_getitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_getitem, object, name)
    }

    #[inline(always)]
    fn native_getitem(&self, name: &Type) -> ObjectResult {
        native_foreach_type!(self.as_ref(), native_getitem, object, name)
    }
}

impl method::SetItem for RtObject {
    fn op_setitem(&self, rt: &Runtime, name: &RtObject, item: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Type, item: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self.as_ref(), native_setitem, object, name, item)
    }
}

impl method::DeleteItem for RtObject {
    fn op_delitem(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_delitem, object, name)
    }
}

impl method::Count for RtObject {
    fn meth_count(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Type) -> RtResult<rs::Integer> {
        native_foreach_type!(self.as_ref(), native_meth_count, object, name)
    }
}


impl method::Append for RtObject {
    fn meth_append(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self.as_ref(), native_meth_append, object, name)
    }
}


impl method::Extend for RtObject {
    fn meth_extend(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Type) -> RtResult<rs::None> {
        native_foreach_type!(self.as_ref(), native_meth_extend, object, name)
    }
}


impl method::Pop for RtObject {
    fn meth_pop(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_meth_pop, object, name)
    }
}


impl method::Remove for RtObject {
    fn meth_remove(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_meth_remove, object, name)
    }
}


impl method::IsDisjoint for RtObject {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Type) -> RtResult<rs::Boolean> {
        native_foreach_type!(self.as_ref(), native_meth_isdisjoint, object, name)
    }
}


impl method::AddItem for RtObject {
    fn meth_add(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Type) -> RtResult<Type> {
        native_foreach_type!(self.as_ref(), native_meth_add, object, name)
    }
}


impl method::Keys for RtObject {
    fn meth_keys(&self, rt: &Runtime) -> ObjectResult {
        foreach_type!(self.as_ref(), rt, meth_keys, object)
    }

    fn native_meth_keys(&self) -> RtResult<rs::Tuple> {
        native_foreach_type!(self.as_ref(), native_meth_keys, object)
    }
}


method_not_implemented!(RtObject,
    Await   Clear   Close   DelAttr   Delete
    DescriptorGet   DescriptorSet   DescriptorSetName   Discard   Enter
    Exit   Get   GetAttribute   Init   Items
    LengthHint   New   PopItem   ReflectedAdd   ReflectedBitwiseAnd
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift
    ReflectedMatrixMultiply  ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift   ReflectedSubtract  ReflectedTrueDivision   ReflectedXOr   Reversed
    Rounding   Send   SetDefault   Throw   Update   Values
);


//
// Weak Object References
//

pub struct WeakRtObject(RuntimeWeakRef);


impl Default for WeakRtObject {
    fn default() -> WeakRtObject {
        WeakRtObject(WeakRc::default())
    }
}


impl WeakRtObject {
    pub fn weak_count(&self) -> rs::Integer {
        let count: rs::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return rs::Integer::zero(),
            };

            count = objref.weak_count();
            drop(objref)
        }

        count
    }

    pub fn strong_count(&self) -> rs::Integer {
        let count: rs::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return rs::Integer::zero(),
            };

            count = objref.strong_count();
            drop(objref)
        }

        count
    }

    pub fn upgrade(&self) -> ObjectResult {
        match WeakRc::upgrade(&self.0) {
            None => Err(Error::system(
                &format!("{} {}; file: {} line: {}",
                         "Attempted to create a strong ref to an object with no existing refs, ",
                         "this is a bug!; file: {}, line: {}",
                         file!(), line!()))),
            Some(objref) => Ok(RtObject(objref)),
        }
    }
}


impl Clone for WeakRtObject {
    fn clone(&self) -> Self {
        WeakRtObject((self.0).clone())
    }
}


impl Hash for WeakRtObject {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


method_not_implemented!(WeakRtObject,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close
    ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Equal   Exit   Extend   FloatCast
    FloorDivision   Get   GetAttr   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan   Hashed
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
    use ::api::method::Add;
    use ::runtime::traits::IntegerProvider;
    use ::runtime::Runtime;

    fn setup() -> (Runtime,) {
        (Runtime::new(),)
    }

    #[test]
    fn __add__() {
        let (rt,) = setup();
        let one = rt.int(1);
        let two = one.op_add(&rt, &one).unwrap();

        assert_eq!(two, rt.int(2))
    }
}