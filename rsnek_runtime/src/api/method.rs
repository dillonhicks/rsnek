///! Union of all traits used to define the Python Object API. See: `PyAPI`.
use std::borrow::Borrow;

use ::api::RtObject;
use ::api::result::{Error, ObjectResult, RtResult};
use ::runtime::Runtime;
use ::runtime::traits::{BooleanProvider, IntegerProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Native};
use ::system::primitives as rs;


// ----------------------------------
//  Types and Initialization
// ----------------------------------
/// Metaclass instance creator
api_trait!(4ary, self, __new__, New, op_new, native_new);
/// Class constructor generally gets passed the instance created in __new__
//api_trait!(4ary, self, __init__, Init, op_init, native_init);
#[allow(unused_variables)]
pub trait Init {
    fn op_init(&mut self, rt: &Runtime, named_args: &RtObject, args: &RtObject, kwargs: &RtObject) -> ObjectResult {
        Err(Error::not_implemented())
    }
    fn native_init(&mut self, named_args: &Type, args: &Type, kwargs: &Type) -> RtResult<rs::None> {
        Err(Error::not_implemented())
    }
}
/// Trait to define a destructor.
///
/// Shared with descriptor
api_trait!(binary, self, __del__, Delete, op_del, native_del);

// API Properties?
// api_trait!(4ary, self, __bases___, Bases, op_bases, native_bases);
// __module__
// __code__
// __doc__
// __class__

#[inline(always)]
pub fn memory_address<T>(data: &T) -> rs::ObjectId {
    (data as *const _) as rs::ObjectId
}


// ----------------------------------
//  Object
// ----------------------------------
api_trait!(binary, self, __getattr__, GetAttr, op_getattr, native_getattr, RtObject);
api_trait!(binary, self, __getattribute__, GetAttribute, op_getattribute, native_getattribute);
api_trait!(ternary, self, __setattr__, SetAttr, op_setattr, native_setattr, rs::None);
api_trait!(binary, self, __delattr__, DelAttr, op_delattr, native_delattr);


// ----------------------------------
// Identity and Hashing
//
//  Note that the Id and Is(Not?) are not directly
//  represented at runtime except through the `id()`
//  and `is / is not` keyword operators.
// ----------------------------------
// api_trait!(unary, self, id, Id, op_id, native_id, rs::ObjectId);
//

pub trait Id {
    fn op_id(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> rs::ObjectId {
        (&self as *const _) as rs::ObjectId
    }
}

pub trait Is
    where Self: Id
{
    fn op_is(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_is(&self, other: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() == other.native_id())
    }
}

pub trait IsNot
    where Self: Id
{
    fn op_is_not(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_is_not(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }


    fn native_is_not(&self, other: &Type) -> RtResult<rs::Boolean> {
        Ok(self.native_id() != other.native_id())
    }
}

/// From CPython's Docs:
/// Called by built-in function hash() and for operations on members of hashed collections including
/// set, frozenset, and dict. __hash__() should return an integer. The only required property is
/// that objects which compare equal have the same hash value; it is advised to mix together
/// the hash values of the components of the object that also play a part in comparison
/// of objects by packing them into a tuple and hashing the tuple.
api_trait!(unary, self, __hash__, Hashed, op_hash, native_hash, rs::HashId);

// ----------------------------------
//  String Formatting
// -----------------------------------
api_trait!(unary, self, __string__, StringCast, op_str, native_str, rs::String);
api_trait!(unary, self, __bytes__, BytesCast, op_bytes, native_bytes, rs::Bytes);
api_trait!(unary, self, __repr__, StringRepresentation, op_repr, native_repr, rs::String);
api_trait!(unary, self, __format__, StringFormat, op_format, native_format, rs::String);

// ----------------------------------
//  Rich Comparisons
// -----------------------------------
/// `api_trait!(binary, self, __eq__, Equal, op_eq, native_eq, rs::Boolean);`
pub trait Equal {
    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_eq(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, other: &Type) -> RtResult<rs::Boolean> {
        Ok(memory_address(&self) == other.native_id())
    }
}

/// `api_trait!(binary, self, __ne__, NotEqual, op_ne, native_ne, rs::Boolean);`
pub trait NotEqual {
    /// Default implementation of equals fallsbacks to !op_is
    fn op_ne(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_ne(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_ne(&self, other: &Type) -> RtResult<rs::Boolean> {
        Ok(memory_address(&self) != other.native_id())
    }
}

api_trait!(binary, self, __lt__, LessThan, op_lt, native_lt, rs::Boolean);
api_trait!(binary, self, __le__, LessOrEqual, op_le, native_le, rs::Boolean);
api_trait!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge, rs::Boolean);
api_trait!(binary, self, __gt__, GreaterThan, op_gt, native_gt, rs::Boolean);

// ----------------------------------
//  Numeric Casts
// -----------------------------------
api_trait!(unary, self, __bool__, BooleanCast, op_bool, native_bool, rs::Boolean);
api_trait!(unary, self, __int__, IntegerCast, op_int, native_int, rs::Integer);
api_trait!(unary, self, __float__, FloatCast, op_float, native_float, rs::Float);
api_trait!(unary, self, __complex__, ComplexCast, op_complex, native_complex, rs::Complex);
api_trait!(unary, self, __round__, Rounding, op_round, native_round, rs::Number);
api_trait!(unary, self, __index__, Index, op_index, native_index, rs::Integer);

// Standard unary operators
api_trait!(unary, self, __neg__, NegateValue, op_neg, native_neg, rs::Number);
api_trait!(unary, self, __abs__, AbsValue, op_abs, native_abs, rs::Number);
api_trait!(unary, self, __pos__, PositiveValue, op_pos, native_pos, rs::Number);
api_trait!(unary, self, __invert__, InvertValue, op_invert, native_invert, rs::Number);



// ----------------------------------
//  Operators
// -----------------------------------

api_trait!(binary, self, __add__, Add, op_add, native_add, Native);
api_trait!(binary, self, __and__, BitwiseAnd, op_and, native_and);
api_trait!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
api_trait!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
api_trait!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
api_trait!(binary, self, __mod__, Modulus, op_mod, native_mod);
api_trait!(binary, self, __mul__, Multiply, op_mul, native_mul, rs::Native);
api_trait!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);
api_trait!(binary, self, __or__, BitwiseOr, op_or, native_or);
api_trait!(ternary, self, __pow__, Pow, op_pow, native_pow);
api_trait!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);
api_trait!(binary, self, __sub__, Subtract, op_sub, native_sub);
api_trait!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
api_trait!(binary, self, __xor__, XOr, op_xor, native_xor);

// Reflected Operators
api_trait!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);
api_trait!(binary, self, __rand__, ReflectedBitwiseAnd, op_rand, native_rand);
api_trait!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);
api_trait!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);
api_trait!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);
api_trait!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);
api_trait!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);
api_trait!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);
api_trait!(binary, self, __ror__, ReflectedBitwiseOr, op_ror, native_ror);
api_trait!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);
api_trait!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);
api_trait!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);
api_trait!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);
api_trait!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

// In place operators
api_trait!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);
api_trait!(binary, self, __iand__, InPlaceBitwiseAnd, op_iand, native_iand);
api_trait!(binary, self, __idivmod__, InPlaceDivMod, op_idivmod, native_idivmod);
api_trait!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);
api_trait!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);
api_trait!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);
api_trait!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);
api_trait!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);
api_trait!(binary, self, __ior__, InPlaceBitwiseOr, op_ior, native_ior);
api_trait!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);
api_trait!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);
api_trait!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);
api_trait!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);
api_trait!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);



// -----------------------------------
//  Collections
// -----------------------------------
api_trait!(binary, self, __contains__, Contains, op_contains, native_contains, rs::Boolean);
api_trait!(unary, self, __iter__, Iter, op_iter, native_iter, rs::Iterator);
api_trait!(4ary, self, __call__, Call, op_call, native_call);
api_trait!(unary, self, __len__, Length, op_len, native_len, rs::Integer);
api_trait!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint, rs::Integer);
api_trait!(unary, self, __next__, Next, op_next, native_next, RtObject);
api_trait!(unary, self, __reversed__, Reversed, op_reversed, native_reversed);

// Sequences
api_trait!(binary, self, __getitem__, GetItem, op_getitem, native_getitem, RtObject);
api_trait!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem, rs::None);
api_trait!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
api_trait!(binary, self, count, Count, meth_count, native_meth_count, rs::Integer);
api_trait!(binary, self, append, Append, meth_append, native_meth_append, rs::None);
api_trait!(binary, self, extend, Extend, meth_extend, native_meth_extend, rs::None);
api_trait!(binary, self, pop, Pop, meth_pop, native_meth_pop);
api_trait!(binary, self, remove, Remove, meth_remove, native_meth_remove);


// Sets
api_trait!(binary, self, isdisjoint, IsDisjoint, meth_isdisjoint, native_meth_isdisjoint, rs::Boolean);
api_trait!(binary, self, add, AddItem, meth_add, native_meth_add);
api_trait!(unary, self, discard, Discard, meth_discard, native_meth_discard);
api_trait!(unary, self, clear, Clear, meth_clear, native_meth_clear);


// Mapping
api_trait!(binary, self, get, Get, meth_get, native_meth_get);
api_trait!(unary, self, keys, Keys, meth_keys, native_meth_keys, rs::Tuple);
api_trait!(unary, self, values, Values, meth_values, native_meth_values);
api_trait!(unary, self, items, Items, meth_items, native_meth_items);
api_trait!(binary, self, popitem, PopItem, meth_popitem, native_meth_popitem);
api_trait!(binary, self, update, Update, meth_update, native_meth_update);
api_trait!(ternary, self, setdefault, SetDefault, meth_setdefault, native_meth_setdefault);


// Generators and Coroutines
api_trait!(unary, self, __await__, Await, op_await, native_await);
api_trait!(binary, self, send, Send, meth_send, native_meth_send);
api_trait!(binary, self, throw, Throw, meth_throw, native_meth_throw);
api_trait!(unary, self, close, Close, meth_close, native_meth_close);


// -----------------------------------
//  Context Managers
// -----------------------------------
api_trait!(4ary, self, __exit__, Exit, op_exit, native_exit);
api_trait!(unary, self, __enter__, Enter, op_enter, native_enter);


// -----------------------------------
//  Descriptors
// -----------------------------------
api_trait!(ternary, self, __get__, DescriptorGet, op_get, native_get);
api_trait!(ternary, self, __set__, DescriptorSet, op_set, native_set);
api_trait!(ternary, self, __set_name__, DescriptorSetName, op_set_name, native_set_name);
