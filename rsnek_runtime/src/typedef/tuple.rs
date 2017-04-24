use std::fmt;
use std::ops::{Add, Deref};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use itertools::Itertools;
use num::{ToPrimitive, Zero};

use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::{Runtime, IntegerProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id, Length};
use object::selfref::{self, SelfRef};

use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;


pub struct PyTupleType {
    pub empty: ObjectRef,
}


impl typing::BuiltinType for PyTupleType {
    type T = PyTuple;
    type V = native::Tuple;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyTupleType::inject_selfref(PyTupleType::alloc(value))
    }

    fn init_type() -> Self {
        PyTupleType { empty: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())) }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Tuple(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(ref tuple) => {
                tuple.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyTuple {
            value: TupleValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct TupleValue(pub native::Tuple);
pub type PyTuple = RtValue<TupleValue>;


impl object::PyAPI for PyTuple {}
impl method::New for PyTuple {}
impl method::Init for PyTuple {}
impl method::Delete for PyTuple {}
impl method::GetAttr for PyTuple {}
impl method::GetAttribute for PyTuple {}
impl method::SetAttr for PyTuple {}
impl method::DelAttr for PyTuple {}

impl method::Hashed for PyTuple {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(hashid) => Ok(rt.int(hashid)),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        if self.native_len().unwrap().is_zero() {
            let mut s = DefaultHasher::new();
            match self.rc.upgrade() {
                Ok(objref) => {
                    let boxed: &Box<Builtin> = objref.0.borrow();
                    boxed.native_id().hash(&mut s);
                    return Ok(s.finish());
                }
                Err(err) => return Err(err),
            }
        }

        self.value
            .0
            .iter()
            .map(|ref item| {
                     let boxed: &Box<Builtin> = item.0.borrow();
                     boxed.native_hash()
                 })
            .fold_results(0, Add::add)
    }
}

impl method::StringCast for PyTuple {
    fn native_str(&self) -> NativeResult<native::String> {

        let result = self.value.0.iter()
                .map(|ref item| {
                     let boxed: &Box<Builtin> = item.0.borrow();
                     return boxed.native_str()
                 })
                .fold_results("".to_string(), |acc, s| [&acc[..], &s[..]].join(", "));

        match result {
            Ok(s) => Ok(format!("({})", s)),
            Err(err) => Err(err)
        }

    }
}
impl method::BytesCast for PyTuple {}
impl method::StringFormat for PyTuple {}
impl method::StringRepresentation for PyTuple {}
impl method::Equal for PyTuple {}
impl method::NotEqual for PyTuple {}
impl method::LessThan for PyTuple {}
impl method::LessOrEqual for PyTuple {}
impl method::GreaterOrEqual for PyTuple {}
impl method::GreaterThan for PyTuple {}
impl method::BooleanCast for PyTuple {}
impl method::IntegerCast for PyTuple {}
impl method::FloatCast for PyTuple {}
impl method::ComplexCast for PyTuple {}
impl method::Rounding for PyTuple {}
impl method::Index for PyTuple {}
impl method::NegateValue for PyTuple {}
impl method::AbsValue for PyTuple {}
impl method::PositiveValue for PyTuple {}
impl method::InvertValue for PyTuple {}
impl method::Add for PyTuple {}
impl method::BitwiseAnd for PyTuple {}
impl method::DivMod for PyTuple {}
impl method::FloorDivision for PyTuple {}
impl method::LeftShift for PyTuple {}
impl method::Modulus for PyTuple {}
impl method::Multiply for PyTuple {}
impl method::MatrixMultiply for PyTuple {}
impl method::BitwiseOr for PyTuple {}
impl method::Pow for PyTuple {}
impl method::RightShift for PyTuple {}
impl method::Subtract for PyTuple {}
impl method::TrueDivision for PyTuple {}
impl method::XOr for PyTuple {}
impl method::ReflectedAdd for PyTuple {}
impl method::ReflectedBitwiseAnd for PyTuple {}
impl method::ReflectedDivMod for PyTuple {}
impl method::ReflectedFloorDivision for PyTuple {}
impl method::ReflectedLeftShift for PyTuple {}
impl method::ReflectedModulus for PyTuple {}
impl method::ReflectedMultiply for PyTuple {}
impl method::ReflectedMatrixMultiply for PyTuple {}
impl method::ReflectedBitwiseOr for PyTuple {}
impl method::ReflectedPow for PyTuple {}
impl method::ReflectedRightShift for PyTuple {}
impl method::ReflectedSubtract for PyTuple {}
impl method::ReflectedTrueDivision for PyTuple {}
impl method::ReflectedXOr for PyTuple {}
impl method::InPlaceAdd for PyTuple {}
impl method::InPlaceBitwiseAnd for PyTuple {}
impl method::InPlaceDivMod for PyTuple {}
impl method::InPlaceFloorDivision for PyTuple {}
impl method::InPlaceLeftShift for PyTuple {}
impl method::InPlaceModulus for PyTuple {}
impl method::InPlaceMultiply for PyTuple {}
impl method::InPlaceMatrixMultiply for PyTuple {}
impl method::InPlaceBitwiseOr for PyTuple {}
impl method::InPlacePow for PyTuple {}
impl method::InPlaceRightShift for PyTuple {}
impl method::InPlaceSubtract for PyTuple {}
impl method::InPlaceTrueDivision for PyTuple {}
impl method::InPlaceXOr for PyTuple {}
impl method::Contains for PyTuple {}
impl method::Iter for PyTuple {}
impl method::Call for PyTuple {}
impl method::Length for PyTuple {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_len() {
            Ok(length) => Ok(rt.int(length)),
            Err(err) => Err(err),
        }
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.len()))
    }
}
impl method::LengthHint for PyTuple {}
impl method::Next for PyTuple {}
impl method::Reversed for PyTuple {}
impl method::GetItem for PyTuple {
    /// native getitem now that we have self refs?
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, index: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = index.0.borrow();
        self.native_getitem(boxed)
    }

    fn native_getitem(&self, index: &Builtin) -> RuntimeResult {
        match index {
            &Builtin::Int(ref obj) => {
                match obj.value.0.to_usize() {
                    Some(idx) => {
                        match self.value.0.get(idx) {
                            Some(objref) => Ok(objref.clone()),
                            None => Err(Error::runtime("Index out of range")),
                        }
                    }
                    None => Err(Error::runtime("Index out of range")),
                }
            }
            _ => Err(Error::typerr("index was not an integer")),
        }
    }
}
impl method::SetItem for PyTuple {}
impl method::DeleteItem for PyTuple {}
impl method::Count for PyTuple {}
impl method::Append for PyTuple {}
impl method::Extend for PyTuple {}
impl method::Pop for PyTuple {}
impl method::Remove for PyTuple {}
impl method::IsDisjoint for PyTuple {}
impl method::AddItem for PyTuple {}
impl method::Discard for PyTuple {}
impl method::Clear for PyTuple {}
impl method::Get for PyTuple {}
impl method::Keys for PyTuple {}
impl method::Values for PyTuple {}
impl method::Items for PyTuple {}
impl method::PopItem for PyTuple {}
impl method::Update for PyTuple {}
impl method::SetDefault for PyTuple {}
impl method::Await for PyTuple {}
impl method::Send for PyTuple {}
impl method::Throw for PyTuple {}
impl method::Close for PyTuple {}
impl method::Exit for PyTuple {}
impl method::Enter for PyTuple {}
impl method::DescriptorGet for PyTuple {}
impl method::DescriptorSet for PyTuple {}
impl method::DescriptorSetName for PyTuple {}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for PyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tuple({:?})", self.value.0)
    }
}

impl fmt::Debug for PyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tuple({:?})", self.value.0)
    }
}


#[cfg(test)]
mod _api_method {
    use runtime::{TupleProvider, BooleanProvider};
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let tuple = rt.tuple(native::None());
        let tuple2 = tuple.clone();
        let tuple3 = rt.tuple(vec![rt.tuple(native::None())]);

        let boxed: &Box<Builtin> = tuple.0.borrow();

        let result = boxed.op_is(&rt, &tuple2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = boxed.op_is(&rt, &tuple3).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    mod __hash__ {
        use runtime::{StringProvider, IntegerProvider, DictProvider};
        use super::*;

        #[test]
        fn empty_stable() {
            let rt = setup_test();
            let tuple = rt.tuple(native::None());
            let tuple2 = tuple.clone();

            let boxed: &Box<Builtin> = tuple.0.borrow();
            let r1 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = tuple2.0.borrow();
            let r2 = boxed.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
        }

        #[test]
        fn hashable_items() {
            let rt = setup_test();
            let empty = rt.tuple(native::None());

            let tuple = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);
            let tuple2 = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);

            let boxed: &Box<Builtin> = tuple.0.borrow();
            let r1 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = tuple2.0.borrow();
            let r2 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = empty.0.borrow();
            let r3 = boxed.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
            assert!(r1 != r3);
        }

        #[test]
        #[should_panic]
        fn unhashable_items_causes_error() {
            let rt = setup_test();

            let tuple = rt.tuple(vec![rt.dict(native::None())]);
            let boxed: &Box<Builtin> = tuple.0.borrow();
            boxed.op_hash(&rt).unwrap();
        }
    }
}


#[cfg(all(feature="old", test))]
mod old {
    use std;
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::fmt;
    use std::ops::Deref;
    use std::rc::{Weak, Rc};

    use num::{BigInt, FromPrimitive};

    use result::RuntimeResult;
    use runtime::Runtime;
    use error::{Error, ErrorType};
    use object;

    use super::objectref;
    use super::objectref::ObjectRef;
    use super::builtin::Builtin;

    #[derive(Clone)]
    pub struct Tuple(Box<[ObjectRef]>);


    #[derive(Clone)]
    pub struct TupleObject {
        pub value: Tuple,
    }

    impl Tuple {
        fn from_vec(vector: &Vec<ObjectRef>) -> Tuple {
            Tuple(vector.clone().into_boxed_slice())
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //      Struct Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl TupleObject {
        pub fn new(value: &Vec<ObjectRef>) -> TupleObject {
            let tuple = TupleObject { value: Tuple::from_vec(&value.clone()) };

            return tuple;
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Python Object Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl objectref::RtObject for TupleObject {}

    impl object::model::PyObject for TupleObject {}

    impl object::model::PyBehavior for TupleObject {
        fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            let borrowed: &Box<Builtin> = rhs.0.borrow();
            match borrowed.deref() {
                &Builtin::Tuple(ref obj) => {
                    let mut array = self.value
                        .0
                        .clone()
                        .into_vec();
                    array.extend_from_slice(obj.value.0.as_ref());
                    let new_tuple: ObjectRef = TupleObject::new(&array).to();
                    runtime.alloc(new_tuple)
                }
                _ => Err(Error(ErrorType::Type, "TypeError cannot add to Tuple")),
            }
        }
    }


    impl objectref::ToRtWrapperType<Builtin> for TupleObject {
        #[inline]
        fn to(self) -> Builtin {
            Builtin::Tuple(self)
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for TupleObject {
        #[inline]
        fn to(self) -> ObjectRef {
            ObjectRef::new(self.to())
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl fmt::Display for Tuple {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.0.as_ref())
        }
    }

    impl fmt::Display for TupleObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl fmt::Debug for TupleObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //         Tests
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    #[cfg(test)]
    mod impl_pybehavior {
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
        api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);

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
}
