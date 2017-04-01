/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::rc::Rc;
use num::Zero;

use object::typing::BuiltinType;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::none::{PyNoneType, NONE};
use typedef::boolean::PyBooleanType;
use typedef::integer::PyIntegerType;
use typedef::string::PyStringType;
use typedef::dictionary::PyDictType;
use typedef::object::PyObjectType;

pub trait NoneProvider {
    fn none(&self) -> ObjectRef;
}

pub trait BooleanProvider<T> {
    fn bool(&self, value: T) -> ObjectRef;
}

pub trait IntegerProvider<T> {
    fn int(&self, value: T) -> ObjectRef;
}

pub trait DictProvider<T> {
    fn dict(&self, value: T) -> ObjectRef;
}

pub trait StringProvider<T> {
    fn str(&self, value: T) -> ObjectRef;
}

pub trait ObjectProvider<T> {
    fn object(&self, value: T) -> ObjectRef;
}

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


/// Well Known builtin types
pub struct BuiltinTypes {
    none: PyNoneType,
    bool: PyBooleanType,
    int: PyIntegerType,
    dict: PyDictType,
    string: PyStringType,
    object: PyObjectType
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
            string: PyStringType::init_type(),
            object: PyObjectType::init_type(),
        };

        let internal = RuntimeInternal {
            types: builtins,
        };


        Runtime(Rc::new(Box::new(internal)))
    }

    // Type Convenience Methods for Builtin Types


    #[inline(always)]
    pub fn bool_true(&self) -> ObjectRef {
        self.bool(true)
    }

    #[inline(always)]
    pub fn bool_false(&self) -> ObjectRef {
        self.bool(false)
    }

}

//
// None
//
impl NoneProvider for Runtime {
    #[inline]
    fn none(&self) -> ObjectRef {
        self.0.types.none.new(&self, NONE)
    }
}

//
// Boolean
//
impl BooleanProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bool(&self, value: native::None) -> ObjectRef {
        self.0.types.bool.new(&self, false)
    }
}

impl BooleanProvider<native::Boolean> for Runtime {
    fn bool(&self, value: native::Boolean) -> ObjectRef {
        self.0.types.bool.new(&self, value)
    }
}

//
// Integer
//

impl IntegerProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn int(&self, value: native::None) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::zero())
    }
}


impl IntegerProvider<native::Integer> for Runtime {
    fn int(&self, value: native::Integer) -> ObjectRef {
        self.0.types.int.new(&self, value)
    }
}

impl IntegerProvider<native::ObjectId> for Runtime {
    fn int(&self, value: native::ObjectId) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from(value))
    }
}

impl IntegerProvider<i32> for Runtime {
    fn int(&self, value: i32) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from(value))
    }
}


//
// String
//

impl StringProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::None) -> ObjectRef {
        return self.0.types.string.empty.clone()
    }
}

impl StringProvider<native::String> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::String) -> ObjectRef {
        return self.0.types.string.new(&self, value)
    }
}

impl StringProvider<&'static str> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: &'static str) -> ObjectRef {
        return self.0.types.string.new(&self, value.to_string())
    }
}

//
// Dict
//
impl DictProvider<native::Dict> for Runtime {
    fn dict(&self, value: native::Dict) -> ObjectRef {
        self.0.types.dict.new(&self, value)
    }
}

impl DictProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn dict(&self, value: native::None) -> ObjectRef {
        self.0.types.dict.new(&self, native::Dict::new())
    }
}

//
// Object
//

impl ObjectProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::None) -> ObjectRef {
        self.0.types.object.new(&self, native::Object {
            dict: self.dict(native::None()),
            bases: self.dict(native::None())
        })
    }
}


impl ObjectProvider<native::Object> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::Object) -> ObjectRef {
        self.0.types.object.new(&self, value)
    }
}

// stdlib
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
