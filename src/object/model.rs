use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use std::rc::Weak;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher, SipHasher};
use std::cell::{RefCell, Cell};

use num::FromPrimitive;
use num::Zero;

use typedef::objectref::ToRtWrapperType;
use result::{RuntimeResult, NativeResult};
use error::Error;
use runtime::Runtime;

use typedef::builtin::Builtin;
use typedef::objectref::{WeakObjectRef, ObjectRef};
use typedef::native;
use typedef::integer::IntegerObject;
use typedef::string::StringObject;


#[deprecated]
pub trait PyObject {}


/// Define the base default python object behavior
#[deprecated]
pub trait PyBehavior {
    // Identity operators
    // api_method!(unary, self, __bool__, Truth, op_bool, native_bool);
    // api_method!(binary, self, __not__, Not, op_not, native_not);
    // api_method!(binary, self, is_, Is, op_is, native_is);
    // api_method!(binary, self, is_not, IsNot, op_is_not, native_is_not);
    // api_method!(unary, self, __repr__, ToStringRepr, op_repr, native_repr, native::String);
    // api_method!(unary, self, __str__, ToString, op_str, native_str, native::String);
    api_method!(unary, self, __del__, Delete, op_del, native_del);

    fn identity(&self, rt: &Runtime) -> RuntimeResult {
        let objref: ObjectRef = IntegerObject::new_u64(self.native_identity()).to();
        return rt.alloc(objref);
    }

    fn native_identity(&self) -> native::ObjectId {
        return (&self as *const _) as native::ObjectId;
    }

    fn op_is(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is(rhs_builtin).unwrap() {
            Ok(rt.OldTrue())
        } else {
            Ok(rt.OldFalse())
        }
    }

    fn op_is_not(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is_not(rhs_builtin).unwrap() {
            Ok(rt.OldTrue())
        } else {
            Ok(rt.OldFalse())
        }
    }

    fn native_is(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_identity() == other.native_identity())
    }

    fn native_is_not(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(!self.native_is(other).unwrap())
    }

    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        if self.native_bool().unwrap() {
            Ok(rt.OldTrue())
        } else {
            Ok(rt.OldFalse())
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(true)
    }

    /// Default implementation gives a string similar to pythons default
    /// repr in the form "<object [NAME] at [ADDR]>".
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_str() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        // TODO: Once types are implemented this should inject the name
        // from the type struct.
        Ok(format!("<object UnknownType at {:?}>", (&self as *const _)))
    }

    /// `op_str()` falls back to `op_repr` as per cPython's default
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.op_repr(rt)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        return self.native_repr();
    }

    /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
    api_method!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
    api_method!(binary, self, __format__, Format, op_format, native_format);


    /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
    //api_method!(binary, self, __eq__, Equal, op_eq, native_eq);
    //api_method!(binary, self, __ne__, NotEqual, op_ne, native_ne);
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_eq(rhs_builtin).unwrap() {
            Ok(rt.OldTrue())
        } else {
            Ok(rt.OldFalse())
        }
    }

    /// Default implementation of equals fallsbacks to op_is_not.
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_ne(rhs_builtin).unwrap() {
            Ok(rt.OldTrue())
        } else {
            Ok(rt.OldFalse())
        }
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return self.native_is(other);
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_ne(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return Ok(!self.native_eq(other).unwrap());
    }

    api_method!(binary, self, __lt__, LessThan, op_lt, native_lt);
    api_method!(binary, self, __le__, LessOrEqual, op_le, native_le);
    api_method!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
    api_method!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

    // Called by built-in function hash() and for operations on members of hashed collections including
    // set, frozenset, and dict. __hash__() should return an integer. The only required property is
    // that objects which compare equal have the same hash value; it is advised to mix together
    // the hash values of the components of the object that also play a part in comparison
    // of objects by packing them into a tuple and hashing the tuple. Example:
    // api_method!(unary, self, __hash__, Hashable, op_hash, native_hash);
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err),
        }
    }

    /// Default implementation of the native hash is to
    /// use the ptr identity and hash that.
    /// Numerical types especially should override
    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        self.native_identity().hash(&mut s);
        Ok(s.finish())
    }


    // 3.3.6. Emulating container types
    api_method!(unary, self, __len__, Length, op_len, native_len, native::Integer);
    api_method!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint, native::Integer);
    api_method!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
    api_method!(binary, self, __missing__, MissingItem, op_missing, native_missing);
    api_method!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem, native::NoneValue);
    api_method!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
    api_method!(unary, self, __iter__, Iterator, op_iter, native_iter);
    api_method!(unary, self, __reversed__, Reverse, op_reverse, native_reverse);
    api_method!(binary, self, __contains__, Contains, op_contains, native_contains, native::Boolean);

    // 3.3.7. Emulating numeric types
    //
    // The following methods can be defined to emulate numeric objects. Methods corresponding to
    // operations that are not supported by the particular kind of number implemented
    // (e.g., bitwise operations for non-integral numbers) should be left undefined.
    api_method!(binary, self, __add__, Add, op_add, native_add);
    api_method!(binary, self, __and__, And, op_and, native_and);
    api_method!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
    api_method!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
    api_method!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
    api_method!(binary, self, __mod__, Modulus, op_mod, native_mod);
    api_method!(binary, self, __mul__, Multiply, op_mul, native_mul);
    api_method!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);
    api_method!(binary, self, __or__, Or, op_or, native_or);
    api_method!(ternary, self, __pow__, Pow, op_pow, native_pow);
    api_method!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);
    api_method!(binary, self, __sub__, Subtract, op_sub, native_sub);
    api_method!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
    api_method!(binary, self, __xor__, XOr, op_xor, native_xor);

    // Reflected Operators
    api_method!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);
    api_method!(binary, self, __rand__, ReflectedAnd, op_rand, native_rand);
    api_method!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);
    api_method!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);
    api_method!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);
    api_method!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);
    api_method!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);
    api_method!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);
    api_method!(binary, self, __ror__, ReflectedOr, op_ror, native_ror);
    api_method!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);
    api_method!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);
    api_method!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);
    api_method!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);
    api_method!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

    // In place operators
    api_method!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);
    api_method!(binary, self, __iand__, InPlaceAnd, op_iand, native_iand);
    api_method!(binary, self, __idivmod__, InPlaceDivMod, op_idivmod, native_idivmod);
    api_method!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);
    api_method!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);
    api_method!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);
    api_method!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);
    api_method!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);
    api_method!(binary, self, __ior__, InPlaceOr, op_ior, native_ior);
    api_method!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);
    api_method!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);
    api_method!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);
    api_method!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);
    api_method!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);

    // Standard unary operators
    api_method!(unary, self, __neg__, Negate, op_neg, native_neg);
    api_method!(unary, self, __abs__, Abs, op_abs, native_abs);
    api_method!(unary, self, __pos__, Positive, op_pos, native_pos);
    api_method!(unary, self, __invert__, Invert, op_invert, native_invert);

    // Standard numeric conversions
    api_method!(unary, self, __complex__, ToComplex, op_complex, native_complex, native::Complex);
    api_method!(unary, self, __int__, ToInt, op_int, native_int, native::Integer);
    api_method!(unary, self, __float__, ToFloat, op_float, native_float, native::Float);
    api_method!(unary, self, __round__, ToRounded, op_round, native_round, native::Integer);

    api_method!(unary, self, __index__, ToIndex, op_index, native_index, native::Integer);
}

//trait CollectionPyBehavior: PyBehavior {
//    api_method!(unary, self, __len__, Length, op_len, native_len);
//    api_method!(unary, self, __iter__, Iterator, op_iter, native_iter);
//    api_method!(binary, self, __contains__, Contains, op_contains, native_contains);
//}
//
//trait SequencePyBehavior: PyBehavior {
//    api_method!(unary, self, __len__, Length, op_len, native_len);
//
//}

///// Sized	 	__len__
//pub trait Sized {
//    fn op_len(&self, &Runtime) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//}
//
///// Collection	Sized, Iterable, Container	__contains__, __iter__, __len__
//pub trait Collection: Sized + Iterable + Container {}
//
///// Sequence	Reversible, Collection	__getitem__, __len__	__contains__, __iter__, __reversed__, index, and count
//pub trait Sequence: Reversible + Collection {
//    fn op_getitem(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn index(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn count(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//}

///// MutableSequence	Sequence	__getitem__, __setitem__, __delitem__, __len__, insert	Inherited Sequence methods and append, reverse, extend, pop, remove, and __iadd__
//pub trait MutableSequence: Sequence {
//    fn op_setitem(&self, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn op_delitem(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn op_iadd(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn insert(&self, &Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn append(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn extend(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn pop(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//
//    fn remove(&self, &Runtime, &ObjectRef) -> RuntimeResult {
//        Err(Error::not_implemented())
//    }
//}
//
//// Set	Collection	__contains__, __iter__, __len__	__le__, __lt__, __eq__, __ne__, __gt__, __ge__, __and__, __or__, __sub__, __xor__, and isdisjoint
////pub trait Set: Collection {
////    fn op__le__, __lt__, __eq__, __ne__, __gt__, __ge__, __and__, __or__, __sub__, __xor__, and isdisjoint
////}
//// MutableSet	Set	__contains__, __iter__, __len__, add, discard	Inherited Set methods and clear, pop, remove, __ior__, __iand__, __ixor__, and __isub__
//
/// This module represents more advanced types outside of the scope of
/// the initial phase of "get the builtin type system working" since
/// their implementation can heavily influence the design of the interpreter
/// ... I'll cross that bridge when I get to it.
#[cfg(test)]
mod incomplete_models {
    use super::*;
    // __init_subclass__
    // __prepare__
    // __instancecheck__
    // __subclasscheck__

    trait TypeModel {
        api_method!(variadic, self, __new__, New, new_type, native_new_type);
        api_method!(variadic, self, __init__, Init, init_instance, native_init_instance);
    api_method!(binary, self, __getattr__, GetAttribute, op_getattr, native_getattr);
    api_method!(binary, self, __getattribute__, StrictGetAttribute, op_getattribute, native_getattribute);
    api_method!(binary, self, __setattr__, SetAttribute, op_setattr, native_setattr);
    api_method!(binary, self, __del__, DeleteAttribute, op_delattr, native_delattr);
    api_method!(binary, self, __dir__, Directory, op_dir, native_dir);
    }

    trait DescriptorModel {
        api_method!(ternary, self, __get__, DescriptorGet, op_descriptor_get, native_descriptor_get);
        api_method!(ternary, self, __set__, DescriptorSet, op_descriptor_set, native_descriptor_set);
        api_method!(binary, self, __del__, DescriptorDelete, op_descriptor_del, native_descriptor_del);
        api_method!(ternary, self, __set_name__, DescriptorSetName, op_descriptor_set_name, native_descriptor_set_name);
    }


    trait ContextManagerModel {
        api_method!(unary, self, __enter__, ContextEnter, op_ctx_enter, native_ctx_enter);
        api_method!(4ary, self, __exit__, ContextExit, op_ctx_exit, native_ctx_exit);
    }

    trait CoroutineModel {
        api_method!(unary, self, __await__, CoroutineAwait, op_coro_await, native_coro_await);
        api_method!(binary, self, send, CoroutineSend, op_coro_send, native_coro_send);
        api_method!(4ary, self, throw, CoroutineThrow, op_coro_throw, native_coro_throw);
        api_method!(unary, self, close, CoroutineClose, op_coro_close, native_coro_close);
    }
}
