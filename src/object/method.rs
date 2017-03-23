use std::borrow::Borrow;
use std::hash::{Hash, Hasher, SipHasher};

use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::{ObjectRef, ToRtWrapperType};
use typedef::integer::IntegerObject;


/// Big index of all traits used to define builtin objects

// ----------------------------------
// Identity and Hashing
//
//  Note that the Id and Is(Not?) are not directly
//  represented at runtime except through the `id()`
//  and `is / is not` keyword operators.
// ----------------------------------
api_trait!(unary, self, id, Id, op_id, native_id, native::ObjectId);
api_trait!(binary, self, is_, Is, op_is, native_is, native::Boolean);
api_trait!(binary, self, is_not, IsNot, op_is_not, native_is_not, native::Boolean);
api_trait!(unary, self, __hash__, Hashed, op_hash, native_hash, native::HashId);


// ----------------------------------
//  Rich Comparisons
// -----------------------------------
api_trait!(binary, self, __eq__, Equal, op_eq, native_eq);
api_trait!(binary, self, __ne__, NotEqual, op_ne, native_ne);
api_trait!(binary, self, __lt__, LessThan, op_lt, native_lt);
api_trait!(binary, self, __le__, LessOrEqual, op_le, native_le);
api_trait!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
api_trait!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

// ----------------------------------
//  Numeric Casts
// -----------------------------------
api_trait!(unary, self, __bool__, BooleanCast, op_bool, native_bool, native::Boolean);
api_trait!(unary, self, __int__, IntegerCast, op_int, native_int, native::Integer);
api_trait!(unary, self, __float__, FloatCast, op_float, native_float, native::Float);
api_trait!(unary, self, __complex__, ComplexCast, op_complex, native_complex, native::Complex);
api_trait!(unary, self, __round__, Rounding, op_round, native_round);
api_trait!(unary, self, __index__, Index, op_index, native_index, native::Integer);

// Standard unary operators
api_trait!(unary, self, __neg__, NegateValue, op_neg, native_neg);
api_trait!(unary, self, __abs__, AbsValue, op_abs, native_abs);
api_trait!(unary, self, __pos__, PositiveValue, op_pos, native_pos);
api_trait!(unary, self, __invert__, InvertValue, op_invert, native_invert);


// ----------------------------------
//  Operators
// -----------------------------------

api_trait!(binary, self, __add__, Add, op_add, native_add);
api_trait!(binary, self, __and__, BitwiseAnd, op_and, native_and);
api_trait!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
api_trait!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
api_trait!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
api_trait!(binary, self, __mod__, Modulus, op_mod, native_mod);
api_trait!(binary, self, __mul__, Multiply, op_mul, native_mul);
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
api_trait!(binary, self, __contains__, Contains, op_contains, native_contains, native::Boolean);
api_trait!(unary, self, __iter__, Iter, op_iter, native_iter);
api_trait!(variadic, self, __call__, Call, op_call, native_call);
api_trait!(unary, self, __len__, Length, op_len, native_len);
api_trait!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint, native::Integer);
api_trait!(unary, self, __next__, Next, op_next, native_next);
api_trait!(unary, self, __reversed__, Reversed, op_reversed, native_reversed);

// Sequences
api_trait!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
api_trait!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem, native::NoneValue);
api_trait!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
api_trait!(binary, self, count, Count, count, native_count, native::Integer);
api_trait!(binary, self, append, Append, append, native_append);
api_trait!(binary, self, extend, Extend, extend, native_extend);
api_trait!(binary, self, pop, Pop, pop, native_pop, native::Integer);
api_trait!(binary, self, remove, Remove, remove, native_remove);


// Sets
api_trait!(unary, self, isdisjoint, IsDisjoint, isdisjoint, native_isdisjoint, native::Boolean);
api_trait!(binary, self, add, AddItem, add, native_add);
api_trait!(unary, self, discard, Discard, discard, native_discard);
api_trait!(unary, self, clear, Clear, clear, native_clear);


// Mapping
api_trait!(binary, self, get, Get, get, native_get);
api_trait!(unary, self, keys, Keys, keys, native_keys);
api_trait!(unary, self, values, Values, values, native_values);
api_trait!(unary, self, items, Items, items, native_items);
api_trait!(binary, self, popitem, PopItem, popitem, native_popitem);
api_trait!(binary, self, update, Update, update, native_update);
api_trait!(ternary, self, setdefault, SetDefault, setdefault, native_setdefault);


// Generators and Coroutines
api_trait!(unary, self, __await__, Await, op_await, native_await);
api_trait!(binary, self, send, Send, send, native_send);
api_trait!(binary, self, throw, Throw, throw, native_throw);
api_trait!(unary, self, close, Close, close, native_close);


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
api_trait!(binary, self, __del__, DescriptorDelete, op_del, native_del);
api_trait!(ternary, self, __set_name__, DescriptorSetName, op_set_name, native_set_name);