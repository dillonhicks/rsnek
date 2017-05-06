/// Wrapper around the reference counted pointed to all
/// runtime objects. In CPython, the refcount is as a field in the
/// PyObject struct. Due to the design of rust, all access to the underlying
/// structs must be proxied through the rc for ownership and lifetime analysis.
use std;
use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};

use num::Zero;
use serde::ser::{Serialize, Serializer};

use ::error::{Error, ErrorType};
use ::object;
use ::object::method::{self, Id, Next, StringCast, StringRepresentation, Equal};
use ::result::{NativeResult, RuntimeResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider, BooleanProvider};
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, ObjectId};


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      Types and Structs
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

pub struct ObjectRef(pub native::RuntimeRef);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//       Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl ObjectRef {
    #[inline]
    pub fn new(value: Builtin) -> ObjectRef {
        ObjectRef(Rc::new(Box::new(value)))
    }

    /// Downgrade the ObjectRef to a WeakObjectRef
    pub fn downgrade(&self) -> WeakObjectRef {
        WeakObjectRef(Rc::downgrade(&self.0))
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


impl Serialize for ObjectRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {

        serializer.serialize_str(&self.to_string())
    }
}


impl std::cmp::PartialEq for ObjectRef {
    fn eq(&self, rhs: &ObjectRef) -> bool {
        let lhs_box: &Box<Builtin> = self.0.borrow();

        let rhs_box: &Box<Builtin> = rhs.0.borrow();
        *lhs_box.deref() == *rhs_box.deref()
    }
}


impl std::cmp::Eq for ObjectRef {}


impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        ObjectRef((self.0).clone())
    }
}

impl Hash for ObjectRef {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, s: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


impl std::fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl std::fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let boxed: &Box<Builtin> = self.0.borrow();
        let builtin = boxed.deref();
        write!(f, "<{:?} {:?}>", builtin, builtin.native_id())
    }
}

impl AsRef<Builtin> for ObjectRef {
    fn as_ref(&self) -> &Builtin {
        let boxed: &Box<Builtin> = self.0.borrow();
        boxed.deref()
    }
}

/// While it is cool to be able to directly iterate over an objectref
/// it is impractical and likely impossible to debug if the critical
/// case is hit.
impl Iterator for ObjectRef {
    type Item = ObjectRef;

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




impl object::PyAPI for ObjectRef {}
impl method::New for ObjectRef {}
impl method::Init for ObjectRef {}
impl method::Delete for ObjectRef {}
impl method::GetAttr for ObjectRef {
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_getattr, lhs, name)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_getattr, lhs, name)
    }
}

impl method::GetAttribute for ObjectRef {}
impl method::SetAttr for ObjectRef {
    fn op_setattr(&self, rt: &Runtime, name: &ObjectRef, value: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_setattr, lhs, name, value)
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_setattr, lhs, name, value)
    }
}
impl method::DelAttr for ObjectRef {}
impl method::Id for ObjectRef {
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


impl method::Is for ObjectRef {
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

impl method::IsNot for ObjectRef {
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

impl method::Hashed for ObjectRef {

    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_hash, obj)
    }
}

impl method::StringCast for ObjectRef {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_str, obj)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_str, obj)
    }
}
impl method::BytesCast for ObjectRef {
    fn op_bytes(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_bytes, obj)
    }

    fn native_bytes(&self) -> NativeResult<native::Bytes> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_bytes, obj)
    }
}
impl method::StringFormat for ObjectRef {
    fn op_format(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_format, obj)
    }

    fn native_format(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_format, obj)
    }
}
impl method::StringRepresentation for ObjectRef {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_repr, obj)
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_repr, obj)
    }
}

impl method::Equal for ObjectRef {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_eq, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_eq, lhs, rhs)
    }
}
impl method::NotEqual for ObjectRef {
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ne, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ne, lhs, rhs)
    }
}

impl method::LessThan for ObjectRef {
    fn op_lt(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_lt, lhs, rhs)
    }

    fn native_lt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_lt, lhs, rhs)
    }
}
impl method::LessOrEqual for ObjectRef {
    fn op_le(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_le, lhs, rhs)
    }

    fn native_le(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_le, lhs, rhs)
    }
}
impl method::GreaterOrEqual for ObjectRef {
    fn op_ge(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ge, lhs, rhs)
    }

    fn native_ge(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ge, lhs, rhs)
    }
}
impl method::GreaterThan for ObjectRef {
    fn op_gt(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_gt, lhs, rhs)
    }

    fn native_gt(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_gt, lhs, rhs)
    }
}
impl method::BooleanCast for ObjectRef {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_bool, obj)
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_bool, obj)
    }
}

impl method::IntegerCast for ObjectRef {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_int, obj)
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_int, obj)
    }
}

impl method::FloatCast for ObjectRef {
    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_float, obj)
    }

    fn native_float(&self) -> NativeResult<native::Float> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_float, obj)
    }
}

impl method::ComplexCast for ObjectRef {
    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_complex, obj)
    }

    fn native_complex(&self) -> NativeResult<native::Complex> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_complex, obj)
    }
}

impl method::Rounding for ObjectRef {}
impl method::Index for ObjectRef {
    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_index, obj)
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_index, obj)
    }
}
impl method::NegateValue for ObjectRef {
    fn op_neg(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_neg, obj)
    }

    fn native_neg(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_neg, obj)
    }
}
impl method::AbsValue for ObjectRef {
    fn op_abs(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_abs, obj)
    }

    fn native_abs(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_abs, obj)
    }
}
impl method::PositiveValue for ObjectRef {
    fn op_pos(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_pos, obj)
    }

    fn native_pos(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_pos, obj)
    }
}
impl method::InvertValue for ObjectRef {
    fn op_invert(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_invert, obj)
    }

    fn native_invert(&self) -> NativeResult<native::Number> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_invert, obj)
    }
}
impl method::Add for ObjectRef {
    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_add, lhs, rhs)
    }
}
impl method::BitwiseAnd for ObjectRef {
    fn op_and(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_and, lhs, rhs)
    }

    fn native_and(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_and, lhs, rhs)
    }
}
impl method::DivMod for ObjectRef {
    fn op_divmod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_divmod, lhs, rhs)
    }

    fn native_divmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_divmod, lhs, rhs)
    }
}
impl method::FloorDivision for ObjectRef {
    fn op_floordiv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_floordiv, lhs, rhs)
    }

    fn native_floordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_floordiv, lhs, rhs)
    }
}
impl method::LeftShift for ObjectRef {
    fn op_lshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_lshift, lhs, rhs)
    }

    fn native_lshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_lshift, lhs, rhs)
    }
}
impl method::Modulus for ObjectRef {
    fn op_mod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_mod, lhs, rhs)
    }

    fn native_mod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_mod, lhs, rhs)
    }
}
impl method::Multiply for ObjectRef {
    fn op_mul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_mul, lhs, rhs)
    }

    fn native_mul(&self, rhs: &Builtin) -> NativeResult<native::Native> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_mul, lhs, rhs)
    }
}
impl method::MatrixMultiply for ObjectRef {
    fn op_matmul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_matmul, lhs, rhs)
    }

    fn native_matmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_matmul, lhs, rhs)
    }
}
impl method::BitwiseOr for ObjectRef {
    fn op_or(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_or, lhs, rhs)
    }

    fn native_or(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_or, lhs, rhs)
    }
}
impl method::Pow for ObjectRef {
    fn op_pow(&self, rt: &Runtime, power: &ObjectRef, modulus: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_pow, base, power, modulus)
    }

    fn native_pow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_pow, base, power, modulus)
    }
}
impl method::RightShift for ObjectRef {
    fn op_rshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_rshift, lhs, rhs)
    }

    fn native_rshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_rshift, lhs, rhs)
    }
}
impl method::Subtract for ObjectRef {
    fn op_sub(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_sub, lhs, rhs)
    }

    fn native_sub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_sub, lhs, rhs)
    }
}
impl method::TrueDivision for ObjectRef {
    fn op_truediv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_truediv, lhs, rhs)
    }

    fn native_truediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_truediv, lhs, rhs)
    }
}
impl method::XOr for ObjectRef {
    fn op_xor(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_xor, lhs, rhs)
    }

    fn native_xor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_xor, lhs, rhs)
    }
}
impl method::ReflectedAdd for ObjectRef {}
impl method::ReflectedBitwiseAnd for ObjectRef {}
impl method::ReflectedDivMod for ObjectRef {}
impl method::ReflectedFloorDivision for ObjectRef {}
impl method::ReflectedLeftShift for ObjectRef {}
impl method::ReflectedModulus for ObjectRef {}
impl method::ReflectedMultiply for ObjectRef {}
impl method::ReflectedMatrixMultiply for ObjectRef {}
impl method::ReflectedBitwiseOr for ObjectRef {}
impl method::ReflectedPow for ObjectRef {}
impl method::ReflectedRightShift for ObjectRef {}
impl method::ReflectedSubtract for ObjectRef {}
impl method::ReflectedTrueDivision for ObjectRef {}
impl method::ReflectedXOr for ObjectRef {}
impl method::InPlaceAdd for ObjectRef {
    fn op_iadd(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iadd, lhs, rhs)
    }

    fn native_iadd(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iadd, lhs, rhs)
    }
}
impl method::InPlaceBitwiseAnd for ObjectRef {
    fn op_iand(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iand, lhs, rhs)
    }

    fn native_iand(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iand, lhs, rhs)
    }
}
impl method::InPlaceDivMod for ObjectRef {
    fn op_idivmod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_idivmod, lhs, rhs)
    }

    fn native_idivmod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_idivmod, lhs, rhs)
    }
}
impl method::InPlaceFloorDivision for ObjectRef {
    fn op_ifloordiv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ifloordiv, lhs, rhs)
    }

    fn native_ifloordiv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ifloordiv, lhs, rhs)
    }
}
impl method::InPlaceLeftShift for ObjectRef {
    fn op_ilshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ilshift, lhs, rhs)
    }

    fn native_ilshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ilshift, lhs, rhs)
    }
}
impl method::InPlaceModulus for ObjectRef {
    fn op_imod(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imod, lhs, rhs)
    }

    fn native_imod(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imod, lhs, rhs)
    }
}
impl method::InPlaceMultiply for ObjectRef {
    fn op_imul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imul, lhs, rhs)
    }

    fn native_imul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imul, lhs, rhs)
    }
}
impl method::InPlaceMatrixMultiply for ObjectRef {
    fn op_imatmul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_imatmul, lhs, rhs)
    }

    fn native_imatmul(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_imatmul, lhs, rhs)
    }
}
impl method::InPlaceBitwiseOr for ObjectRef {
    fn op_ior(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ior, lhs, rhs)
    }

    fn native_ior(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ior, lhs, rhs)
    }
}
impl method::InPlacePow for ObjectRef {
    fn op_ipow(&self, rt: &Runtime, power: &ObjectRef, modulus: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ipow, base, power, modulus)
    }

    fn native_ipow(&self, power: &Builtin, modulus: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ipow, base, power, modulus)
    }
}
impl method::InPlaceRightShift for ObjectRef {
    fn op_irshift(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_irshift, lhs, rhs)
    }

    fn native_irshift(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_irshift, lhs, rhs)
    }
}
impl method::InPlaceSubtract for ObjectRef {
    fn op_isub(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_isub, lhs, rhs)
    }

    fn native_isub(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_isub, lhs, rhs)
    }
}
impl method::InPlaceTrueDivision for ObjectRef {
    fn op_itruediv(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_itruediv, lhs, rhs)
    }

    fn native_itruediv(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_itruediv, lhs, rhs)
    }
}
impl method::InPlaceXOr for ObjectRef {
    fn op_ixor(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_ixor, lhs, rhs)
    }

    fn native_ixor(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_ixor, lhs, rhs)
    }
}
impl method::Contains for ObjectRef {
    fn op_contains(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_contains, lhs, rhs)
    }

    fn native_contains(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_contains, lhs, rhs)
    }
}
impl method::Iter for ObjectRef {
    fn op_iter(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_iter, lhs)
    }

    fn native_iter(&self) -> NativeResult<native::Iterator> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_iter, lhs)
    }
}
impl method::Call for ObjectRef {
    fn op_call(&self, rt: &Runtime, pos_args: &ObjectRef, starargs: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_call, method, pos_args, starargs, kwargs)
    }

    fn native_call(&self, pos_args: &Builtin, starargs: &Builtin, kwargs: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_call, method, pos_args, starargs, kwargs)
    }
}
impl method::Length for ObjectRef {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_len, lhs)
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_len, lhs)
    }
}
impl method::LengthHint for ObjectRef {}
impl method::Next for ObjectRef {
    fn op_next(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_next, lhs)
    }

    fn native_next(&self) -> NativeResult<ObjectRef> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_next, lhs)
    }
}
impl method::Reversed for ObjectRef {}
impl method::GetItem for ObjectRef {
    fn op_getitem(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_getitem, object, name)
    }

    fn native_getitem(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_getitem, object, name)
    }
}

impl method::SetItem for ObjectRef {
    fn op_setitem(&self, rt: &Runtime, name: &ObjectRef, item: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Builtin, item: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_setitem, object, name, item)
    }
}

impl method::DeleteItem for ObjectRef {
    fn op_delitem(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, op_delitem, object, name)
    }

    fn native_delitem(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_delitem, object, name)
    }
}

impl method::Count for ObjectRef {
    fn meth_count(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_count, object, name)
    }

    fn native_meth_count(&self, name: &Builtin) -> NativeResult<native::Integer> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_count, object, name)
    }
}

impl method::Append for ObjectRef {
    fn meth_append(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_append, object, name)
    }

    fn native_meth_append(&self, name: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_append, object, name)
    }
}

impl method::Extend for ObjectRef {
    fn meth_extend(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_extend, object, name)
    }

    fn native_meth_extend(&self, name: &Builtin) -> NativeResult<native::None> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_extend, object, name)
    }
}

impl method::Pop for ObjectRef {
    fn meth_pop(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_pop, object, name)
    }

    fn native_meth_pop(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_pop, object, name)
    }
}

impl method::Remove for ObjectRef {
    fn meth_remove(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_remove, object, name)
    }

    fn native_meth_remove(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_remove, object, name)
    }
}

impl method::IsDisjoint for ObjectRef {
    fn meth_isdisjoint(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_isdisjoint, object, name)
    }

    fn native_meth_isdisjoint(&self, name: &Builtin) -> NativeResult<native::Boolean> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_isdisjoint, object, name)
    }
}

impl method::AddItem for ObjectRef {
    fn meth_add(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_add, object, name)
    }

    fn native_meth_add(&self, name: &Builtin) -> NativeResult<Builtin> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_add, object, name)
    }
}

impl method::Discard for ObjectRef {}
impl method::Clear for ObjectRef {}
impl method::Get for ObjectRef {}
impl method::Keys for ObjectRef {
    fn meth_keys(&self, rt: &Runtime) -> RuntimeResult {
        let builtin: &Box<Builtin> = self.0.borrow();
        foreach_builtin!(builtin.deref(), rt, meth_keys, object)
    }

    fn native_meth_keys(&self) -> NativeResult<native::Tuple> {
        let builtin: &Box<Builtin> = self.0.borrow();
        native_foreach_builtin!(builtin.deref(), native_meth_keys, object)
    }
}
impl method::Values for ObjectRef {}
impl method::Items for ObjectRef {}
impl method::PopItem for ObjectRef {}
impl method::Update for ObjectRef {}
impl method::SetDefault for ObjectRef {}
impl method::Await for ObjectRef {}
impl method::Send for ObjectRef {}
impl method::Throw for ObjectRef {}
impl method::Close for ObjectRef {}
impl method::Exit for ObjectRef {}
impl method::Enter for ObjectRef {}
impl method::DescriptorGet for ObjectRef {}
impl method::DescriptorSet for ObjectRef {}
impl method::DescriptorSetName for ObjectRef {}


///
pub struct WeakObjectRef(pub native::RuntimeWeakRef);


impl Default for WeakObjectRef {
    fn default() -> WeakObjectRef {
        WeakObjectRef(Weak::default())
    }
}


impl WeakObjectRef {
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
            None => Err(Error::runtime("Attempted to create a strong ref to a an object with no existing refs")),
            Some(objref) => Ok(ObjectRef(objref)),
        }
    }
}

impl Clone for WeakObjectRef {
    fn clone(&self) -> Self {
        WeakObjectRef((self.0).clone())
    }
}

impl Hash for WeakObjectRef {
    #[allow(unused_variables)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // noop since we use Holder elements with manually computed hashes
    }
}


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