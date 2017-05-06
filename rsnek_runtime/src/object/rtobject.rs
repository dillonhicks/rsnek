/// Wrapper around the reference counted pointed to all
/// runtime objects. In CPython, the refcount is as a field in the
/// PyObject struct. Due to the design of rust, all access to the underlying
/// structs must be proxied through the rc for ownership and lifetime analysis.
use std;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use num::Zero;
use serde::ser::{Serialize, Serializer};

use ::error::{Error, ErrorType};
use ::object::method::{self, Id, Next, StringCast, StringRepresentation, Equal};
use ::object;
use ::result::{NativeResult, RuntimeResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider, BooleanProvider};
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, ObjectId};


pub struct RtObject(pub native::RuntimeRef);


impl RtObject {
    #[inline]
    pub fn new(value: Builtin) -> RtObject {
        RtObject(Rc::new(Box::new(value)))
    }

    /// Downgrade the RtObject to a WeakRtObject
    pub fn downgrade(&self) -> WeakRtObject {
        WeakRtObject(Rc::downgrade(&self.0))
    }

    pub fn strong_count(&self) -> native::Integer {
        native::Integer::from(Rc::strong_count(&self.0))
    }

    pub fn weak_count(&self) -> native::Integer {
        native::Integer::from(Rc::weak_count(&self.0))
    }

    pub fn id(&self) -> ObjectId {
        let boxed: &Box<Builtin> = self.0.borrow();
        boxed.native_id()
    }

    pub fn debug_name(&self) -> &str {
        let boxed: &Box<Builtin> = self.0.borrow();
        boxed.debug_name()
    }

    pub fn to_string(&self) -> native::String {
        let boxed: &Box<Builtin> = self.0.borrow();
        match boxed.native_str() {
            Ok(string) => string,
            Err(_) => format!("{}", self)
        }
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
        let lhs_box: &Box<Builtin> = self.0.borrow();

        let rhs_box: &Box<Builtin> = rhs.0.borrow();
        *lhs_box.deref() == *rhs_box.deref()
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
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl std::fmt::Debug for RtObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl AsRef<Builtin> for RtObject {
    fn as_ref(&self) -> &Builtin {
        let boxed: &Box<Builtin> = self.0.borrow();
        boxed.deref()
    }
}

/// While it is cool to be able to directly iterate over an objectref
/// it is impractical and likely impossible to debug if the critical
/// case is hit.
impl Iterator for RtObject {
    type Item = RtObject;

    fn next(&mut self) -> Option<Self::Item> {
        let boxed: &Box<Builtin> = self.0.borrow();
        match boxed.deref() {
            &Builtin::Iter(ref iterator) => {
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


impl object::PyAPI for RtObject {}

impl method::GetAttr for RtObject {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<RtObject> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_getattr, lhs, name)
    }
}


impl method::SetAttr for RtObject {
    fn op_setattr(&self, rt: &Runtime, name: &RtObject, value: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_setattr, lhs, name, value)
    }
}


impl method::Id for RtObject {
    fn op_id(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> native::ObjectId {
        let builtin: &Box<Builtin> = self.0.borrow();
        expr_foreach_builtin!(builtin.deref(), obj, {
            (obj as *const _) as native::ObjectId
        })
    }
}


impl method::Is for RtObject {
    fn op_is(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
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

impl method::IsNot for RtObject {
    fn op_is_not(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
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


impl method::Hashed for RtObject {

    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_hash, obj)
    }
}


impl method::StringCast for RtObject {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_str, obj)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_str, obj)
    }
}


impl method::BytesCast for RtObject {
    fn op_bytes(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> NativeResult<native::Bytes> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_bytes, obj)
    }
}


impl method::StringFormat for RtObject {
    fn op_format(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_format, obj)
    }

    fn native_format(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_format, obj)
    }
}


impl method::StringRepresentation for RtObject {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_repr, obj)
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_repr, obj)
    }
}

impl method::Equal for RtObject {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_eq, lhs, rhs)
    }
}


impl method::NotEqual for RtObject {
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ne, lhs, rhs)
    }
}

impl method::LessThan for RtObject {
    fn op_lt(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_lt, lhs, rhs)
    }
}


impl method::LessOrEqual for RtObject {
    fn op_le(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_le, lhs, rhs)
    }
}


impl method::GreaterOrEqual for RtObject {
    fn op_ge(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ge, lhs, rhs)
    }
}


impl method::GreaterThan for RtObject {
    fn op_gt(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_gt, lhs, rhs)
    }
}


impl method::BooleanCast for RtObject {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_bool, obj)
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_bool, obj)
    }
}

impl method::IntegerCast for RtObject {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_int, obj)
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_int, obj)
    }
}

impl method::FloatCast for RtObject {
    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_float, obj)
    }

    fn native_float(&self) -> NativeResult<native::Float> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_float, obj)
    }
}

impl method::ComplexCast for RtObject {
    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_complex, obj)
    }

    fn native_complex(&self) -> NativeResult<native::Complex> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_complex, obj)
    }
}


impl method::Index for RtObject {
    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_index, obj)
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_index, obj)
    }
}


impl method::NegateValue for RtObject {
    fn op_neg(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_neg, obj)
    }

    fn native_neg(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_neg, obj)
    }
}


impl method::AbsValue for RtObject {
    fn op_abs(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_abs, obj)
    }

    fn native_abs(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_abs, obj)
    }
}


impl method::PositiveValue for RtObject {
    fn op_pos(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_pos, obj)
    }

    fn native_pos(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_pos, obj)
    }
}


impl method::InvertValue for RtObject {
    fn op_invert(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_invert, obj)
    }

    fn native_invert(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_invert, obj)
    }
}


impl method::Add for RtObject {
    fn op_add(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_add, lhs, rhs)
    }
}


impl method::BitwiseAnd for RtObject {
    fn op_and(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_and, lhs, rhs)
    }
}


impl method::DivMod for RtObject {
    fn op_divmod(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_divmod, lhs, rhs)
    }
}


impl method::FloorDivision for RtObject {
    fn op_floordiv(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_floordiv, lhs, rhs)
    }
}


impl method::LeftShift for RtObject {
    fn op_lshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_lshift, lhs, rhs)
    }
}


impl method::Modulus for RtObject {
    fn op_mod(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_mod, lhs, rhs)
    }
}


impl method::Multiply for RtObject {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Builtin) -> NativeResult<native::Native> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_mul, lhs, rhs)
    }
}


impl method::MatrixMultiply for RtObject {
    fn op_matmul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_matmul, lhs, rhs)
    }
}


impl method::BitwiseOr for RtObject {
    fn op_or(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_or, lhs, rhs)
    }
}


impl method::Pow for RtObject {
    fn op_pow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_pow, base, power, modulus)
    }
}


impl method::RightShift for RtObject {
    fn op_rshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_rshift, lhs, rhs)
    }
}


impl method::Subtract for RtObject {
    fn op_sub(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_sub, lhs, rhs)
    }
}


impl method::TrueDivision for RtObject {
    fn op_truediv(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_truediv, lhs, rhs)
    }
}


impl method::XOr for RtObject {
    fn op_xor(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_xor, lhs, rhs)
    }
}


impl method::InPlaceAdd for RtObject {
    fn op_iadd(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iadd, lhs, rhs)
    }
}


impl method::InPlaceBitwiseAnd for RtObject {
    fn op_iand(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iand, lhs, rhs)
    }
}


impl method::InPlaceDivMod for RtObject {
    fn op_idivmod(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_idivmod, lhs, rhs)
    }
}


impl method::InPlaceFloorDivision for RtObject {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ifloordiv, lhs, rhs)
    }
}


impl method::InPlaceLeftShift for RtObject {
    fn op_ilshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ilshift, lhs, rhs)
    }
}


impl method::InPlaceModulus for RtObject {
    fn op_imod(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imod, lhs, rhs)
    }
}


impl method::InPlaceMultiply for RtObject {
    fn op_imul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imul, lhs, rhs)
    }
}


impl method::InPlaceMatrixMultiply for RtObject {
    fn op_imatmul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imatmul, lhs, rhs)
    }
}


impl method::InPlaceBitwiseOr for RtObject {
    fn op_ior(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ior, lhs, rhs)
    }
}


impl method::InPlacePow for RtObject {
    fn op_ipow(&self, rt: &Runtime, power: &RtObject, modulus: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ipow, base, power, modulus)
    }
}


impl method::InPlaceRightShift for RtObject {
    fn op_irshift(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_irshift, lhs, rhs)
    }
}


impl method::InPlaceSubtract for RtObject {
    fn op_isub(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_isub, lhs, rhs)
    }
}


impl method::InPlaceTrueDivision for RtObject {
    fn op_itruediv(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_itruediv, lhs, rhs)
    }
}


impl method::InPlaceXOr for RtObject {
    fn op_ixor(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ixor, lhs, rhs)
    }
}


impl method::Contains for RtObject {
    fn op_contains(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_contains, lhs, rhs)
    }
}


impl method::Iter for RtObject {
    fn op_iter(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iter, lhs)
    }

    fn native_iter(&self) -> NativeResult<native::Iterator> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iter, lhs)
    }
}


impl method::Call for RtObject {
    fn op_call(&self, rt: &Runtime, pos_args: &RtObject, starargs: &RtObject, kwargs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Builtin, starargs: &Builtin, kwargs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_call, method, pos_args, starargs, kwargs)
    }
}


impl method::Length for RtObject {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_len, lhs)
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_len, lhs)
    }
}


impl method::Next for RtObject {
    fn op_next(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_next, lhs)
    }

    fn native_next(&self) -> NativeResult<RtObject> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_next, lhs)
    }
}


impl method::GetItem for RtObject {
    fn op_getitem(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_getitem, object, name)
    }

    fn native_getitem(&self, name: &Builtin) -> NativeResult<RtObject> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_getitem, object, name)
    }
}

impl method::SetItem for RtObject {
    fn op_setitem(&self, rt: &Runtime, name: &RtObject, item: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Builtin, item: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_setitem, object, name, item)
    }
}

impl method::DeleteItem for RtObject {
    fn op_delitem(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_delitem, object, name)
    }
}

impl method::Count for RtObject {
    fn meth_count(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Builtin) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_count, object, name)
    }
}


impl method::Append for RtObject {
    fn meth_append(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_append, object, name)
    }
}


impl method::Extend for RtObject {
    fn meth_extend(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_extend, object, name)
    }
}


impl method::Pop for RtObject {
    fn meth_pop(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_pop, object, name)
    }
}


impl method::Remove for RtObject {
    fn meth_remove(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_remove, object, name)
    }
}


impl method::IsDisjoint for RtObject {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_isdisjoint, object, name)
    }
}


impl method::AddItem for RtObject {
    fn meth_add(&self, rt: &Runtime, name: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_add, object, name)
    }
}


impl method::Keys for RtObject {
    fn meth_keys(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_keys, object)
    }

    fn native_meth_keys(&self) -> NativeResult<native::Tuple> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_keys, object)
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

pub struct WeakRtObject(pub native::RuntimeWeakRef);


impl Default for WeakRtObject {
    fn default() -> WeakRtObject {
        WeakRtObject(Weak::default())
    }
}


impl WeakRtObject {
    pub fn weak_count(&self) -> native::Integer {
        let count: native::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return native::Integer::zero(),
            };

            count = objref.weak_count();
            drop(objref)
        }

        count
    }

    pub fn strong_count(&self) -> native::Integer {
        let count: native::Integer;
        {
            let objref = match self.upgrade() {
                Ok(strong) => strong,
                Err(_) => return native::Integer::zero(),
            };

            count = objref.strong_count();
            drop(objref)
        }

        count
    }

    pub fn upgrade(&self) -> RuntimeResult {
        match Weak::upgrade(&self.0) {
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
    use ::object::method::Add;
    use ::traits::IntegerProvider;
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