/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::rc::Rc;
use num::FromPrimitive;
use num::Zero;

use object::typing::BuiltinType;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::none::{PyNoneType, NONE};
use typedef::boolean::PyBooleanType;
use typedef::integer::PyIntegerType;
use typedef::string::PyStringType;
use typedef::dictionary::PyDictType;


pub const STATIC_INT_IDX_OFFSET: usize = 5;
pub const STATIC_INT_RANGE: std::ops::Range<isize> = (-(STATIC_INT_IDX_OFFSET as isize)..1025);
pub const STATIC_INT_RANGE_MAX: usize = 1025 + STATIC_INT_IDX_OFFSET;

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


pub struct BuiltinTypes {
    none: PyNoneType,
    bool: PyBooleanType,
    int: PyIntegerType,
    dict: PyDictType,
    string: PyStringType
}

/// Concrete struct that holds the current runtime state, heap, etc.
/// TODO: add ability to intern objects?
struct RuntimeInternal {
    types: BuiltinTypes
}


/// Type that is the Reference Counted wrapper around the actual runtime
///
/// Patterns about References Taken from:
///  https://ricardomartins.cc/2016/06/08/interior-mutability
type RuntimeRef = Rc<Box<RuntimeInternal>>;


/// Cloning a runtime just increases the strong reference count and gives
/// back another RC'd RuntimeInternal wrapper `Runtime`.
impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime((self.0).clone())
    }
}


impl Runtime {
    pub fn new() -> Runtime {
        let builtins = BuiltinTypes {
            none: PyNoneType::init_type(),
            bool: PyBooleanType::init_type(),
            int: PyIntegerType::init_type(),
            dict: PyDictType::init_type(),
            string: PyStringType::init_type()
        };

        let internal = RuntimeInternal {
            types: builtins,
        };


        Runtime(Rc::new(Box::new(internal)))
    }

    // Type Convenience Methods for Builtin Types

    // PyNone
    #[inline(always)]
    pub fn none(&self) -> ObjectRef {
        return self.0.types.none.new(&self, NONE)
    }

    // PyInteger
    pub fn int(&self, value: native::Integer) -> ObjectRef {
        self.0.types.int.new(&self, value)
    }

    pub fn int_zero(&self) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::zero())
    }

    pub fn int_one(&self) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from_usize(1).unwrap())
    }

    // PyBoolean
    pub fn bool(&self, value: native::Boolean) -> ObjectRef {
        self.0.types.bool.new(&self, value)
    }

    pub fn bool_true(&self) -> ObjectRef {
        self.bool(true)
    }

    pub fn bool_false(&self) -> ObjectRef {
        self.bool(false)
    }

    // PyString
    pub fn str(&self, value: native::String) -> ObjectRef {
        self.0.types.string.new(&self, value)
    }

    pub fn str_empty(&self) -> ObjectRef {
        self.0.types.string.empty.clone()
    }

    pub fn dict(&self, value: native::Dict) -> ObjectRef {
        self.0.types.dict.new(&self, value)
    }
}


impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime()")
    }
}


#[cfg(all(feature="old", test))]
mod impl_runtime {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn static_integers_Zero_and_One() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.ZeroOld(), rt.ZeroOld());
        assert_eq!(rt.OneOld(), rt.OneOld());
    }

    #[test]
    fn static_int_full_range() {
        let mut rt = Runtime::new(None);
        for idx in STATIC_INT_RANGE {
            assert!(rt.IntOld(idx).is_some());
        }
    }

    #[test]
    fn static_int_bad_idx_lower_bound() {
        let mut rt = Runtime::new(None);
        assert!(rt.IntOld(-(1 + STATIC_INT_IDX_OFFSET as isize)).is_none());
    }

    #[test]
    fn static_int_bad_idx_upper_bound() {
        let mut rt = Runtime::new(None);
        assert!(rt.IntOld(1 + (STATIC_INT_RANGE_MAX as isize)).is_none());
    }

}
