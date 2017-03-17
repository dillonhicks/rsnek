use result::{RuntimeResult, NativeResult};
use error::Error;
use runtime::Runtime;

use typedef::native::*;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;

pub trait UnaryOperation {}
pub trait BinaryOperation {}
pub trait TernaryOperation {}
pub trait NaryOperation {}
pub trait VariadicOperation {}

type Args<T> = Vec<T>;

//macro_rules! special_method_name {
//    ($name:tt) =>  (const PYTHON_NAME: &'static str = stringify!($name);)
//}


macro_rules! api_method {
    (unary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, rt: &mut Runtime) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
    (binary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, rhs: &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
    (ternary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, rt: &mut Runtime, arg1: &ObjectRef, arg2: &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, arg1: &Builtin, arg2: &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
   (4ary, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {

            fn $fname(&$sel, rt: &mut Runtime, arg1: &ObjectRef, arg2: &ObjectRef, arg3: &ObjectRef) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, arg1: &Builtin, arg2: &Builtin, arg3: &Builtin) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }

    };
    (variadic, $sel:ident, $pyname:ident, $tname:ident, $fname:ident, $nfname:ident) => {
            fn $fname(&$sel, rt: &mut Runtime, args: &Args<ObjectRef>) -> RuntimeResult {
                Err(Error::not_implemented())
            }

            fn $nfname(&$sel, rhs: &Args<Builtin>) -> NativeResult<Builtin> {
                Err(Error::not_implemented())
            }
    }
}

pub trait PythonObject {
    api_method!(unary, self, __del__, Delete, op_del, native_del);
    api_method!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
    api_method!(unary, self, __str__, ToString, op_str, native_str);

    /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
    api_method!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
    api_method!(binary, self, __format__, Format, op_format, native_format);


    /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
    api_method!(binary, self, __lt__, LessThan, op_lt, native_lt);
    api_method!(binary, self, __le__, LessOrEqual, op_le, native_le);
    api_method!(binary, self, __eq__, Equal, op_eq, native_eq);
    api_method!(binary, self, __ne__, NotEqual, op_ne, native_ne);
    api_method!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
    api_method!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

    // Called by built-in function hash() and for operations on members of hashed collections including
    // set, frozenset, and dict. __hash__() should return an integer. The only required property is
    // that objects which compare equal have the same hash value; it is advised to mix together
    // the hash values of the components of the object that also play a part in comparison
    // of objects by packing them into a tuple and hashing the tuple. Example:
    api_method!(unary, self, __hash__, Hashable, op_hash, native_hash);

    // Identity operators
    api_method!(unary, self, __bool__, Truth, op_truth, native_truth);
    api_method!(binary, self, __not__, Not, op_not, native_not);
    api_method!(binary, self, is_, Is, op_is, native_is);
    api_method!(binary, self, is_not, IsNot, op_is_not, native_is_not);


    // 3.3.6. Emulating container types
    api_method!(unary, self, __len__, Length, op_len, native_len);
    api_method!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint);
    api_method!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
    api_method!(binary, self, __missing__, MissingItem, op_missing, native_missing);
    api_method!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem);
    api_method!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
    api_method!(unary, self, __iter__, Iterator, op_iter, native_iter);
    api_method!(unary, self, __reversed__, Reverse, op_reverse, native_reverse);
    api_method!(binary, self, __contains__, Contains, op_contains, native_contains);


    // 3.3.7. Emulating numeric types
    //
    // The following methods can be defined to emulate numeric objects. Methods corresponding to
    // operations that are not supported by the particular kind of number implemented
    // (e.g., bitwise operations for non-integral numbers) should be left undefined.
    api_method!(binary, self, __add__, Add, op_add, native_add);
    api_method!(binary, self, __and__, And, op_and, native_and);
    api_method!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
    api_method!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
    api_method!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
    api_method!(binary, self, __mod__, Modulus, op_mod, native_mod);
    api_method!(binary, self, __mul__, Multiply, op_mul, native_mul);
    api_method!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);
    api_method!(binary, self, __or__, Or, op_or, native_or);
    api_method!(ternary, self, __pow__, Pow, op_pow, native_pow);
    api_method!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);
    api_method!(binary, self, __sub__, Subtract, op_sub, native_sub);
    api_method!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
    api_method!(binary, self, __xor__, XOr, op_xor, native_xor);

    // Reflected Operators
    api_method!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);
    api_method!(binary, self, __rand__, ReflectedAnd, op_rand, native_rand);
    api_method!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);
    api_method!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);
    api_method!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);
    api_method!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);
    api_method!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);
    api_method!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);
    api_method!(binary, self, __ror__, ReflectedOr, op_ror, native_ror);
    api_method!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);
    api_method!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);
    api_method!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);
    api_method!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);
    api_method!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

    // In place operators
    api_method!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);
    api_method!(binary, self, __iand__, InPlaceAnd, op_iand, native_iand);
    api_method!(binary, self, __rdivmod__, InPlaceDivMod, op_idivmod, native_idivmod);
    api_method!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);
    api_method!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);
    api_method!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);
    api_method!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);
    api_method!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);
    api_method!(binary, self, __ior__, InPlaceOr, op_ior, native_ior);
    api_method!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);
    api_method!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);
    api_method!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);
    api_method!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);
    api_method!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);

    // Standard unary operators
    api_method!(unary, self, __neg__, Negate, op_neg, native_neg);
    api_method!(unary, self, __abs__, Abs, op_abs, native_abs);
    api_method!(unary, self, __pos__, Positive, op_pos, native_pos);
    api_method!(unary, self, __invert__, Invert, op_invert, native_invert);

    // Standard numeric conversions
    api_method!(unary, self, __complex__, ToComplex, op_complex, native_complex);
    api_method!(unary, self, __int__, ToInt, op_int, native_int);
    api_method!(unary, self, __float__, ToFloat, op_float, native_float);
    api_method!(unary, self, __round__, ToRounded, op_round, native_round);
    api_method!(unary, self, __index__, ToIndex, op_index, native_index);
}

mod py_type_definitions {
    use super::*;
    // __init_subclass__
    // __prepare__
    // __instancecheck__
    // __subclasscheck__
    pub trait TypeModel {
        api_method!(variadic, self, __new__, New, new_type, native_new_type);
        api_method!(variadic, self, __init__, Init, init_instance, native_init_instance);
        api_method!(binary, self, __getattr__, GetAttribute, op_getattr, native_getattr);
        api_method!(binary, self, __getattribute__, StrictGetAttribute, op_getattribute, native_getattribute);
        api_method!(binary, self, __setattr__, SetAttribute, op_setattr, native_setattr);
        api_method!(binary, self, __del__, DeleteAttribute, op_delattr, native_delattr);
        api_method!(binary, self, __dir__, Directory, op_dir, native_dir);
    }

}

mod py_descriptor_protocol {
    use super::*;
    
    pub trait DescriptorModel {
        api_method!(ternary, self, __get__, DescriptorGet, op_descriptor_get, native_descriptor_get);
        api_method!(ternary, self, __set__, DescriptorSet, op_descriptor_set, native_descriptor_set);
        api_method!(binary, self, __del__, DescriptorDelete, op_descriptor_del, native_descriptor_del);
        api_method!(ternary, self, __set_name__, DescriptorSetName, op_descriptor_set_name, native_descriptor_set_name);        
    }

}

mod py_context_managers {
    use super::*;
    pub trait ContextManagerModel {
        api_method!(unary, self, __enter__, ContextEnter, op_ctx_enter, native_ctx_enter);
        api_method!(4ary, self, __exit__, ContextExit, op_ctx_exit, native_ctx_exit);
    }
}

mod py_coroutines {
    use super::*;
    pub trait CoroutineModel {
        api_method!(unary, self, __await__, CoroutineAwait, op_coro_await, native_coro_await);
        api_method!(binary, self, send, CoroutineSend, op_coro_send, native_coro_send);
        api_method!(4ary, self, throw, CoroutineThrow, op_coro_throw, native_coro_throw);
        api_method!(unary, self, close, CoroutineClose, op_coro_close, native_coro_close);
    }
}
