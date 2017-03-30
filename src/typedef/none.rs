use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use runtime::Runtime;
use object::selfref::{self, SelfRef};
use object::{RtValue, PyAPI, method, typing};

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub const NONE: &'static native::None =  &native::None();
pub const NONE_STR: &'static str = "None";


pub struct PyNoneType {
    singleton_none: ObjectRef
}


impl typing::BuiltinType for PyNoneType {
    type T = PyNone;
    type V = &'static native::None;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        return self.singleton_none.clone()
    }

    fn init_type() -> Self {
        PyNoneType {
            singleton_none: PyNoneType::inject_selfref(PyNoneType::alloc(NONE))
        }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::None(value));

        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::None(ref none) => {
                none.rc.set(&objref.clone());
            },
            _ => unreachable!()
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyNone {
            value: NoneValue(value),
            rc: selfref::RefCount::default(),
        }
    }


}


pub struct NoneValue(&'static native::None);
pub type PyNone = RtValue<NoneValue>;


impl PyAPI for PyNone {}
impl method::New for PyNone {}
impl method::Init for PyNone {}
impl method::Delete for PyNone {}
impl method::GetAttr for PyNone {}
impl method::GetAttribute for PyNone {}
impl method::SetAttr for PyNone {}
impl method::DelAttr for PyNone {}
impl method::Id for PyNone {}
impl method::Is for PyNone {}
impl method::IsNot for PyNone {}
impl method::Hashed for PyNone {}
impl method::StringCast for PyNone {}
impl method::BytesCast for PyNone {}
impl method::StringFormat for PyNone {}
impl method::StringRepresentation for PyNone {}
impl method::Equal for PyNone {}
impl method::NotEqual for PyNone {}
impl method::LessThan for PyNone {}
impl method::LessOrEqual for PyNone {}
impl method::GreaterOrEqual for PyNone {}
impl method::GreaterThan for PyNone {}
impl method::BooleanCast for PyNone {}
impl method::IntegerCast for PyNone {}
impl method::FloatCast for PyNone {}
impl method::ComplexCast for PyNone {}
impl method::Rounding for PyNone {}
impl method::Index for PyNone {}
impl method::NegateValue for PyNone {}
impl method::AbsValue for PyNone {}
impl method::PositiveValue for PyNone {}
impl method::InvertValue for PyNone {}
impl method::Add for PyNone {}
impl method::BitwiseAnd for PyNone {}
impl method::DivMod for PyNone {}
impl method::FloorDivision for PyNone {}
impl method::LeftShift for PyNone {}
impl method::Modulus for PyNone {}
impl method::Multiply for PyNone {}
impl method::MatrixMultiply for PyNone {}
impl method::BitwiseOr for PyNone {}
impl method::Pow for PyNone {}
impl method::RightShift for PyNone {}
impl method::Subtract for PyNone {}
impl method::TrueDivision for PyNone {}
impl method::XOr for PyNone {}
impl method::ReflectedAdd for PyNone {}
impl method::ReflectedBitwiseAnd for PyNone {}
impl method::ReflectedDivMod for PyNone {}
impl method::ReflectedFloorDivision for PyNone {}
impl method::ReflectedLeftShift for PyNone {}
impl method::ReflectedModulus for PyNone {}
impl method::ReflectedMultiply for PyNone {}
impl method::ReflectedMatrixMultiply for PyNone {}
impl method::ReflectedBitwiseOr for PyNone {}
impl method::ReflectedPow for PyNone {}
impl method::ReflectedRightShift for PyNone {}
impl method::ReflectedSubtract for PyNone {}
impl method::ReflectedTrueDivision for PyNone {}
impl method::ReflectedXOr for PyNone {}
impl method::InPlaceAdd for PyNone {}
impl method::InPlaceBitwiseAnd for PyNone {}
impl method::InPlaceDivMod for PyNone {}
impl method::InPlaceFloorDivision for PyNone {}
impl method::InPlaceLeftShift for PyNone {}
impl method::InPlaceModulus for PyNone {}
impl method::InPlaceMultiply for PyNone {}
impl method::InPlaceMatrixMultiply for PyNone {}
impl method::InPlaceBitwiseOr for PyNone {}
impl method::InPlacePow for PyNone {}
impl method::InPlaceRightShift for PyNone {}
impl method::InPlaceSubtract for PyNone {}
impl method::InPlaceTrueDivision for PyNone {}
impl method::InPlaceXOr for PyNone {}
impl method::Contains for PyNone {}
impl method::Iter for PyNone {}
impl method::Call for PyNone {}
impl method::Length for PyNone {}
impl method::LengthHint for PyNone {}
impl method::Next for PyNone {}
impl method::Reversed for PyNone {}
impl method::GetItem for PyNone {}
impl method::SetItem for PyNone {}
impl method::DeleteItem for PyNone {}
impl method::Count for PyNone {}
impl method::Append for PyNone {}
impl method::Extend for PyNone {}
impl method::Pop for PyNone {}
impl method::Remove for PyNone {}
impl method::IsDisjoint for PyNone {}
impl method::AddItem for PyNone {}
impl method::Discard for PyNone {}
impl method::Clear for PyNone {}
impl method::Get for PyNone {}
impl method::Keys for PyNone {}
impl method::Values for PyNone {}
impl method::Items for PyNone {}
impl method::PopItem for PyNone {}
impl method::Update for PyNone {}
impl method::SetDefault for PyNone {}
impl method::Await for PyNone {}
impl method::Send for PyNone {}
impl method::Throw for PyNone {}
impl method::Close for PyNone {}
impl method::Exit for PyNone {}
impl method::Enter for PyNone {}
impl method::DescriptorGet for PyNone {}
impl method::DescriptorSet for PyNone {}
impl method::DescriptorSetName for PyNone {}



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for PyNone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PyNone")
    }
}

impl fmt::Debug for PyNone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PyNone")
    }
}


#[cfg(all(feature="old", test))]
mod old {
    #[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct NoneType(());

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Struct Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl NoneType {}


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Python Object Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl objectref::RtObject for NoneType {}

    impl object::model::PyObject for NoneType {}

    impl object::model::PyBehavior for NoneType {
        fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            let builtin: &Box<Builtin> = rhs.0.borrow();
            match self.native_eq(builtin.deref()) {
                Ok(value) => if value { Ok(rt.OldTrue()) } else { Ok(rt.OldFalse()) },
                Err(err) => Err(err),
            }
        }

        fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
            match rhs {
                &Builtin::None(ref obj) => Ok(true),
                _ => Ok(false),
            }
        }

        fn native_is(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
            match rhs {
                &Builtin::None(ref obj) => Ok(true),
                _ => Ok(false),
            }
        }

        fn native_bool(&self) -> NativeResult<native::Boolean> {
            return Ok(false);
        }

        fn op_int(&self, rt: &Runtime) -> RuntimeResult {
            Err(Error::attribute())
        }

        fn native_int(&self) -> NativeResult<native::Integer> {
            Err(Error::attribute())
        }

        fn op_float(&self, rt: &Runtime) -> RuntimeResult {
            Err(Error::attribute())
        }

        fn native_float(&self) -> NativeResult<native::Float> {
            Err(Error::attribute())
        }

        fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
            Err(Error::attribute())
        }

        fn native_complex(&self) -> NativeResult<native::Complex> {
            Err(Error::attribute())
        }

        fn op_index(&self, rt: &Runtime) -> RuntimeResult {
            Err(Error::attribute())
        }

        fn native_index(&self) -> NativeResult<native::Integer> {
            Err(Error::attribute())
        }

        fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_repr() {
                Ok(string) => rt.alloc(StringObject::new(string).to()),
                Err(err) => unreachable!(),
            }
        }

        fn native_repr(&self) -> NativeResult<native::String> {
            Ok(NONE_STR.to_string())
        }

        fn op_str(&self, rt: &Runtime) -> RuntimeResult {
            self.op_repr(rt)
        }

        fn native_str(&self) -> NativeResult<native::String> {
            return self.native_repr();
        }
    }


    impl objectref::ToRtWrapperType<Builtin> for NoneType {
        #[inline]
        fn to(self) -> Builtin {
            return Builtin::None(self);
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for NoneType {
        #[inline]
        fn to(self) -> ObjectRef {
            ObjectRef::new(self.to())
        }
    }


    #[cfg(all(feature = "old", test))]
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
    /// None is None
        #[test]
        fn is_() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();

            let result = boxed.op_is(&rt, &rt.None()).unwrap();
            assert_eq!(result, rt.OldTrue());

            let result = boxed.op_is(&rt, &rt.OldFalse()).unwrap();
            assert_eq!(result, rt.OldFalse());

            let result = boxed.op_is(&rt, &rt.OldTrue()).unwrap();
            assert_eq!(result, rt.OldFalse());
        }


        /// None == None
        #[test]
        fn __eq__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();

            let result = boxed.op_eq(&rt, &rt.None()).unwrap();
            assert_eq!(result, rt.OldTrue());

            let result = boxed.op_eq(&rt, &rt.OldFalse()).unwrap();
            assert_eq!(result, rt.OldFalse());

            let result = boxed.op_eq(&rt, &rt.OldTrue()).unwrap();
            assert_eq!(result, rt.OldFalse());
        }

        #[test]
        fn __bool__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();
            let result = boxed.op_bool(&rt).unwrap();

            assert_eq!(result, rt.OldFalse());
        }

        #[test]
        #[should_panic]
        fn __int__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();
            let result = boxed.op_int(&rt).unwrap();
        }

        #[test]
        #[should_panic]
        fn __complex__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();
            let result = boxed.op_complex(&rt).unwrap();
        }

        #[test]
        #[should_panic]
        fn __float__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();
            let result = boxed.op_float(&rt).unwrap();
        }

        #[test]
        #[should_panic]
        fn __index__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();
            let result = boxed.op_index(&rt).unwrap();
        }

        #[test]
        fn __repr__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();

            let none_str: ObjectRef = rt.alloc(StringObject::from_str(NONE_STR).to()).unwrap();
            let result = boxed.op_repr(&rt).unwrap();
            assert_eq!(result, none_str);
        }

        #[test]
        fn __str__() {
            let mut rt = Runtime::new(None);
            let none = rt.None();
            let boxed: &Box<Builtin> = none.0.borrow();

            let none_str: ObjectRef = rt.alloc(StringObject::from_str(NONE_STR).to()).unwrap();
            let result = boxed.op_str(&rt).unwrap();
            assert_eq!(result, none_str);
        }

        api_test_stub!(unary, self, __del__, Delete, op_del, native_del);
        //api_test_stub!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
        //api_test_stub!(unary, self, __str__, ToString, op_str, native_str);

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
        //api_test_stub!(unary, self, __bool__, Truth, op_bool, native_bool, native::Boolean);
        api_test_stub!(unary, self, __not__, Not, op_not, native_not, native::Boolean);
        //api_test_stub!(binary, self, is_, Is, op_is, native_is, native::Boolean);
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
        //api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
        //api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
        //api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
        api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
        //api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);
    }
}