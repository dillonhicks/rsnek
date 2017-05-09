//! Union of all traits used to define the Python Object API. See: `PyAPI`.
//!
use std::borrow::Borrow;

use ::api::RtObject;
use ::api::result::{Error, ObjectResult, RtResult};
use ::runtime::Runtime;
use ::runtime::traits::{BooleanProvider, IntegerProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Native};
use ::system::primitives as rs;



/// Metaclass instance creator
/// object.__new__
/// 
/// 
/// ```python
/// a.__new__()
/// ```
///
api_trait!(4ary, self, __new__, New, op_new, native_new);

/// object.__init__ - Class constructor generally gets passed the instance created in __new__
///
/// ```python
/// class A:
///     pass
///
/// A()
/// ```
///
#[allow(unused_variables)]
pub trait Init {
    fn op_init(&mut self, rt: &Runtime, named_args: &RtObject, args: &RtObject, kwargs: &RtObject) -> ObjectResult {
        Err(Error::not_implemented())
    }
    fn native_init(&mut self, named_args: &Type, args: &Type, kwargs: &Type) -> RtResult<rs::None> {
        Err(Error::not_implemented())
    }
}

/// object.__del__ - Trait to define a destructor.
///
/// ```python
/// del a
/// ```
///
api_trait!(binary, self, __del__, Delete, op_del, native_del);


#[inline(always)]
pub fn memory_address<T>(data: &T) -> rs::ObjectId {
    (data as *const _) as rs::ObjectId
}


/// object.__getattr__
/// The native api must always return `RtObject`
/// 
/// ```python
/// a.b
/// ```
///
api_trait!(binary, self, __getattr__, GetAttr, op_getattr, native_getattr, RtObject);

/// object.__getattribute__
///
/// Called when __getattr__ fails.
/// 
/// ```python
/// a.b
/// ```
///
api_trait!(binary, self, __getattribute__, GetAttribute, op_getattribute, native_getattribute);

/// object.__setattr__
/// The native api must always return `rs::None`
/// 
/// ```python
/// 
/// ```
/// 
api_trait!(ternary, self, __setattr__, SetAttr, op_setattr, native_setattr, rs::None);

/// object.__delattr__
/// 
/// 
/// ```python
/// 
/// ```
///
api_trait!(binary, self, __delattr__, DelAttr, op_delattr, native_delattr);


//
// Identity and Hashing
//
//  Note that the Id and Is(Not?) are not directly
//  represented at runtime except through the `id()`
//  and `is / is not` keyword operators.
//

/// `Id`
/// The native api must always return `rs::ObjectId`
/// 
/// ```python
/// id(a)
/// ```
///
pub trait Id {
    fn op_id(&self, rt: &Runtime) -> ObjectResult {
        Ok(rt.int(self.native_id()))
    }

    fn native_id(&self) -> rs::ObjectId {
        (&self as *const _) as rs::ObjectId
    }
}

/// `Is` - Test for reference equality
/// The native api must always return `rs::Boolean`
///
/// Implementation Details:
///
/// Like CPython, the default implementation for this trait uses the memory address of the
/// backing object as a unique identifier.
///
/// ```python
/// a is b
/// ```
///
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

/// `IsNot` - Implementation Detail
/// The native api must always return `rs::Boolean`
///
/// ```python
/// a is not b
/// ```
///
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

/// object.__hash__
///
/// The native api must always return `rs::HashId`
///
/// From CPython's Docs:
///
///  Called by built-in function hash() and for operations on members of hashed collections including
///  set, frozenset, and dict. __hash__() should return an integer. The only required property is
///  that objects which compare equal have the same hash value; it is advised to mix together
///  the hash values of the components of the object that also play a part in comparison
///  of objects by packing them into a tuple and hashing the tuple.
///
/// ```python
/// hash(a)
/// ```
///
api_trait!(unary, self, __hash__, Hashed, op_hash, native_hash, rs::HashId);

/// object.__string__ - Convert an object into a string
///
/// The native api must always return `rs::String`
/// 
/// ```python
/// str(a)
/// ```
///
api_trait!(unary, self, __string__, StringCast, op_str, native_str, rs::String);

/// object.__bytes__
/// The native api must always return `rs::Bytes`
/// 
/// ```python
/// bytes(a)
/// ```
///
api_trait!(unary, self, __bytes__, BytesCast, op_bytes, native_bytes, rs::Bytes);

/// object.__repr__ - Convert an object into an `eval()`-able representation
///
/// The native api must always return `rs::String`
/// 
/// ```python
/// repr(a)
/// ```
///
api_trait!(unary, self, __repr__, StringRepresentation, op_repr, native_repr, rs::String);

/// object.__format__
///
/// The native api must always return `rs::String`
/// 
/// ```python
/// 
/// ```
///
api_trait!(unary, self, __format__, StringFormat, op_format, native_format, rs::String);


/// object.__eq__ - Compare value equality
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a == b
/// ```
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

/// object.__ne__ - Compare value inequality
///
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a != b
/// ```
///
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


/// object.__lt__
/// The native api must always return `rs::Boolean`
/// ```python
/// a < b
/// ```
api_trait!(binary, self, __lt__, LessThan, op_lt, native_lt, rs::Boolean);

/// object.__le__
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a <= b
/// ```
api_trait!(binary, self, __le__, LessOrEqual, op_le, native_le, rs::Boolean);

/// object.__ge__
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a >= b
/// ```
api_trait!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge, rs::Boolean);

/// object.__gt__
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a > b
/// ```
api_trait!(binary, self, __gt__, GreaterThan, op_gt, native_gt, rs::Boolean);


/// object.__bool__
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// bool(a)
///
/// # also used in comparison tests
///
/// if a:
///     pass
///
/// ```
api_trait!(unary, self, __bool__, BooleanCast, op_bool, native_bool, rs::Boolean);

/// object.__int__
/// The native api must always return `rs::Integer`
/// 
/// ```python
/// int(a)
/// ```
api_trait!(unary, self, __int__, IntegerCast, op_int, native_int, rs::Integer);

/// object.__float__
/// The native api must always return `rs::Float`
/// 
/// ```python
/// float(a)
/// ```
api_trait!(unary, self, __float__, FloatCast, op_float, native_float, rs::Float);

/// object.__complex__
/// The native api must always return `rs::Complex`
/// 
/// ```python
/// complex(a)
/// ```
api_trait!(unary, self, __complex__, ComplexCast, op_complex, native_complex, rs::Complex);
/// object.__round__
/// The native api must always return `rs::Number`
/// 
/// ```python
/// round(a)
/// ```
api_trait!(unary, self, __round__, Rounding, op_round, native_round, rs::Number);
/// object.__index__
/// The native api must always return `rs::Integer`
/// 
/// ```python
///
/// ```
api_trait!(unary, self, __index__, Index, op_index, native_index, rs::Integer);


/// object.__neg__
/// The native api must always return `rs::Number`
/// 
/// ```python
/// -a
/// ```
api_trait!(unary, self, __neg__, NegateValue, op_neg, native_neg, rs::Number);

/// object.__abs__
/// The native api must always return `rs::Number`
/// 
/// ```python
/// abs(a)
/// ```
api_trait!(unary, self, __abs__, AbsValue, op_abs, native_abs, rs::Number);

/// object.__pos__
/// The native api must always return `rs::Number`
/// 
/// ```python
/// +a
/// ```
api_trait!(unary, self, __pos__, PositiveValue, op_pos, native_pos, rs::Number);

/// object.__invert__
/// The native api must always return `rs::Number`
/// 
/// ```python
/// ~a
/// ```
api_trait!(unary, self, __invert__, InvertValue, op_invert, native_invert, rs::Number);


/// object.__add__
/// The native api must always return `Native`
/// 
/// ```python
/// a + b
/// ```
api_trait!(binary, self, __add__, Add, op_add, native_add, Native);
/// object.__and__
/// 
/// 
/// ```python
/// a - b
/// ```
api_trait!(binary, self, __and__, BitwiseAnd, op_and, native_and);
/// object.__divmod__ - `(x//y, x%y)`
///
/// ```python
/// divmod(a, b)
/// ```
api_trait!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);

/// object.__floordiv__
///
/// ```python
/// a // b
/// ```
api_trait!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);

/// object.__lshift__
/// 
/// 
/// ```python
/// a << b
/// ```
api_trait!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);

/// object.__mod__
/// 
/// 
/// ```python
/// a % b
/// ```
api_trait!(binary, self, __mod__, Modulus, op_mod, native_mod);

/// object.__mul__
/// The native api must always return `rs::Native`
/// 
/// ```python
/// a * b
/// ```
api_trait!(binary, self, __mul__, Multiply, op_mul, native_mul, rs::Native);

/// object.__matmul__
/// 
/// 
/// ```python
/// a @ b
/// ```
api_trait!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);

/// object.__or__
/// 
/// 
/// ```python
/// a | b
/// ```
api_trait!(binary, self, __or__, BitwiseOr, op_or, native_or);

/// object.__pow__
/// 
/// 
/// ```python
/// a ** b
/// # or
/// pow(a, b, c) # a ** b % c
/// ```
api_trait!(ternary, self, __pow__, Pow, op_pow, native_pow);

/// object.__rshift__
/// 
/// 
/// ```python
/// a >> b
/// ```
api_trait!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);

/// object.__sub__
/// 
/// 
/// ```python
/// a - b
/// ```
api_trait!(binary, self, __sub__, Subtract, op_sub, native_sub);

/// object.__truediv__
/// 
/// 
/// ```python
/// a / b
/// ```
api_trait!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
/// object.__xor__
/// 
/// 
/// ```python
/// a ^ b
/// ```
api_trait!(binary, self, __xor__, XOr, op_xor, native_xor);

//
// Reflected Operators
//

/// object.__radd__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);

/// object.__rand__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rand__, ReflectedBitwiseAnd, op_rand, native_rand);

/// object.__rdivmod__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);

/// object.__rfloordiv__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);

/// object.__rlshift__
/// 
/// 
/// ```python
/// 
/// ```
///
api_trait!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);

/// object.__rmod__
/// 
/// 
/// ```python
/// 
/// ```
///
api_trait!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);

/// object.__rmul__
/// 
/// 
/// ```python
/// 
/// ```
///
api_trait!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);

/// object.__rmatmul__
/// 
/// 
/// ```python
/// 
/// ```
///
api_trait!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);

/// object.__ror__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __ror__, ReflectedBitwiseOr, op_ror, native_ror);

/// object.__rpow__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);

/// object.__rrshift__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);

/// object.__rsub__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);

/// object.__rtruediv__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);

/// object.__rxor__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

//
// Inplace Operators (+=, &=, etc.)
//

/// object.__iadd__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);

/// object.__iand__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __iand__, InPlaceBitwiseAnd, op_iand, native_iand);

/// object.__idivmod__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __idivmod__, InPlaceDivMod, op_idivmod, native_idivmod);

/// object.__ifloordiv__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);

/// object.__ilshift__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);

/// object.__imod__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);

/// object.__imul__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);

/// object.__imatmul__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);

/// object.__ior__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __ior__, InPlaceBitwiseOr, op_ior, native_ior);

/// object.__ipow__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);

/// object.__irshift__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);

/// object.__isub__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);

/// object.__itruediv__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);

/// object.__ixor__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);



// -----------------------------------
//  Collections
// -----------------------------------
/// object.__contains__
/// The native api must always return `rs::Boolean`
///
/// Note the operands are reversed.
///
/// ```python
/// b in a
/// ```
api_trait!(binary, self, __contains__, Contains, op_contains, native_contains, rs::Boolean);

/// object.__iter__
/// The native api must always return `rs::Iterator`
/// 
/// ```python
/// iter(a)
///
/// # also
///
/// for elem in a:
///     pass
/// ```
api_trait!(unary, self, __iter__, Iter, op_iter, native_iter, rs::Iterator);

/// object.__call__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(4ary, self, __call__, Call, op_call, native_call);

/// object.__len__
/// The native api must always return `rs::Integer`
/// 
/// ```python
/// len(a)
/// ```
api_trait!(unary, self, __len__, Length, op_len, native_len, rs::Integer);

/// object.__length_hint__
/// The native api must always return `rs::Integer`
/// 
/// ```python
/// 
/// ```
api_trait!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint, rs::Integer);

/// object.__next__
/// The native api must always return `RtObject`
/// 
/// ```python
/// next(a)
/// ```
/// object.__next__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(unary, self, __next__, Next, op_next, native_next, RtObject);

/// object.__reversed__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(unary, self, __reversed__, Reversed, op_reversed, native_reversed);

/// __getitem__
///
/// ```python
/// x[item]
/// ```
/// object.__getitem__
/// The native api must always return `RtObject`
/// 
/// ```python
/// 
/// ```
/// object.__getitem__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(binary, self, __getitem__, GetItem, op_getitem, native_getitem, RtObject);

/// __setitem__
///
/// ```python
/// x[item] = value
/// ```
/// object.__setitem__
/// The native api must always return `rs::None`
/// 
/// ```python
/// a[b] = c
/// ```
api_trait!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem, rs::None);

/// object.__delitem__
/// 
/// 
/// ```python
/// del a[b]
/// ```
api_trait!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);

/// object.count
/// The native api must always return `rs::Integer`
/// 
/// ```python
/// a.count(b)
/// ```
api_trait!(binary, self, count, Count, meth_count, native_meth_count, rs::Integer);

/// object.append
/// The native api must always return `rs::None`
/// 
/// ```python
/// a.append(b)
/// ```
api_trait!(binary, self, append, Append, meth_append, native_meth_append, rs::None);

/// object.extend
/// The native api must always return `rs::None`
/// 
/// ```python
/// a.extend(b)
/// ```
api_trait!(binary, self, extend, Extend, meth_extend, native_meth_extend, rs::None);

/// object.pop
/// 
/// 
/// ```python
/// a.pop(b)
/// ```
api_trait!(binary, self, pop, Pop, meth_pop, native_meth_pop);

/// object.remove
/// 
/// 
/// ```python
/// a.remove(b)
/// ```
api_trait!(binary, self, remove, Remove, meth_remove, native_meth_remove);

// Sets
/// object.isdisjoint
/// The native api must always return `rs::Boolean`
/// 
/// ```python
/// a.isdisjoint(b)
/// ```
api_trait!(binary, self, isdisjoint, IsDisjoint, meth_isdisjoint, native_meth_isdisjoint, rs::Boolean);

/// object.add
/// 
/// 
/// ```python
/// a.add(b)
/// ```
api_trait!(binary, self, add, AddItem, meth_add, native_meth_add);

/// object.discard
/// 
/// 
/// ```python
/// ```
api_trait!(unary, self, discard, Discard, meth_discard, native_meth_discard);

/// object.clear
/// 
/// 
/// ```python
/// a.clear()
/// ```
api_trait!(unary, self, clear, Clear, meth_clear, native_meth_clear);


// Mapping
/// object.get
/// 
/// 
/// ```python
/// a.get()
/// ```
api_trait!(binary, self, get, Get, meth_get, native_meth_get);

/// object.keys
/// The native api must always return `rs::Tuple`
/// 
/// ```python
/// a.keys()
/// ```
api_trait!(unary, self, keys, Keys, meth_keys, native_meth_keys, rs::Tuple);

/// object.values
/// 
/// 
/// ```python
/// a.values()
/// ```
api_trait!(unary, self, values, Values, meth_values, native_meth_values);

/// object.items
/// 
/// 
/// ```python
/// a.items()
/// ```
api_trait!(unary, self, items, Items, meth_items, native_meth_items);

/// object.popitem
/// 
/// 
/// ```python
/// a.popitem(b)
/// ```
api_trait!(binary, self, popitem, PopItem, meth_popitem, native_meth_popitem);

/// object.update
/// 
/// 
/// ```python
/// a.update(b)
/// ```
api_trait!(binary, self, update, Update, meth_update, native_meth_update);
/// object.setdefault
/// 
/// 
/// ```python
/// a.setdefault(b, c)
/// ```
api_trait!(ternary, self, setdefault, SetDefault, meth_setdefault, native_meth_setdefault);


// Generators and Coroutines
/// object.__await__
/// 
/// 
/// ```python
/// await a
/// ```
api_trait!(unary, self, __await__, Await, op_await, native_await);

/// object.send
/// 
/// 
/// ```python
/// a.send(b)
/// ```
api_trait!(binary, self, send, Send, meth_send, native_meth_send);

/// object.throw
/// 
/// 
/// ```python
/// a.throw(b)
/// ```
api_trait!(binary, self, throw, Throw, meth_throw, native_meth_throw);

/// object.close
/// 
/// 
/// ```python
/// a.close()
/// ```
api_trait!(unary, self, close, Close, meth_close, native_meth_close);

//
//  Context Managers
//

/// object.__exit__
///
/// ```python
/// with a:
///
///   pass # <- called here
/// ```
api_trait!(4ary, self, __exit__, Exit, op_exit, native_exit);

/// object.__enter__
///
/// ```python
/// with a: # <- called here
///
///   pass
/// ```
api_trait!(unary, self, __enter__, Enter, op_enter, native_enter);


//
//  Descriptors
//

/// object.__get__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(ternary, self, __get__, DescriptorGet, op_get, native_get);

/// object.__set__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(ternary, self, __set__, DescriptorSet, op_set, native_set);

/// object.__set_name__
/// 
/// 
/// ```python
/// 
/// ```
api_trait!(ternary, self, __set_name__, DescriptorSetName, op_set_name, native_set_name);
