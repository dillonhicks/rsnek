use std::fmt;
use std::borrow::Borrow;
use std::ops::Deref;

use num::Zero;
use num::ToPrimitive;

use runtime::{Runtime, BooleanProvider, StringProvider, IntegerProvider, FloatProvider};
use error::Error;
use result::{NativeResult, RuntimeResult};
use object::{self, RtValue, method, typing};
use object::selfref::{self, SelfRef};

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::number::{self, FloatAdapter, IntAdapter};


#[derive(Clone)]
pub struct PyFloatType {}


impl typing::BuiltinType for PyFloatType {
    type T = PyFloat;
    type V = native::Float;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: native::Float) -> ObjectRef {
        // TODO: {T99} Investigate object interning, see the methodology in integer.rs.
        // Can that be generalized?
        PyFloatType::inject_selfref(PyFloatType::alloc(value))
    }

    fn init_type() -> Self {
        PyFloatType {}
    }

    fn inject_selfref(value: PyFloat) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Float(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Float(ref int) => {
                int.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyFloat {
            value: FloatValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


impl method::New for PyFloatType {}
impl method::Init for PyFloatType {}
impl method::Delete for PyFloatType {}


pub struct FloatValue(pub native::Float);
pub type PyFloat = RtValue<FloatValue>;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::PyAPI for PyFloat {}
impl method::New for PyFloat {}
impl method::Init for PyFloat {}
impl method::Delete for PyFloat {}
impl method::GetAttr for PyFloat {}
impl method::GetAttribute for PyFloat {}
impl method::SetAttr for PyFloat {}
impl method::DelAttr for PyFloat {}
impl method::Id for PyFloat {}
impl method::Is for PyFloat {}
impl method::IsNot for PyFloat {}
impl method::Hashed for PyFloat {
    // TODO: {T87} python has its own algo for hashing floats ensure to look at that for compat.
}

impl method::StringCast for PyFloat {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_str() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(number::format_float(&self.value.0))
    }
}

impl method::BytesCast for PyFloat {}
impl method::StringFormat for PyFloat {}
impl method::StringRepresentation for PyFloat {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_repr() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        Ok(number::format_float(&self.value.0))
    }
}

impl method::Equal for PyFloat {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => Ok(rt.bool(value)),
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match *other {
            Builtin::Float(ref float) => Ok(self.value.0 == float.value.0),
            Builtin::Int(ref int) => Ok(FloatAdapter(&self.value.0) == IntAdapter(&int.value.0)),
            _ => Ok(false),
        }
    }
}
impl method::NotEqual for PyFloat {}
impl method::LessThan for PyFloat {}
impl method::LessOrEqual for PyFloat {}
impl method::GreaterOrEqual for PyFloat {}
impl method::GreaterThan for PyFloat {}
impl method::BooleanCast for PyFloat {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        if self.native_bool().unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.0.is_zero());
    }
}

impl method::IntegerCast for PyFloat {
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => Ok(rt.int(int)),
            _ => unreachable!()
        }
    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        return Ok(native::Integer::from(self.value.0 as i64));
    }
}
impl method::FloatCast for PyFloat {
    #[allow(unused_variables)]
    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        self.rc.upgrade()
    }

    fn native_float(&self) -> NativeResult<native::Float> {
        return Ok(self.value.0);
    }
}
impl method::ComplexCast for PyFloat {}
impl method::Rounding for PyFloat {}
impl method::Index for PyFloat {}
impl method::NegateValue for PyFloat {}
impl method::AbsValue for PyFloat {}
impl method::PositiveValue for PyFloat {}
impl method::InvertValue for PyFloat {}
impl method::Add for PyFloat {
    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref(){
            &Builtin::Float(ref rhs) => {
                // TODO: {T103} Use checked arithmetic where appropriate... this is not the only
                // example. But the float (and some int) methods are likely to be the highest
                // frequency.
                Ok(rt.float(self.value.0 + rhs.value.0))
            }
            &Builtin::Int(ref rhs) => {
                match rhs.value.0.to_f64() {
                    Some(float) => Ok(rt.float(self.value.0 + float)),
                    None => Err(Error::overflow(&format!("{:?} + {} overflows", self.value.0, rhs.value.0))),
                }
            }
            other => Err(Error::typerr(&format!("Cannot add {} to float", other.debug_name()))),
        }
    }

}

impl method::BitwiseAnd for PyFloat {}
impl method::DivMod for PyFloat {}
impl method::FloorDivision for PyFloat {}
impl method::LeftShift for PyFloat {}
impl method::Modulus for PyFloat {}
impl method::Multiply for PyFloat {}
impl method::MatrixMultiply for PyFloat {}
impl method::BitwiseOr for PyFloat {}
impl method::Pow for PyFloat {}
impl method::RightShift for PyFloat {}
impl method::Subtract for PyFloat {}
impl method::TrueDivision for PyFloat {}
impl method::XOr for PyFloat {}
impl method::ReflectedAdd for PyFloat {}
impl method::ReflectedBitwiseAnd for PyFloat {}
impl method::ReflectedDivMod for PyFloat {}
impl method::ReflectedFloorDivision for PyFloat {}
impl method::ReflectedLeftShift for PyFloat {}
impl method::ReflectedModulus for PyFloat {}
impl method::ReflectedMultiply for PyFloat {}
impl method::ReflectedMatrixMultiply for PyFloat {}
impl method::ReflectedBitwiseOr for PyFloat {}
impl method::ReflectedPow for PyFloat {}
impl method::ReflectedRightShift for PyFloat {}
impl method::ReflectedSubtract for PyFloat {}
impl method::ReflectedTrueDivision for PyFloat {}
impl method::ReflectedXOr for PyFloat {}
impl method::InPlaceAdd for PyFloat {}
impl method::InPlaceBitwiseAnd for PyFloat {}
impl method::InPlaceDivMod for PyFloat {}
impl method::InPlaceFloorDivision for PyFloat {}
impl method::InPlaceLeftShift for PyFloat {}
impl method::InPlaceModulus for PyFloat {}
impl method::InPlaceMultiply for PyFloat {}
impl method::InPlaceMatrixMultiply for PyFloat {}
impl method::InPlaceBitwiseOr for PyFloat {}
impl method::InPlacePow for PyFloat {}
impl method::InPlaceRightShift for PyFloat {}
impl method::InPlaceSubtract for PyFloat {}
impl method::InPlaceTrueDivision for PyFloat {}
impl method::InPlaceXOr for PyFloat {}
impl method::Contains for PyFloat {}
impl method::Iter for PyFloat {}
impl method::Call for PyFloat {}
impl method::Length for PyFloat {}
impl method::LengthHint for PyFloat {}
impl method::Next for PyFloat {}
impl method::Reversed for PyFloat {}
impl method::GetItem for PyFloat {}
impl method::SetItem for PyFloat {}
impl method::DeleteItem for PyFloat {}
impl method::Count for PyFloat {}
impl method::Append for PyFloat {}
impl method::Extend for PyFloat {}
impl method::Pop for PyFloat {}
impl method::Remove for PyFloat {}
impl method::IsDisjoint for PyFloat {}
impl method::AddItem for PyFloat {}
impl method::Discard for PyFloat {}
impl method::Clear for PyFloat {}
impl method::Get for PyFloat {}
impl method::Keys for PyFloat {}
impl method::Values for PyFloat {}
impl method::Items for PyFloat {}
impl method::PopItem for PyFloat {}
impl method::Update for PyFloat {}
impl method::SetDefault for PyFloat {}
impl method::Await for PyFloat {}
impl method::Send for PyFloat {}
impl method::Throw for PyFloat {}
impl method::Close for PyFloat {}
impl method::Exit for PyFloat {}
impl method::Enter for PyFloat {}
impl method::DescriptorGet for PyFloat {}
impl method::DescriptorSet for PyFloat {}
impl method::DescriptorSetName for PyFloat {}


// ---------------
//  stdlib traits
// ---------------


impl fmt::Display for PyFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}

impl fmt::Debug for PyFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


#[cfg(all(feature="old", test))]
mod old {
    #[derive(Clone, Debug)]
    pub struct FloatObject {
        pub value: Float,
    }


    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+
    ///      Struct Traits
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl FloatObject {
        pub fn new(value: f64) -> FloatObject {
            return FloatObject { value: value };
        }

        pub fn add_integer(float: &FloatObject, integer: &IntegerObject) -> CastResult<FloatObject> {
            match integer.value.to_f64() {
                Some(other) => Ok(FloatObject::new(float.value + other)),
                None => Err(Error(ErrorType::Overflow, "Floating Point Overflow")),
            }
        }
    }

    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+
    ///    Python Object Traits
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl objectref::RtObject for FloatObject {}

    impl object::model::PyObject for FloatObject {}

    impl object::model::PyBehavior for FloatObject {
        fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            let builtin: &Box<Builtin> = rhs.0.borrow();

            match builtin.deref() {
                &Builtin::Float(ref obj) => {
                    let new_number = FloatObject::new(self.value + obj.value);
                    let num_ref: ObjectRef = new_number.to();
                    runtime.alloc(num_ref)
                }
                &Builtin::Integer(ref obj) => {
                    let new_number = FloatObject::add_integer(&self, &obj)?;
                    let num_ref: ObjectRef = new_number.to();
                    runtime.alloc(num_ref)
                }
                _ => Err(Error(ErrorType::Type, "TypeError cannot add to float")),
            }
        }
    }


    impl objectref::ToRtWrapperType<Builtin> for FloatObject {
        #[inline]
        fn to(self) -> Builtin {
            return Builtin::Float(self);
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for FloatObject {
        #[inline]
        fn to(self) -> ObjectRef {
            ObjectRef::new(self.to())
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //      stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl fmt::Display for FloatObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //          Tests
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
