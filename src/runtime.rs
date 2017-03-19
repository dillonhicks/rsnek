/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::rc::Rc;

pub use result::RuntimeResult;
use heap::Heap;

use typedef::native::ObjectId;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use typedef::boolean::BooleanObject;
use typedef::objectref::ToRtWrapperType;
use typedef::none::NONE_TYPE;

/// If not size is given, fallback to 256kb.
pub const DEFAULT_HEAP_CAPACITY: usize = 256 * 1024;


/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


/// Concrete struct that holds the current runtime state, heap, etc.
struct RuntimeInternal {
    heap: Heap,
    singletons: SingletonIndex
}


//noinspection RsFieldNaming
struct SingletonIndex {
    True: ObjectRef,
    False: ObjectRef,
    None: ObjectRef
}


pub enum Singleton {
    True,
    False,
    None
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
    pub fn new(heap_size: Option<usize>) -> Runtime {
        let size = match heap_size {
            Some(i) => i,
            None => DEFAULT_HEAP_CAPACITY
        };

        let mut heap = Heap::new(size);

        let True: ObjectRef = BooleanObject::new_true().to();
        let False: ObjectRef = BooleanObject::new_false().to();

        let singletons = SingletonIndex {
            True: heap.alloc_static(True).unwrap(),
            False: heap.alloc_static(False).unwrap(),
            None: heap.alloc_static(NONE_TYPE.to()).unwrap()
        };

        let runtime = RuntimeInternal {
            heap: heap,
            singletons: singletons
        };

        Runtime(Rc::new(Box::new(runtime)))
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
        return self.0.heap.find_object(id)
    }

    #[allow(non_snake_case)]
    pub fn True(&self) -> ObjectRef {
        self.0.singletons.True.clone()
    }

    #[allow(non_snake_case)]
    pub fn False(&self) -> ObjectRef {
        self.0.singletons.False.clone()
    }

    #[allow(non_snake_case)]
    pub fn None(&self) -> ObjectRef {
        self.0.singletons.None.clone()
    }

    #[cfg(rsnek_debug)]
    pub fn debug_references(&self) {
        (self.0.borrow_mut()).heap.print_ref_counts()
    }
}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime(heap={:?})", self.0.heap)
    }
}