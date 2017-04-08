use std;
use std::ops::{Deref, Neg};
use std::borrow::Borrow;
use num::{Signed, Zero, FromPrimitive};

use object::{self, RtValue, typing, method};
use object::method::{BooleanCast, IntegerCast, StringRepresentation};
use object::selfref::{self, SelfRef};

use runtime::{Runtime, BooleanProvider};
use runtime::{StringProvider, IntegerProvider};
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;
use typedef::native::{self, Number};


pub const TRUE_STR: &'static str = "True";
pub const FALSE_STR: &'static str = "False";
pub const TRUE_BYTES: &'static [u8] = &[1];
pub const FALSE_BYTES: &'static [u8] = &[0];



#[derive(Clone)]
pub struct PyBooleanType {
    singleton_true: ObjectRef,
    singleton_false: ObjectRef,
}


impl typing::BuiltinType for PyBooleanType {
    type T = PyBoolean;
    type V = native::Boolean;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        if value {
            self.singleton_true.clone()
        } else {
            self.singleton_false.clone()
        }
    }

    fn init_type() -> Self {
        PyBooleanType {
            singleton_true: PyBooleanType::inject_selfref(PyBooleanType::alloc(true)),
            singleton_false: PyBooleanType::inject_selfref(PyBooleanType::alloc(false)),
        }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Bool(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Bool(ref boolean) => {
                boolean.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        let int = if value {
            native::Integer::from_usize(1).unwrap()
        } else {
            native::Integer::zero()
        };
        PyBoolean {
            value: BoolValue(int),
            rc: selfref::RefCount::default(),
        }
    }
}


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


impl object::PyAPI for PyBoolean {}
impl method::New for PyBoolean {}
impl method::Init for PyBoolean {}
impl method::Delete for PyBoolean {}
impl method::GetAttr for PyBoolean {}
impl method::GetAttribute for PyBoolean {}
impl method::SetAttr for PyBoolean {}
impl method::DelAttr for PyBoolean {}
impl method::Hashed for PyBoolean {}
impl method::StringCast for PyBoolean {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.op_repr(rt)
    }

    fn native_str(&self) -> NativeResult<native::String> {
        self.native_repr()
    }
}
impl method::BytesCast for PyBoolean {
    //    fn op_bytes(&self, rt: &Runtime) -> RuntimeResult {
    //        // TODO: Fix after PyBytes is implemented
    //        Err(Error::not_implemented())
    //    }

    fn native_bytes(&self) -> NativeResult<native::Bytes> {
        let result = if self.value.0.is_zero() {
            FALSE_BYTES.to_vec()
        } else {
            TRUE_BYTES.to_vec()
        };
        Ok(result)
    }
}
impl method::StringFormat for PyBoolean {}
impl method::StringRepresentation for PyBoolean {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_repr() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        let value = if self.value.0.is_zero() {
            FALSE_STR
        } else {
            TRUE_STR
        };
        Ok(value.to_string())
    }
}

/// `x == y`
impl method::Equal for PyBoolean {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => {
                if value {
                    Ok(rt.bool(true))
                } else {
                    Ok(rt.bool(false))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool().unwrap() == value),
            Err(err) => Err(err),
        }
    }
}

impl method::NotEqual for PyBoolean {
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_ne(builtin.deref()) {
            Ok(value) => {
                if value {
                    Ok(rt.bool(true))
                } else {
                    Ok(rt.bool(false))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool().unwrap() != value),
            Err(err) => Err(err),
        }
    }
}

impl method::LessThan for PyBoolean {}
impl method::LessOrEqual for PyBoolean {}
impl method::GreaterOrEqual for PyBoolean {}
impl method::GreaterThan for PyBoolean {}

impl method::BooleanCast for PyBoolean {
    #[allow(unused_variables)]
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        self.rc.upgrade()
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.0.is_zero())
    }
}
impl method::IntegerCast for PyBoolean {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(self.value.0.clone()))
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        Ok(self.value.0.clone())
    }
}

// TODO: FIXME when float is finished
impl method::FloatCast for PyBoolean {
    //    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
    //        unimplemented!()
    //        //        match self.native_float() {
    //        //            Ok(float) => rt.alloc(FloatObject::new(float).to()),
    //        //            Err(err) => Err(err),
    ////        }
    //
    //    }
    //
    //    fn native_float(&self) -> NativeResult<native::Float> {
    //        return Ok(self.value.0.to_f64().unwrap());
    //    }
}

// TODO: FIXME when complex is finished
impl method::ComplexCast for PyBoolean {
    //    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
    //        unimplemented!()
    //        //        match self.native_complex() {
    //        //            Ok(value) => rt.alloc(ComplexObject::from_native(value).to()),
    //        //            Err(err) => Err(err),
    //        //        }
    //    }
    //
    //    fn native_complex(&self) -> NativeResult<native::Complex> {
    //        Ok(native::Complex::new(self.value.0.to_f64().unwrap(), 0.))
    //    }
}

/// `round(True) => 1` `round(False) => 0`
impl method::Rounding for PyBoolean {
    fn op_round(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_round() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_round(&self) -> NativeResult<Number> {
        Ok(Number::Int(self.value.0.clone()))
    }
}

/// `__index___`
impl method::Index for PyBoolean {
    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_index() {
            Ok(int) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_index(&self) -> NativeResult<native::Integer> {
        self.native_int()
    }
}

/// `-self`
impl method::NegateValue for PyBoolean {
    fn op_neg(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_neg() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_neg(&self) -> NativeResult<Number> {
        Ok(Number::Int(self.value
                           .0
                           .clone()
                           .neg()))
    }
}

/// `__abs__`
impl method::AbsValue for PyBoolean {
    fn op_abs(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_abs() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_abs(&self) -> NativeResult<Number> {
        Ok(Number::Int(self.value.0.abs()))
    }
}

/// `+self`
impl method::PositiveValue for PyBoolean {
    fn op_pos(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_pos() {
            Ok(Number::Int(int)) => Ok(rt.int(int)),
            _ => unreachable!(),
        }
    }

    fn native_pos(&self) -> NativeResult<Number> {
        Ok(Number::Int(self.value.0.clone()))
    }
}

impl method::InvertValue for PyBoolean {}
impl method::Add for PyBoolean {}
impl method::BitwiseAnd for PyBoolean {}
impl method::DivMod for PyBoolean {}
impl method::FloorDivision for PyBoolean {}
impl method::LeftShift for PyBoolean {}
impl method::Modulus for PyBoolean {}
impl method::Multiply for PyBoolean {}
impl method::MatrixMultiply for PyBoolean {}
impl method::BitwiseOr for PyBoolean {}
impl method::Pow for PyBoolean {}
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
impl method::ReflectedPow for PyBoolean {}
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
impl method::InPlacePow for PyBoolean {}
impl method::InPlaceRightShift for PyBoolean {}
impl method::InPlaceSubtract for PyBoolean {}
impl method::InPlaceTrueDivision for PyBoolean {}
impl method::InPlaceXOr for PyBoolean {}
impl method::Contains for PyBoolean {}
impl method::Iter for PyBoolean {}
impl method::Call for PyBoolean {}
impl method::Length for PyBoolean {}
impl method::LengthHint for PyBoolean {}
impl method::Next for PyBoolean {}
impl method::Reversed for PyBoolean {}
impl method::GetItem for PyBoolean {}
impl method::SetItem for PyBoolean {}
impl method::DeleteItem for PyBoolean {}
impl method::Count for PyBoolean {}
impl method::Append for PyBoolean {}
impl method::Extend for PyBoolean {}
impl method::Pop for PyBoolean {}
impl method::Remove for PyBoolean {}
impl method::IsDisjoint for PyBoolean {}
impl method::AddItem for PyBoolean {}
impl method::Discard for PyBoolean {}
impl method::Clear for PyBoolean {}
impl method::Get for PyBoolean {}
impl method::Keys for PyBoolean {}
impl method::Values for PyBoolean {}
impl method::Items for PyBoolean {}
impl method::PopItem for PyBoolean {}
impl method::Update for PyBoolean {}
impl method::SetDefault for PyBoolean {}
impl method::Await for PyBoolean {}
impl method::Send for PyBoolean {}
impl method::Throw for PyBoolean {}
impl method::Close for PyBoolean {}
impl method::Exit for PyBoolean {}
impl method::Enter for PyBoolean {}
impl method::DescriptorGet for PyBoolean {}
impl method::DescriptorSet for PyBoolean {}
impl method::DescriptorSetName for PyBoolean {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod _api_method {
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();
        let t = rt.bool(true);

        let f_ref: &Box<Builtin> = f.0.borrow();

        let result = f_ref.op_is(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(true), "BooleanObject is(op_is)");

        let result = f_ref.op_is(&rt, &t).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    #[test]
    fn is_not() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();
        let t = rt.bool(true);

        let f_ref: &Box<Builtin> = f.0.borrow();

        let result = f_ref.op_is_not(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(false), "BooleanObject is(op_is)");

        let result = f_ref.op_is_not(&rt, &t).unwrap();
        assert_eq!(result, rt.bool(true));
    }


    #[test]
    fn __eq__() {
        let rt = setup_test();

        let f = rt.bool(false);
        let f2 = f.clone();

        let f_ref: &Box<Builtin> = f.0.borrow();
        let result = f_ref.op_eq(&rt, &f2).unwrap();
        assert_eq!(result, rt.bool(true))
    }

    #[test]
    fn __bool__() {
        let rt = setup_test();

        let (t, f) = (rt.bool(true), rt.bool(false));

        let t_ref: &Box<Builtin> = t.0.borrow();
        let f_ref: &Box<Builtin> = f.0.borrow();

        let result = t_ref.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = f_ref.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    #[test]
    fn __int__() {
        let rt = setup_test();

        let one = rt.int(1);

        let t = rt.bool(true);
        let t_ref: &Box<Builtin> = t.0.borrow();

        let result = t_ref.op_int(&rt).unwrap();
        assert_eq!(result, one);
    }
}

#[cfg(all(feature="old", test))]
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

        let False = rt.OldFalse();
        let False2 = False.clone();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.native_is(False_ref.deref()).unwrap();
        assert_eq!(result, true, "BooleanObject native is(native_is)");

        let result = False_ref.op_is(&mut rt, &False2).unwrap();
        assert_eq!(result, rt.OldTrue(), "BooleanObject is(op_is)");

    }

    ///
    /// True == True
    #[test]
    fn __eq__() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);

        let False = rt.OldFalse();
        let True = rt.OldTrue();

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

        let True = rt.OldTrue();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_bool(&rt).unwrap();
        assert_eq!(rt.OldTrue(), result);

        let result = True_ref.native_bool().unwrap();
        assert_eq!(result, true);

    }

    #[test]
    fn __int__() {
        let mut rt = Runtime::new(None);

        let one: ObjectRef = IntegerObject::new_u64(1).to();

        let True = rt.OldTrue();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_int(&rt).unwrap();
        assert_eq!(result, one);
    }

    #[test]
    fn __complex__() {
        let mut rt = Runtime::new(None);

        let one_complex: ObjectRef = ComplexObject::from_f64(1.0, 0.0).to();
        let zero_complex: ObjectRef = ComplexObject::from_f64(0.0, 0.0).to();

        let True = rt.OldTrue();
        let True = rt.OldTrue();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_complex(&rt).unwrap();
        assert_eq!(result, one_complex);

        let False = rt.OldFalse();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_complex(&rt).unwrap();
        assert_eq!(result, zero_complex);
    }

    #[test]
    fn __float__() {
        let mut rt = Runtime::new(None);

        let one_float: ObjectRef = FloatObject::new(1.0).to();
        let zero_float: ObjectRef = FloatObject::new(0.0).to();

        let True = rt.OldTrue();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_float(&rt).unwrap();
        assert_eq!(result, one_float);

        let False = rt.OldFalse();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_float(&rt).unwrap();
        assert_eq!(result, zero_float);
    }

    #[test]
    fn __index__() {
        let mut rt = Runtime::new(None);

        let zero: ObjectRef = IntegerObject::new_u64(0).to();
        let one: ObjectRef = IntegerObject::new_u64(1).to();

        let True = rt.OldTrue();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_index(&rt).unwrap();
        assert_eq!(result, one);

        let False = rt.OldFalse();
        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_index(&rt).unwrap();
        assert_eq!(result, zero);
    }

    #[test]
    fn __repr__() {
        let mut rt = Runtime::new(None);

        let true_str: ObjectRef = rt.alloc(StringObject::from_str(TRUE_STR).to()).unwrap();
        let false_str: ObjectRef = rt.alloc(StringObject::from_str(FALSE_STR).to()).unwrap();

        let True = rt.OldTrue();
        let False = rt.OldFalse();

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

        let True = rt.OldTrue();
        let False = rt.OldFalse();

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


#[cfg(all(feature="old", test))]
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
