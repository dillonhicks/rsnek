use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};
use std::hash::{Hash, SipHasher, Hasher};

use num::{Zero, FromPrimitive, ToPrimitive, BigInt};

use object;
use error::{Error, ErrorType};
use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use typedef::objectref::ToRtWrapperType;

use typedef::native;
use typedef::objectref;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::float::FloatObject;
use typedef::complex::ComplexObject;
use typedef::string::StringObject;


pub use typedef::native::Integer;


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct IntegerObject {
    pub value: Integer,
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl IntegerObject {
    #[inline]
    pub fn new_i64(value: i64) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

    #[inline]
    pub fn new_u64(value: u64) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

    pub fn new_bigint(value: Integer) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for IntegerObject {}
impl object::model::PyObject for IntegerObject {}
impl object::model::PyBehavior for IntegerObject {

    fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Integer(ref obj) => {
                let new_number: ObjectRef = IntegerObject::new_bigint(&self.value + &obj.value).to();
                runtime.alloc(new_number)
            }
            &Builtin::Float(ref obj) => {
                let new_number: ObjectRef = FloatObject::add_integer(obj, &self)?.to();
                runtime.alloc(new_number)
            }
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to int")),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        self.hash(&mut s);
        Ok(s.finish())
    }

    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err)
        }
    }

    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => {
                if value {
                    Ok(rt.True())
                } else {
                    Ok(rt.False())
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match other {
            &Builtin::Integer(ref obj) => Ok(self.value == obj.value),
            _ => Ok(false),
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.is_zero())
    }

    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => {
                // TODO: once self refs are are implemented, just
                // clone the ref and pass it back
                rt.alloc(IntegerObject::new_bigint(int).to())
            },
            Err(err) => Err(err)
        }
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        return Ok(self.value.clone())
    }

    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_float() {
            Ok(float) => rt.alloc(FloatObject::new(float).to()),
            Err(err) => Err(err)
        }

    }

    fn native_float(&self) -> NativeResult<native::Float> {
        return Ok(self.value.to_f64().unwrap())
    }

    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_complex() {
            Ok(value) => rt.alloc(ComplexObject::from_native(value).to()),
            Err(err) => Err(err)
        }
    }

    fn native_complex(&self) -> NativeResult<native::Complex> {
        return Ok(native::Complex::new(self.value.to_f64().unwrap(), 0.));
    }

    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        self.op_int(&rt)
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        return self.native_int()
    }

    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_str() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => unreachable!()
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        Ok(format!("{}", self.value))
    }

    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.op_repr(rt)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        return self.native_repr()
    }
}


impl objectref::ToRtWrapperType<Builtin> for IntegerObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Integer(self);
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for IntegerObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///          Tests
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

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


    #[test]
    fn test_integer_alloc() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = ObjectRef::new(Builtin::Integer(IntegerObject::new_i64(1)));
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        let one_clone = one.clone();
        assert_eq!(Rc::strong_count(&one.0), 5);

    }

    /// Create integer object on the stack and try to allocate it
    /// in the runtime.
    ///
    #[test]
    fn test_alloc_integer() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        /// A new integer should only alloc one spot on the heap
        assert_eq!(runtime.heap_size(), 1);
        println!("{:?}", runtime);
    }

    /// int+int => int
    /// api::BinaryOperation::op_add
    #[test]
    fn __add__() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one: ObjectRef = IntegerObject::new_i64(1).to();
        let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref: &Box<Builtin> = one.0.borrow();
        let two = one_ref.op_add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    /// Just try to init the runtime
    #[test]
    fn __eq__() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);


        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = rt.alloc(one_object.clone()).unwrap();
        assert_eq!(rt.heap_size(), 1);

        let another_one: ObjectRef= IntegerObject::new_i64(1).to();
        let one2: ObjectRef = rt.alloc(another_one.clone()).unwrap();
        assert_eq!(rt.heap_size(), 2);

        println!("{:?}", rt);

        let one_builtin: &Box<Builtin> = one.0.borrow();
        let result = one_builtin.op_eq(&mut rt, &one2).unwrap();

        assert_eq!(result, rt.True())
    }

    #[test]
    #[allow(non_snake_case)]
    fn __bool__() {
        let mut rt = Runtime::new(None);

        let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
        let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = neg_one.0.borrow();
        let result = test_case.op_bool(&rt).unwrap();
        assert_eq!(result, rt.True());

        let test_case: &Box<Builtin> = zero.0.borrow();
        let result = test_case.op_bool(&rt).unwrap();
        assert_eq!(result, rt.False());

        let test_case: &Box<Builtin> = pos_one.0.borrow();
        let result = test_case.op_bool(&rt).unwrap();
        assert_eq!(result, rt.True());
    }

    #[test]
    fn __int__() {
        let mut rt = Runtime::new(None);

        let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
        let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = neg_one.0.borrow();
        let result = test_case.op_int(&rt).unwrap();
        assert_eq!(result, neg_one);

        let test_case: &Box<Builtin> = zero.0.borrow();
        let result = test_case.op_int(&rt).unwrap();
        assert_eq!(result, zero);

        let test_case: &Box<Builtin> = pos_one.0.borrow();
        let result = test_case.op_int(&rt).unwrap();
        assert_eq!(result, pos_one);
    }

    #[test]
    fn __complex__() {
        let mut rt = Runtime::new(None);

        let zero_complex = rt.alloc(ComplexObject::from_f64(0.0, 0.0).to()).unwrap();
        let one_complex = rt.alloc(ComplexObject::from_f64(1.0, 0.0).to()).unwrap();

        let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let one_int: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = zero_int.0.borrow();
        let result = test_case.op_complex(&rt).unwrap();
        assert_eq!(result, zero_complex);

        let test_case: &Box<Builtin> = one_int.0.borrow();
        let result = test_case.op_complex(&rt).unwrap();
        assert_eq!(result, one_complex);
    }

    #[test]
    fn __float__() {
        let mut rt = Runtime::new(None);

        let zero_float = rt.alloc(FloatObject::new(0.0).to()).unwrap();
        let one_float = rt.alloc(FloatObject::new(1.0).to()).unwrap();

        let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let one_int: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = zero_int.0.borrow();
        let result = test_case.op_complex(&rt).unwrap();
        assert_eq!(result, zero_float);

        let test_case: &Box<Builtin> = one_int.0.borrow();
        let result = test_case.op_complex(&rt).unwrap();
        assert_eq!(result, one_float);
    }

    #[test]
    fn __str__() {
        let mut rt = Runtime::new(None);

        let neg_one_str: ObjectRef = rt.alloc(StringObject::from_str("-1").to()).unwrap();
        let zero_str: ObjectRef = rt.alloc(StringObject::from_str("0").to()).unwrap();
        let pos_one_str: ObjectRef = rt.alloc(StringObject::from_str("1").to()).unwrap();

        let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
        let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = neg_one.0.borrow();
        let result = test_case.op_str(&rt).unwrap();
        assert_eq!(result, neg_one_str);

        let test_case: &Box<Builtin> = zero.0.borrow();
        let result = test_case.op_str(&rt).unwrap();
        assert_eq!(result, zero_str);

        let test_case: &Box<Builtin> = pos_one.0.borrow();
        let result = test_case.op_str(&rt).unwrap();
        assert_eq!(result, pos_one_str);
    }


    #[test]
    fn __repr__() {
        let mut rt = Runtime::new(None);

        let neg_one_str: ObjectRef = rt.alloc(StringObject::from_str("-1").to()).unwrap();
        let zero_str: ObjectRef = rt.alloc(StringObject::from_str("0").to()).unwrap();
        let pos_one_str: ObjectRef = rt.alloc(StringObject::from_str("1").to()).unwrap();

        let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
        let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
        let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

        let test_case: &Box<Builtin> = neg_one.0.borrow();
        let result = test_case.op_repr(&rt).unwrap();
        assert_eq!(result, neg_one_str);

        let test_case: &Box<Builtin> = zero.0.borrow();
        let result = test_case.op_repr(&rt).unwrap();
        assert_eq!(result, zero_str);

        let test_case: &Box<Builtin> = pos_one.0.borrow();
        let result = test_case.op_repr(&rt).unwrap();
        assert_eq!(result, pos_one_str);

    }

    api_test_stub!(unary, self, __del__, Delete, op_del, native_del);
    // api_test_stub!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
    // api_test_stub!(unary, self, __str__, ToString, op_str, native_str);

    /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
    api_test_stub!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
    api_test_stub!(binary, self, __format__, Format, op_format, native_format);


    /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
    api_test_stub!(binary, self, __lt__, LessThan, op_lt, native_lt);
    api_test_stub!(binary, self, __le__, LessOrEqual, op_le, native_le);
    //api_test_stub!(binary, self, __eq__, Equal, op_eq, native_eq, native::Boolean);
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
    // api_test_stub!(binary, self, __add__, Add, op_add, native_add);
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
    //    api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
    //    api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
    //    api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
    //    api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
    //    api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);
}
