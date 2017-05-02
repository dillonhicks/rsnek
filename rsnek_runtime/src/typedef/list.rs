use std::fmt;
use std::ops::{Add, Deref};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use itertools::Itertools;
use num::{ToPrimitive, Zero};

use ::resource::strings;
use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{BooleanProvider, IntegerProvider, IteratorProvider, DefaultListProvider, ListProvider};
use object::{self, RtValue, typing, PyAPI};
use object::method::{self, Id, Length};
use object::selfref::{self, SelfRef};

use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;


pub struct PyListType {
    pub empty: ObjectRef,
}


impl typing::BuiltinType for PyListType {
    type T = PyList;
    type V = native::List;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyListType::inject_selfref(PyListType::alloc(value))
    }

    fn init_type() -> Self {
        PyListType { empty: PyListType::inject_selfref(PyListType::alloc(native::List::new())) }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::List(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::List(ref list) => {
                list.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyList {
            value: ListValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}

pub struct ListValue(pub native::List);
pub type PyList = RtValue<ListValue>;


impl PyAPI for PyList {}
impl method::New for PyList {}
impl method::Init for PyList {}
impl method::Delete for PyList {}
impl method::GetAttr for PyList {}
impl method::GetAttribute for PyList {}
impl method::SetAttr for PyList {}
impl method::DelAttr for PyList {}
impl method::Hashed for PyList {}
impl method::StringCast for PyList {}
impl method::BytesCast for PyList {}
impl method::StringFormat for PyList {}
impl method::StringRepresentation for PyList {}
impl method::Equal for PyList {}
impl method::NotEqual for PyList {}
impl method::LessThan for PyList {}
impl method::LessOrEqual for PyList {}
impl method::GreaterOrEqual for PyList {}
impl method::GreaterThan for PyList {}
impl method::BooleanCast for PyList {}
impl method::IntegerCast for PyList {}
impl method::FloatCast for PyList {}
impl method::ComplexCast for PyList {}
impl method::Rounding for PyList {}
impl method::Index for PyList {}
impl method::NegateValue for PyList {}
impl method::AbsValue for PyList {}
impl method::PositiveValue for PyList {}
impl method::InvertValue for PyList {}
impl method::Add for PyList {}
impl method::BitwiseAnd for PyList {}
impl method::DivMod for PyList {}
impl method::FloorDivision for PyList {}
impl method::LeftShift for PyList {}
impl method::Modulus for PyList {}
impl method::Multiply for PyList {}
impl method::MatrixMultiply for PyList {}
impl method::BitwiseOr for PyList {}
impl method::Pow for PyList {}
impl method::RightShift for PyList {}
impl method::Subtract for PyList {}
impl method::TrueDivision for PyList {}
impl method::XOr for PyList {}
impl method::ReflectedAdd for PyList {}
impl method::ReflectedBitwiseAnd for PyList {}
impl method::ReflectedDivMod for PyList {}
impl method::ReflectedFloorDivision for PyList {}
impl method::ReflectedLeftShift for PyList {}
impl method::ReflectedModulus for PyList {}
impl method::ReflectedMultiply for PyList {}
impl method::ReflectedMatrixMultiply for PyList {}
impl method::ReflectedBitwiseOr for PyList {}
impl method::ReflectedPow for PyList {}
impl method::ReflectedRightShift for PyList {}
impl method::ReflectedSubtract for PyList {}
impl method::ReflectedTrueDivision for PyList {}
impl method::ReflectedXOr for PyList {}
impl method::InPlaceAdd for PyList {}
impl method::InPlaceBitwiseAnd for PyList {}
impl method::InPlaceDivMod for PyList {}
impl method::InPlaceFloorDivision for PyList {}
impl method::InPlaceLeftShift for PyList {}
impl method::InPlaceModulus for PyList {}
impl method::InPlaceMultiply for PyList {}
impl method::InPlaceMatrixMultiply for PyList {}
impl method::InPlaceBitwiseOr for PyList {}
impl method::InPlacePow for PyList {}
impl method::InPlaceRightShift for PyList {}
impl method::InPlaceSubtract for PyList {}
impl method::InPlaceTrueDivision for PyList {}
impl method::InPlaceXOr for PyList {}
impl method::Contains for PyList {}
impl method::Iter for PyList {}
impl method::Call for PyList {}
impl method::Length for PyList {
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
impl method::LengthHint for PyList {}
impl method::Next for PyList {}
impl method::Reversed for PyList {}
impl method::GetItem for PyList {}
impl method::SetItem for PyList {}
impl method::DeleteItem for PyList {}
impl method::Count for PyList {}
impl method::Append for PyList {}
impl method::Extend for PyList {}
impl method::Pop for PyList {}
impl method::Remove for PyList {}
impl method::IsDisjoint for PyList {}
impl method::AddItem for PyList {}
impl method::Discard for PyList {}
impl method::Clear for PyList {}
impl method::Get for PyList {}
impl method::Keys for PyList {}
impl method::Values for PyList {}
impl method::Items for PyList {}
impl method::PopItem for PyList {}
impl method::Update for PyList {}
impl method::SetDefault for PyList {}
impl method::Await for PyList {}
impl method::Send for PyList {}
impl method::Throw for PyList {}
impl method::Close for PyList {}
impl method::Exit for PyList {}
impl method::Enter for PyList {}
impl method::DescriptorGet for PyList {}
impl method::DescriptorSet for PyList {}
impl method::DescriptorSetName for PyList {}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for PyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List({:?})", self.value.0)
    }
}

impl fmt::Debug for PyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List({:?})", self.value.0)
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use ::traits::{NoneProvider, DefaultListProvider};
    use super::*;

    fn setup() -> (Runtime,) {
        (Runtime::new(),)
    }

    #[test]
    fn new_default() {
        let (rt,) = setup();
        rt.default_list();
    }

    #[test]
    fn __len__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();
        let boxed: &Box<Builtin> = list.0.borrow();

        let len = boxed.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(0));
        let len = boxed.native_len().unwrap();
        assert_eq!(len, native::Integer::zero());

        // Three Elements
        let list = rt.list(vec![rt.none(), rt.none(), rt.none()]);
        let boxed: &Box<Builtin> = list.0.borrow();

        let len = boxed.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(3));
        let len = boxed.native_len().unwrap();
        assert_eq!(len, native::Integer::from(3));
    }

}

#[cfg(all(feature="old", test))]
mod old {
    #[derive(Clone)]
    pub struct List(RefCell<native::List>);

    #[derive(Clone)]
    pub struct ListObject {
        pub value: List,
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Struct Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl List {
        fn new(vector: Vec<ObjectRef>) -> List {
            List(RefCell::new(vector))
        }
    }


    impl ListObject {
        pub fn new(value: &Vec<ObjectRef>) -> ListObject {
            let list = ListObject { value: List::new(value.clone()) };

            return list;
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Python Object Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl objectref::RtObject for ListObject {}

    impl object::model::PyObject for ListObject {}

    impl object::model::PyBehavior for ListObject {
        fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            let builtin: &Box<Builtin> = rhs.0.borrow();

            match builtin.deref() {
                &Builtin::List(ref obj) => {
                    // Modify the new to allow runtime to give weakrefs back to self
                    let lhs_cell: Ref<Vec<ObjectRef>> = self.value.0.borrow();
                    let lhs_borrow: &Vec<ObjectRef> = lhs_cell.borrow().deref();

                    // Inc refcounts for the objects transferred over to self's List
                    let rhs_borrow = obj.value.0.borrow();

                    let mut new_list: Vec<ObjectRef> = Vec::with_capacity(lhs_borrow.len() + rhs_borrow.len());
                    lhs_borrow.iter().map(|objref| new_list.push(objref.clone()));
                    rhs_borrow.iter().map(|objref| new_list.push(objref.clone()));


                    /// DUMB DUMB DUMB THIS IS A COPY AND NOT THE REF TO THE ORIGINAL LIST!!!
                    let l: ObjectRef = ListObject::new(&new_list).to();
                    rt.alloc(l)
                }
                _ => Err(Error(ErrorType::Type, "TypeError cannot add to List")),
            }
        }
    }


    impl objectref::ToRtWrapperType<Builtin> for ListObject {
        #[inline]
        fn to(self) -> Builtin {
            return Builtin::List(self);
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for ListObject {
        #[inline]
        fn to(self) -> ObjectRef {
            ObjectRef::new(self.to())
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.0.borrow())
        }
    }

    impl fmt::Display for ListObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl fmt::Debug for ListObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }


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
