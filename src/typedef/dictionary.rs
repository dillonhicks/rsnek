use std::fmt;
use std::borrow::Borrow;
use num::{Zero, FromPrimitive};
use std::cell::RefCell;
use result::{NativeResult, RuntimeResult};

use runtime::Runtime;
use error::Error;
use super::objectref::{self, ObjectRef};
use typedef;
use typedef::native::*;
use typedef::native;
use typedef::integer::IntegerObject;
use typedef::objectref::ToRtWrapperType;
use typedef::builtin::Builtin;

use object;


#[derive(Clone, Debug)]
pub struct DictionaryObject {
    value: RefCell<Dictionary>
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl DictionaryObject {
    fn new () -> Self {
        return DictionaryObject {
            value: RefCell::new(Dictionary::new())
        }
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl typedef::objectref::RtObject for DictionaryObject {}
impl object::model::PyObject for DictionaryObject {}
impl object::model::PyBehavior for DictionaryObject {

    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        Err(Error::typerr("Unhashable type dict"))
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        Err(Error::typerr("Unhashable type dict"))
    }

    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        Ok(if self.native_bool().unwrap() {rt.True()} else {rt.False()})
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.borrow().is_empty())
    }

    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_len() {
            Ok(int) => rt.alloc(IntegerObject::new_bigint(int).to()),
            Err(_) => unreachable!()
        }
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        match Integer::from_usize(self.value.borrow().len()) {
            Some(int) => Ok(int),
            None => Err(Error::value("ValueError converting native integer"))
        }
    }

    fn op_setitem(&self, rt: &Runtime, keyref: &ObjectRef, valueref: &ObjectRef) -> RuntimeResult {
        let key_value: &Box<Builtin>  = keyref.borrow();
        match key_value.native_hash() {
            Ok(hash) => {
                let key = Key(hash, keyref.clone());
                let result = self.value.borrow_mut().insert(key, valueref.clone());
                //if result.is_some() {Ok(rt.None())} else {Err(Error::runtime("RuntimeError: Cannot add item to dictionary"))}
                Ok(rt.None())
            },
            Err(err) => Err(Error::typerr("TypeError: Unhashable key type"))
        }
    }

}


impl typedef::objectref::ToRtWrapperType<Builtin> for DictionaryObject {
    fn to(self) -> Builtin {
        Builtin::Dictionary(self)
    }
}


impl typedef::objectref::ToRtWrapperType<ObjectRef> for DictionaryObject {
    fn to(self) -> ObjectRef{
        ObjectRef::new(Builtin::Dictionary(self))
    }
}


impl fmt::Display for DictionaryObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[cfg(test)]
mod impl_pybehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use super::{Integer, IntegerObject};
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

    use super::*;

    /// Call the identity function to make sure it succeeds
    #[test]
    fn identity() {
        let mut rt = Runtime::new(None);
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.identity(&rt).unwrap();
    }

    /// Ensure that the dictionary reference equality succeeds with itself
    #[test]
    fn op_is() {
        let mut rt = Runtime::new(None);
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.op_is(&rt, &empty_dict).unwrap();
        assert_eq!(result, rt.True());
    }

    /// Ensure that the dictionary reference equality does not
    /// match against two separarte dictionaries.
    #[test]
    fn op_is_not() {
        let mut rt = Runtime::new(None);
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();
        let another_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.op_is_not(&rt, &another_dict).unwrap();
        assert_eq!(result, rt.False());
    }

    /// Mutable collection types should not be hashable
    #[test]
    #[should_panic]
    fn __hash__() {
        let mut rt = Runtime::new(None);
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();
        let result = dict_ref.op_hash(&rt).unwrap();
    }


    #[test]
    fn __bool__() {
        let mut rt = Runtime::new(None);
        let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.op_len(&rt).unwrap();
        assert_eq!(result, zero_int);
    }



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
    // api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);

    // Identity operators
    // api_test_stub!(unary, self, identity, Identity, identity, native_identity, native::Boolean);
    // api_test_stub!(unary, self, __bool__, Truth, op_bool, native_bool, native::Boolean);
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


#[cfg(test)]
mod impl_mappingpybehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use super::{Integer, IntegerObject};
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

    use super::*;


    #[test]
    fn op_len() {
        let mut rt = Runtime::new(None);
        let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.op_len(&rt).unwrap();
        assert_eq!(result, zero);
    }

    #[test]
    fn op_setitem() {
        let mut rt = Runtime::new(None);
        let key: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let value: ObjectRef = rt.alloc(StringObject::from_str("Dictionary Value").to()).unwrap();
        let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

        let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

        let result = dict_ref.op_setitem(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.None());

        let result = dict_ref.op_len(&rt).unwrap();
        assert_eq!(result, rt.One());
    }

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

}