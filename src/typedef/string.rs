use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};
use std::hash::{Hash, SipHasher, Hasher};

use num::{BigInt, FromPrimitive};

use object;
use typedef::native;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use error::{Error, ErrorType};

use typedef::objectref::ToRtWrapperType;
use typedef::integer::IntegerObject;
use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;


#[derive(Clone, Debug, Hash)]
pub struct StringObject {
    pub value: String,
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl StringObject {
    pub fn from_str(value: &'static str) -> StringObject {
        return StringObject::new(value.to_string());
    }

    pub fn new(value: std::string::String) -> StringObject {
        StringObject { value: value }
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl objectref::RtObject for StringObject {}
impl object::model::PyObject for StringObject {}
impl object::model::PyBehavior for StringObject {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err),
            _ => unreachable!(),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        self.hash(&mut s);
        Ok(s.finish())
    }

    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => if value { Ok(rt.True()) } else { Ok(rt.False()) },
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::String(ref string) => Ok(self.value.eq(&string.value)),
            _ => Ok(false),
        }
    }

    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_repr() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        Ok(format!("{:?}", self.value.clone()))
    }

    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        // TODO: Refs back to self in the object holder type - this should be just a
        // return self.ref.clone().
        match self.native_str() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => unreachable!(),
        }
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(self.value.clone())
    }

    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();
        match self.native_add(&builtin.deref()) {
            Ok(Builtin::String(string)) => rt.alloc(string.to()),
            Err(err) => Err(err),
            _ => unreachable!(),
        }
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin> {
        match rhs {
            &Builtin::String(ref obj) => {
                let new_string = StringObject::new(self.value.clone() + obj.value.borrow());
                Ok(Builtin::String(new_string))
            }
            _ => Err(Error::typerr("TypeError cannot add to str")),
        }
    }
}


impl objectref::ToRtWrapperType<Builtin> for StringObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::String(self);
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for StringObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//         Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod impl_pybehavior {
    use super::*;
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::objectref::ToRtWrapperType;
    use typedef::complex::ComplexObject;
    use object::model::PyBehavior;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

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

    /// Called by built-in function hash() and for operations on members of hashed collections including
    /// set, frozenset, and dict. __hash__() should return an integer. The only required property is
    /// that objects which compare equal have the same hash value; it is advised to mix together
    /// the hash values of the components of the object that also play a part in comparison
    /// of objects by packing them into a tuple and hashing the tuple. Example:
    /// `api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);`
    #[test]
    fn __hash__() {
        let mut rt = Runtime::new(None);
        let string = rt.alloc(StringObject::from_str("I hash two strings in the mornin...").to()).unwrap();

        let boxed: &Box<Builtin> = string.0.borrow();
        // Dunno what to test here... we can hash and it is stable?
        let result1 = boxed.op_hash(&rt).unwrap();
        let result2 = boxed.op_hash(&rt).unwrap();
        assert_eq!(result1, result2);
    }

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

    /// `api_test_stub!(binary, self, __add__, Add, op_add, native_add);`
    #[test]
    fn __add__() {
        let mut rt = Runtime::new(None);
        let cake = rt.alloc(StringObject::from_str("The cake").to()).unwrap();
        let lie = rt.alloc(StringObject::from_str(" is a lie!").to()).unwrap();
        let the_cake_is_a_lie = rt.alloc(StringObject::from_str("The cake is a lie!").to()).unwrap();

        let boxed: &Box<Builtin> = cake.0.borrow();
        let result = boxed.op_add(&rt, &lie).unwrap();
        assert_eq!(result, the_cake_is_a_lie);
    }

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
