use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::str::FromStr;

use num::ToPrimitive;

use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use traits::{IntegerProvider, BooleanProvider, StringProvider, DefaultStringProvider, IteratorProvider};
use error::Error;

use object::{self, RtValue};
use object::selfref::{self, SelfRef};
use object::typing::{self, BuiltinType};
use object::method;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::collection::sequence;
use resource::strings;

pub struct PyStringType {
    pub empty: ObjectRef,
}


impl typing::BuiltinType for PyStringType {
    type T = PyString;
    type V = native::String;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyStringType::inject_selfref(PyStringType::alloc(value))
    }



    fn init_type() -> Self {
        PyStringType { empty: PyStringType::inject_selfref(PyStringType::alloc("".to_string())) }
    }


    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Str(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Str(ref string) => {
                string.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }


    fn alloc(value: Self::V) -> Self::T {
        PyString {
            value: StringValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct StringValue(pub native::String);


pub type PyString = RtValue<StringValue>;


impl fmt::Debug for PyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String {{ {:?} }}", self.value.0)
    }
}




impl object::PyAPI for PyString {}
impl method::New for PyString {}
impl method::Init for PyString {}
impl method::Delete for PyString {}
impl method::GetAttr for PyString {}
impl method::GetAttribute for PyString {}
impl method::SetAttr for PyString {}
impl method::DelAttr for PyString {}
impl method::Id for PyString {}
impl method::Is for PyString {}
impl method::IsNot for PyString {}
impl method::Hashed for PyString {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => Ok(rt.int(native::Integer::from(value))),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.value.0.hash(&mut s);
        Ok(s.finish())
    }
}
impl method::StringCast for PyString {

    #[allow(unused_variables)]
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        self.rc.upgrade()
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(self.value.0.clone())
    }

}
impl method::BytesCast for PyString {}
impl method::StringFormat for PyString {}
impl method::StringRepresentation for PyString {}
impl method::Equal for PyString {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(boxed) {
            Ok(value) => Ok(rt.bool(value)),
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::Str(ref string) => Ok(self.value.0.eq(&string.value.0)),
            _ => Ok(false),
        }
    }
}
impl method::NotEqual for PyString {}
impl method::LessThan for PyString {}
impl method::LessOrEqual for PyString {}
impl method::GreaterOrEqual for PyString {}
impl method::GreaterThan for PyString {}
impl method::BooleanCast for PyString {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_bool() {
            Ok(bool) => Ok(rt.bool(bool)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.0.is_empty())
    }
}
impl method::IntegerCast for PyString {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => Ok(rt.int(int)),
            Err(err) => Err(err)
        }
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        match native::Integer::from_str(&self.value.0) {
            Ok(int) => Ok(int),
            Err(_) => Err(Error::value(
                &format!("Invalid literal '{}' for int", self.value.0)))
        }
    }

}
impl method::FloatCast for PyString {}
impl method::ComplexCast for PyString {}
impl method::Rounding for PyString {}
impl method::Index for PyString {}
impl method::NegateValue for PyString {}
impl method::AbsValue for PyString {}
impl method::PositiveValue for PyString {}
impl method::InvertValue for PyString {}
impl method::Add for PyString {

    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();
        match builtin.deref() {
            &Builtin::Str(ref other) => Ok(rt.str([&self.value.0[..], &other.value.0[..]].concat())),
            other => Err(Error::typerr(
                &strings_error_bad_operand!("+", "str", other.debug_name()))),
        }
    }

}

impl method::BitwiseAnd for PyString {}
impl method::DivMod for PyString {}
impl method::FloorDivision for PyString {}
impl method::LeftShift for PyString {}
impl method::Modulus for PyString {}
impl method::Multiply for PyString {
    fn op_mul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref int) => {
                match int.value.0.to_usize() {
                    Some(int) if int <= 0   => Ok(rt.default_str()),
                    Some(int) if int == 1   => self.rc.upgrade(),
                    Some(int)               => {
                        let value: String = (0..int)
                            .map(|_| self.value.0.clone())
                            .collect::<Vec<_>>()
                            .concat();
                        Ok(rt.str(value))
                    },
                    None                    => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("*", "str", other.debug_name())))
        }
    }

}
impl method::MatrixMultiply for PyString {}
impl method::BitwiseOr for PyString {}
impl method::Pow for PyString {}
impl method::RightShift for PyString {}
impl method::Subtract for PyString {}
impl method::TrueDivision for PyString {}
impl method::XOr for PyString {}
impl method::ReflectedAdd for PyString {}
impl method::ReflectedBitwiseAnd for PyString {}
impl method::ReflectedDivMod for PyString {}
impl method::ReflectedFloorDivision for PyString {}
impl method::ReflectedLeftShift for PyString {}
impl method::ReflectedModulus for PyString {}
impl method::ReflectedMultiply for PyString {}
impl method::ReflectedMatrixMultiply for PyString {}
impl method::ReflectedBitwiseOr for PyString {}
impl method::ReflectedPow for PyString {}
impl method::ReflectedRightShift for PyString {}
impl method::ReflectedSubtract for PyString {}
impl method::ReflectedTrueDivision for PyString {}
impl method::ReflectedXOr for PyString {}
impl method::InPlaceAdd for PyString {}
impl method::InPlaceBitwiseAnd for PyString {}
impl method::InPlaceDivMod for PyString {}
impl method::InPlaceFloorDivision for PyString {}
impl method::InPlaceLeftShift for PyString {}
impl method::InPlaceModulus for PyString {}
impl method::InPlaceMultiply for PyString {}
impl method::InPlaceMatrixMultiply for PyString {}
impl method::InPlaceBitwiseOr for PyString {}
impl method::InPlacePow for PyString {}
impl method::InPlaceRightShift for PyString {}
impl method::InPlaceSubtract for PyString {}
impl method::InPlaceTrueDivision for PyString {}
impl method::InPlaceXOr for PyString {}
impl method::Contains for PyString {
    fn op_contains(&self, rt: &Runtime, item: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        let truth = self.native_contains(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Builtin) -> NativeResult<native::Boolean> {
        match item {
            &Builtin::Str(ref string) => {
                Ok(self.value.0.contains(&string.value.0))
            },
            other => Err(Error::typerr(&format!(
                "in <string>' requires string as left operand, not {}",
                other.debug_name())))
        }
    }
}
impl method::Iter for PyString {
    fn op_iter(&self, rt: &Runtime) -> RuntimeResult {
        let iter = self.native_iter()?;
        Ok(rt.iter(iter))
    }

    fn native_iter(&self) -> NativeResult<native::Iterator> {
        match self.rc.upgrade() {
            Ok(selfref) => Ok(native::Iterator::new(&selfref)?),
            Err(err) => Err(err)
        }
    }
}

impl method::Call for PyString {}
impl method::Length for PyString {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        Ok(rt.int(self.value.0.len() as i64))
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.len()))
    }
}
impl method::LengthHint for PyString {}
impl method::Next for PyString {}
impl method::Reversed for PyString {}
impl method::GetItem for PyString {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, item: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        self.native_getitem(boxed)
    }

    fn native_getitem(&self, index: &Builtin) -> RuntimeResult {
        let substr = match index {
            &Builtin::Int(ref int) => {
                let byte = sequence::get_index(&self.value.0.as_bytes(), &int.value.0)?;
                // TODO: {T3093} Determine the best policy for strings as a sequence since any kind
                // of encoding ruins the uniform treatment of bytes as a singular index of a
                // string.
                String::from_utf8_lossy(&[byte][..]).to_string()
            }
            _ => return Err(Error::typerr("string indices must be integers")),
        };

        Ok(PyStringType::inject_selfref(PyStringType::alloc(substr)))
    }
}

impl method::SetItem for PyString {}
impl method::DeleteItem for PyString {}
impl method::Count for PyString {}
impl method::Append for PyString {}
impl method::Extend for PyString {}
impl method::Pop for PyString {}
impl method::Remove for PyString {}
impl method::IsDisjoint for PyString {}
impl method::AddItem for PyString {}
impl method::Discard for PyString {}
impl method::Clear for PyString {}
impl method::Get for PyString {}
impl method::Keys for PyString {}
impl method::Values for PyString {}
impl method::Items for PyString {}
impl method::PopItem for PyString {}
impl method::Update for PyString {}
impl method::SetDefault for PyString {}
impl method::Await for PyString {}
impl method::Send for PyString {}
impl method::Throw for PyString {}
impl method::Close for PyString {}
impl method::Exit for PyString {}
impl method::Enter for PyString {}
impl method::DescriptorGet for PyString {}
impl method::DescriptorSet for PyString {}
impl method::DescriptorSetName for PyString {}


#[cfg(all(feature="old", test))]
mod old {
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
                Ok(value) => {
                    if value {
                        Ok(rt.OldTrue())
                    } else {
                        Ok(rt.OldFalse())
                    }
                }
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
            // Refs back to self in the object holder type - this should be just a
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
}
