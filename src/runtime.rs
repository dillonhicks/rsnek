/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::rc::Rc;
use std::cell::RefCell;
use num::FromPrimitive;
use num::Zero;

pub use result::RuntimeResult;
use heap::Heap;
use object::typing::BuiltinType;

use typedef::native::ObjectId;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::boolean::{BooleanObject, PyBoolean, PyBooleanType};
use typedef::integer::{IntegerObject, PyInteger, PyIntegerType};
use typedef::string::{PyStringType};
use typedef::dictionary::{PyDict, PyDictType};
use typedef::objectref::ToRtWrapperType;
use typedef::none::NONE_TYPE;
use typedef::native;

/// If not size is given, fallback to 256kb.
pub const DEFAULT_HEAP_CAPACITY: usize = 256 * 1024;

pub const STATIC_INT_IDX_OFFSET: usize = 5;
pub const STATIC_INT_RANGE: std::ops::Range<isize> = (-(STATIC_INT_IDX_OFFSET as isize)..1025);
pub const STATIC_INT_RANGE_MAX: usize = 1025 + STATIC_INT_IDX_OFFSET;

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


pub struct BuiltinTypes {
    int: PyIntegerType,
    bool: PyBooleanType,
    dict: PyDictType,
    string: PyStringType
}

/// Concrete struct that holds the current runtime state, heap, etc.
struct RuntimeInternal {
    heap: Heap,
    singletons: SingletonIndex,
    types: BuiltinTypes
}



//noinspection RsFieldNaming
#[deprecated]
struct SingletonIndex {
    True: ObjectRef,
    False: ObjectRef,
    None: ObjectRef,
    integers: Box<[ObjectRef]>,
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
    #[inline]
    #[allow(non_snake_case)]
    pub fn new(heap_size: Option<usize>) -> Runtime {
        let size = match heap_size {
            Some(i) => i,
            None => DEFAULT_HEAP_CAPACITY,
        };

        let mut heap = Heap::new(size);

        let True: ObjectRef = BooleanObject::new_true().to();
        let False: ObjectRef = BooleanObject::new_false().to();
        let range: Vec<ObjectRef> =
            STATIC_INT_RANGE.map(|int| IntegerObject::new_i64(int as i64)).map(|obj| heap.alloc_static(obj.to()).unwrap()).collect();

        let singletons = SingletonIndex {
            True: heap.alloc_static(True).unwrap(),
            False: heap.alloc_static(False).unwrap(),
            None: heap.alloc_static(NONE_TYPE.to()).unwrap(),
            integers: range.into(),
        };

        let builtins = BuiltinTypes {
            bool: PyBooleanType::init_type(),
            int: PyIntegerType::init_type(),
            dict: PyDictType::init_type(),
            string: PyStringType::init_type()
        };

        let internal = RuntimeInternal {
            heap: heap,
            singletons: singletons,
            types: builtins,
        };


        Runtime(Rc::new(Box::new(internal)))
    }

    /// Alloc a spot for the object ref in the `Heap` for the `Runtime` this will
    /// mean that there will be at one single strong reference to the `ObjectRef`
    /// for the life of the Runtime.
    ///
    /// This gives the `Runtime` to control when the `Drop<Object>` happens
    /// and finally cleans up struct behind the `ObjectRef`.
    pub fn alloc(&self, reference: ObjectRef) -> RuntimeResult {
        self.0.heap.alloc_dynamic(reference)
    }

    pub fn heap_size(&self) -> usize {
        self.0.heap.size()
    }

    pub fn heap_capacity(&self) -> usize {
        self.0.heap.capacity()
    }

    pub fn find_object(&self, id: ObjectId) -> RuntimeResult {
        return self.0.heap.find_object(id);
    }

    #[cfg(rsnek_debug)]
    pub fn debug_references(&self) {
        (self.0.borrow_mut()).heap.print_ref_counts()
    }

    //
    // Convenience Accessors for Statically Alloc'd Values
    //

    #[allow(non_snake_case)]
    pub fn OldTrue(&self) -> ObjectRef {
        self.0
            .singletons
            .True
            .clone()
    }

    #[allow(non_snake_case)]
    pub fn OldFalse(&self) -> ObjectRef {
        self.0
            .singletons
            .False
            .clone()
    }

    #[allow(non_snake_case)]
    pub fn None(&self) -> ObjectRef {
        self.0
            .singletons
            .None
            .clone()
    }

    // Statically allocated integers to make
    // often created values like 0 and 1 a shortcut.
    #[allow(non_snake_case)]
    pub fn IntOld(&self, idx: isize) -> Option<ObjectRef> {
        match (idx + (STATIC_INT_IDX_OFFSET as isize)) as usize {
            checked_idx @ 0...STATIC_INT_RANGE_MAX => {
                let static_ref = self.0.singletons.integers[checked_idx as usize].clone();
                Some(static_ref.clone())
            }
            _ => None,
        }
    }


    #[allow(non_snake_case)]
    pub fn ZeroOld(&self) -> ObjectRef {
        return self.IntOld(0).unwrap();
    }

    #[allow(non_snake_case)]
    pub fn OneOld(&self) -> ObjectRef {
        return self.IntOld(1).unwrap();
    }

    // Integer
    pub fn int(&self, value: native::Integer) -> ObjectRef {
        self.0.types.int.new(&self, value)
    }

    pub fn int_zero(&self) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::zero())
    }

    pub fn int_one(&self) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from_usize(1).unwrap())
    }

    // Boolean
    pub fn bool(&self, value: native::Boolean) -> ObjectRef {
        self.0.types.bool.new(&self, value)
    }

    pub fn bool_true(&self) -> ObjectRef {
        self.bool(true)
    }

    pub fn bool_false(&self) -> ObjectRef {
        self.bool(false)
    }

    // String
    pub fn str(&self, value: native::String) -> ObjectRef {
        self.0.types.string.new(&self, value)
    }

    pub fn str_empty(&self) -> ObjectRef {
        return self.0.types.string.empty.clone()
    }

}


impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime(heap={:?})", self.0.heap)
    }
}


#[cfg(test)]
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
