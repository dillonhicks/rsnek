use std;
use std::fmt;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

use object::{self, RtValue};
use object::model::PyBehavior;
use object::typing::Type;
use object::method;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::integer::IntegerObject;
use typedef::objectref::ToRtWrapperType;
use typedef::float::FloatObject;
use object::selfref::{RefCount, SelfRef};
use runtime::PythonType;

use typedef::builtin;
use typedef::builtin::Builtin;
use typedef::objectref;
use typedef::objectref::ObjectRef;
use typedef::native;
use typedef::complex::ComplexObject;
use typedef::string::StringObject;

use num::Zero;
use num::FromPrimitive;
use num::ToPrimitive;


pub const TRUE_STR: &'static str = "True";
pub const FALSE_STR: &'static str = "False";


#[allow(non_snake_case)]
pub struct BooleanSingletons {
    True: ObjectRef,
    False: ObjectRef,
}


#[derive(Clone)]
pub struct BooleanType;

#[derive(Clone)]
pub struct BoolValue(pub native::Integer);
pub type PyBoolean = RtValue<BoolValue>;


impl std::fmt::Debug for PyBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.value.0.is_zero() {
            write!(f, "{} - {:?}", FALSE_STR, self.rc)
        } else {
            write!(f, "{} - {:?}", TRUE_STR, self.rc)
        }
    }
}


impl Type for BooleanType {
    type T = PyBoolean;


    fn op_new(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_new() {
            Ok(inst) => {
                let objref: ObjectRef = ObjectRef::new(Builtin::Bool(inst));
                let result = rt.alloc(objref.clone());
                match result {
                    Ok(selfref) => {
                        // TODO: This looks gross but might be the only way
                        // to set the selfref unless the heap can pass back mut
                        // boxes.
                        let builtin: &Box<Builtin> = objref.0.borrow();
                        let bool: &PyBoolean = builtin.bool().unwrap();
                        bool.rc.set(&selfref.clone());
                        Ok(selfref)
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_new(&self) -> NativeResult<Self::T> {
        Ok(PyBoolean {
               value: BoolValue(native::Integer::zero()),
               rc: RefCount::default(),
           })
    }

    fn op_init(&mut self, rt: &Runtime) -> RuntimeResult {
        self.op_new(&rt)
    }

    #[inline]
    fn native_init(&mut self) -> NativeResult<()> {
        Ok(())
    }

    fn op_name(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_name() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn native_name(&self) -> NativeResult<native::String> {
        Ok("Boolean".to_string())
    }
}


impl method::GetAttr for PyBoolean {}
impl method::GetAttribute for PyBoolean {}
impl method::SetAttr for PyBoolean {}
impl method::DelAttr for PyBoolean {}
impl object::identity::DefaultIdentity for PyBoolean {}
impl method::Id for PyBoolean {}
impl method::Is for PyBoolean {}
impl method::IsNot for PyBoolean {}
impl method::Add for PyBoolean {}
impl method::BitwiseAnd for PyBoolean {}
impl method::DivMod for PyBoolean {}
impl method::FloorDivision for PyBoolean {}
impl method::LeftShift for PyBoolean {}
impl method::Modulus for PyBoolean {}
impl method::Multiply for PyBoolean {}
impl method::MatrixMultiply for PyBoolean {}
impl method::BitwiseOr for PyBoolean {}
impl method::Pow for PyBoolean {} // Not exactly a binary op
impl method::RightShift for PyBoolean {}
impl method::Subtract for PyBoolean {}
impl method::TrueDivision for PyBoolean {}
impl method::XOr for PyBoolean {}
impl method::ReflectedAdd for PyBoolean {}
impl method::ReflectedBitwiseAnd for PyBoolean {}
impl method::ReflectedDivMod for PyBoolean {}
impl method::ReflectedFloorDivision for PyBoolean {}
impl method::ReflectedLeftShift for PyBoolean {}
impl method::ReflectedModulus for PyBoolean {}
impl method::ReflectedMultiply for PyBoolean {}
impl method::ReflectedMatrixMultiply for PyBoolean {}
impl method::ReflectedBitwiseOr for PyBoolean {}
impl method::ReflectedPow for PyBoolean {} // Not exactly a binary op
impl method::ReflectedRightShift for PyBoolean {}
impl method::ReflectedSubtract for PyBoolean {}
impl method::ReflectedTrueDivision for PyBoolean {}
impl method::ReflectedXOr for PyBoolean {}
impl method::InPlaceAdd for PyBoolean {}
impl method::InPlaceBitwiseAnd for PyBoolean {}
impl method::InPlaceDivMod for PyBoolean {}
impl method::InPlaceFloorDivision for PyBoolean {}
impl method::InPlaceLeftShift for PyBoolean {}
impl method::InPlaceModulus for PyBoolean {}
impl method::InPlaceMultiply for PyBoolean {}
impl method::InPlaceMatrixMultiply for PyBoolean {}
impl method::InPlaceBitwiseOr for PyBoolean {}
impl method::InPlacePow for PyBoolean {} // Not exactly a binary op
impl method::InPlaceRightShift for PyBoolean {}
impl method::InPlaceSubtract for PyBoolean {}
impl method::InPlaceTrueDivision for PyBoolean {}
impl method::InPlaceXOr for PyBoolean {}


impl ToRtWrapperType<builtin::Builtin> for PyBoolean {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Bool(self)
    }
}

impl ToRtWrapperType<objectref::ObjectRef> for PyBoolean {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Bool(self))
    }
}


/// old
///
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BooleanObject {
    value: native::Integer,
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//       Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl BooleanObject {
    pub fn new_true() -> BooleanObject {
        return BooleanObject { value: native::Integer::from(1) };
    }

    pub fn new_false() -> BooleanObject {
        return BooleanObject { value: native::Integer::from(0) };
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for BooleanObject {}
impl object::model::PyObject for BooleanObject {}
impl object::model::PyBehavior for BooleanObject {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => if value { Ok(rt.True()) } else { Ok(rt.False()) },
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool().unwrap() == value),
            Err(err) => Err(err),
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.is_zero());
    }

    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => rt.alloc(IntegerObject::new_bigint(int).to()),
            Err(err) => Err(err),
        }

    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        return Ok(self.value.clone());
    }

    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_float() {
            Ok(float) => rt.alloc(FloatObject::new(float).to()),
            Err(err) => Err(err),
        }

    }

    fn native_float(&self) -> NativeResult<native::Float> {
        return Ok(self.value.to_f64().unwrap());
    }

    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_complex() {
            Ok(value) => return rt.alloc(ComplexObject::from_native(value).to()),
            Err(err) => Err(err),
        }
    }

    fn native_complex(&self) -> NativeResult<native::Complex> {
        return Ok(native::Complex::new(self.value.to_f64().unwrap(), 0.));
    }

    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        self.op_int(&rt)
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        return self.native_int();
    }

    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_repr() {
            Ok(string) => rt.alloc(StringObject::new(string).to()),
            Err(err) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        let value = if self.value.is_zero() {
            FALSE_STR
        } else {
            TRUE_STR
        };
        Ok(value.to_string())
    }

    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.op_repr(rt)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        return self.native_repr();
    }
}


impl objectref::ToRtWrapperType<builtin::Builtin> for BooleanObject {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Boolean(self)
    }
}

impl objectref::ToRtWrapperType<objectref::ObjectRef> for BooleanObject {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Boolean(self))
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for BooleanObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
#[allow(non_snake_case)]
mod impl_pybehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};
    use object::model::PyBehavior;

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::integer::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::complex::ComplexObject;
    use typedef::objectref::ToRtWrapperType;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

    use super::*;

    /// Reference equality
    ///  True is True
    #[test]
    fn is_() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);

        let False = rt.False();
        let False2 = False.clone();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.native_is(False_ref.deref()).unwrap();
        assert_eq!(result, true, "BooleanObject native is(native_is)");

        let result = False_ref.op_is(&mut rt, &False2).unwrap();
        assert_eq!(result, rt.True(), "BooleanObject is(op_is)");

    }

    ///
    /// True == True
    #[test]
    fn __eq__() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);

        let False = rt.False();
        let True = rt.True();

        let thing1 = rt.alloc(False.clone()).unwrap();
        let False2 = rt.alloc(False.clone()).unwrap();
        let thing3 = rt.alloc(True.clone()).unwrap();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_eq(&rt, &False2.clone()).unwrap();
        assert_eq!(result, True, "BooleanObject equality (op_equals)");

        let result = False_ref.native_eq(False_ref).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    #[allow(non_snake_case)]
    fn __bool__() {
        let mut rt = Runtime::new(None);

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_bool(&rt).unwrap();
        assert_eq!(rt.True(), result);

        let result = True_ref.native_bool().unwrap();
        assert_eq!(result, true);

    }

    #[test]
    fn __int__() {
        let mut rt = Runtime::new(None);

        let one: ObjectRef = IntegerObject::new_u64(1).to();

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_int(&rt).unwrap();
        assert_eq!(result, one);
    }

    #[test]
    fn __complex__() {
        let mut rt = Runtime::new(None);

        let one_complex: ObjectRef = ComplexObject::from_f64(1.0, 0.0).to();
        let zero_complex: ObjectRef = ComplexObject::from_f64(0.0, 0.0).to();

        let True = rt.True();
        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_complex(&rt).unwrap();
        assert_eq!(result, one_complex);

        let False = rt.False();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_complex(&rt).unwrap();
        assert_eq!(result, zero_complex);
    }

    #[test]
    fn __float__() {
        let mut rt = Runtime::new(None);

        let one_float: ObjectRef = FloatObject::new(1.0).to();
        let zero_float: ObjectRef = FloatObject::new(0.0).to();

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_float(&rt).unwrap();
        assert_eq!(result, one_float);

        let False = rt.False();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_float(&rt).unwrap();
        assert_eq!(result, zero_float);
    }

    #[test]
    fn __index__() {
        let mut rt = Runtime::new(None);

        let zero: ObjectRef = IntegerObject::new_u64(0).to();
        let one: ObjectRef = IntegerObject::new_u64(1).to();

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_index(&rt).unwrap();
        assert_eq!(result, one);

        let False = rt.False();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_index(&rt).unwrap();
        assert_eq!(result, zero);
    }

    #[test]
    fn __repr__() {
        let mut rt = Runtime::new(None);

        let true_str: ObjectRef = rt.alloc(StringObject::from_str(TRUE_STR).to()).unwrap();
        let false_str: ObjectRef = rt.alloc(StringObject::from_str(FALSE_STR).to()).unwrap();

        let True = rt.True();
        let False = rt.False();

        let true_ref: &Box<Builtin> = True.0.borrow();
        let result = true_ref.op_repr(&rt).unwrap();
        assert_eq!(result, true_str);

        let false_ref: &Box<Builtin> = False.0.borrow();
        let result = false_ref.op_repr(&rt).unwrap();
        assert_eq!(result, false_str);
    }

    #[test]
    fn __str__() {
        let mut rt = Runtime::new(None);

        let true_str: ObjectRef = rt.alloc(StringObject::from_str(TRUE_STR).to()).unwrap();
        let false_str: ObjectRef = rt.alloc(StringObject::from_str(FALSE_STR).to()).unwrap();

        let True = rt.True();
        let False = rt.False();

        let true_ref: &Box<Builtin> = True.0.borrow();
        let result = true_ref.op_str(&rt).unwrap();
        assert_eq!(result, true_str);

        let false_ref: &Box<Builtin> = False.0.borrow();
        let result = false_ref.op_str(&rt).unwrap();
        assert_eq!(result, false_str);
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
    // api_test_stub!(binary, self, __eq__, Equal, op_eq, native_eq, native::Boolean);
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
    // api_test_stub!(binary, self, is_, Is, op_is, native_is, native::Boolean);
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
    //    api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
    //    api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
    //    api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
    //    api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
    //    api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);

}

#[cfg(test)]
mod impl_pytypebehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};
    use object::model::PyBehavior;

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::integer::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::complex::ComplexObject;
    use typedef::objectref::ToRtWrapperType;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

    use super::*;

    fn setup() -> (Runtime, BooleanType) {
        let rt = Runtime::new(None);
        let bool = BooleanType {};
        (rt, bool)
    }

    #[test]
    fn __new__() {
        let (rt, bool) = setup();

        println!("{:?}", bool.op_new(&rt))
    }
}
