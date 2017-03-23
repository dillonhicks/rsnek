use std;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use object;
use runtime::Runtime;
use result::{NativeResult, RuntimeResult};
use error::{Error, ErrorType};
use object::model::PyBehavior;

use typedef::objectref;
use typedef::objectref::{RtObject, ObjectRef};
use typedef::boolean::{PyBoolean, BooleanObject};
use typedef::integer::IntegerObject;
use typedef::float::FloatObject;
use typedef::string::StringObject;
use typedef::tuple::TupleObject;
use typedef::list::ListObject;
use typedef::complex::ComplexObject;
use typedef::set::SetObject;
use typedef::frozenset::FrozenSetObject;
use typedef::dictionary::DictionaryObject;
use typedef::none::NoneType;
use typedef::native;


pub type CastResult<T: RtObject> = Result<T, Error>;


#[derive(Clone, Debug)]
pub enum Builtin {
    None(NoneType),
    Boolean(BooleanObject),
    Integer(IntegerObject),
    Float(FloatObject),
    String(StringObject),
    Tuple(TupleObject),
    List(ListObject),
    Set(SetObject),
    FrozenSet(FrozenSetObject),
    Dictionary(DictionaryObject),
    Complex(ComplexObject),

    Bool(PyBoolean),
    // Not yet implemented
    Object(()),
    Function(()),
    Method(()),
    Module(()), /*    Type(TypeObject),
                 *    Meta(MetaObject),
                 *    None(NoneObject) */
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl Builtin {
    pub fn int(&self) -> CastResult<&IntegerObject> {
        match *self {
            Builtin::Integer(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not an IntegerObject")),
        }
    }

    pub fn float(&self) -> CastResult<&FloatObject> {
        match *self {
            Builtin::Float(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a FloatObject")),
        }
    }

    pub fn tuple(&self) -> CastResult<&TupleObject> {
        match *self {
            Builtin::Tuple(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a TupleObject")),
        }
    }

    pub fn list(&self) -> CastResult<&ListObject> {
        match *self {
            Builtin::List(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a ListObject")),
        }
    }

    pub fn string(&self) -> CastResult<&StringObject> {
        match *self {
            Builtin::String(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a StringObject")),
        }
    }

    pub fn bool(&self) -> CastResult<&PyBoolean> {
        match *self {
            Builtin::Bool(ref obj) => Ok(obj),
            _ => Err(Error::typerr("Not a PyBoolean")),
        }
    }

}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     Python Object Traits
///
/// For the BuiltinObject this should mean just proxy dispatching the
/// underlying associated function using the foreach macros.
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::identity::DefaultIdentity for Builtin {}
impl object::method::Id for Builtin {}
impl object::method::Is for Builtin {}
impl object::method::IsNot for Builtin {}


/// old
impl objectref::RtObject for Builtin {}
impl object::model::PyObject for Builtin {}
impl object::model::PyBehavior for Builtin {

    //
    // Identity and Equality
    //

    fn identity(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, identity, lhs)
    }

    /// Short circuit the ident to hit the wrapper since
    /// the macro unwrapping causes an extra layer of indirection
    /// and makes comparing porinters at the Object level harder.
    //
    //     fn native_identity(&self) -> native::ObjectId {
    //        native_foreach_builtin!(self, native_identity, lhs)
    //        //return (&self as *const _) as native::ObjectId;
    //     }

    fn op_is(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is, lhs, rhs)
    }

    fn op_is_not(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_eq, lhs, rhs)
    }

    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ne, lhs, rhs)
    }

    fn native_is(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is, lhs, rhs)
    }

    fn native_is_not(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_eq, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_ne, lhs, rhs)
    }


    //
    // Hash
    //
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId>{
        native_foreach_builtin!(self, native_hash, obj)
    }

    //
    // Numeric Conversions
    //
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_bool, obj)
    }

    fn native_bool(&self) -> NativeResult<native::Boolean>{
        native_foreach_builtin!(self, native_bool, obj)
    }
    
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_int, obj)
    }

    fn native_int(&self) -> NativeResult<native::Integer>{
        native_foreach_builtin!(self, native_int, obj)
    }

    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_float, obj)
    }

    fn native_float(&self) -> NativeResult<native::Float>{
        native_foreach_builtin!(self, native_float, obj)
    }

    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_complex, obj)
    }

    fn native_complex(&self) -> NativeResult<native::Complex>{
        native_foreach_builtin!(self, native_complex, obj)
    }

    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_index, obj)
    }

    fn native_index(&self) -> NativeResult<native::Integer>{
        native_foreach_builtin!(self, native_index, obj)
    }

    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_str, obj)
    }

    fn native_str(&self) -> NativeResult<native::String>{
        native_foreach_builtin!(self, native_str, obj)
    }

    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_repr, obj)
    }

    fn native_repr(&self) -> NativeResult<native::String>{
        native_foreach_builtin!(self, native_repr, obj)
    }

    // Numeric operators
    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin>{
        native_foreach_builtin!(self, native_add, lhs, rhs)
    }


    //
    // Collections
    //
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_len, lhs)
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        native_foreach_builtin!(self, native_len, lhs)
    }

    fn op_setitem(&self, rt: &Runtime, name: &ObjectRef, item: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_setitem, object, name, item)
    }

    fn native_setitem(&self, name: &Builtin, item: &Builtin) -> NativeResult<native::NoneValue> {
        native_foreach_builtin!(self, native_setitem, object, name, item)
    }
    
    fn op_getitem(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_getitem, object, name)
    }

    fn native_getitem(&self, name: &Builtin) -> NativeResult<Builtin> {
        native_foreach_builtin!(self, native_getitem, object, name)
    }
}


impl objectref::ToRtWrapperType<Builtin> for Builtin {
    #[inline]
    fn to(self) -> Builtin {
        self
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for Builtin {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self)
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//     stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl std::cmp::PartialEq for Builtin {
    fn eq(&self, rhs: &Builtin) -> bool {
        self.native_eq(rhs).unwrap()
    }
}

impl std::cmp::Eq for Builtin {}


impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Builtin::Integer(ref obj) => write!(f, "{}", obj),
            &Builtin::Float(ref obj) => write!(f, "{}", obj),
            &Builtin::String(ref obj) => write!(f, "{}", obj),
            &Builtin::Tuple(ref obj) => write!(f, "{}", obj),
            &Builtin::List(ref obj) => write!(f, "{}", obj),
            _ => write!(f, "BuiltinType"),
        }
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