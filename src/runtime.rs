/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::any::Any;
use std::rc::{Rc,Weak};
use std::cell::RefCell;

use result::RuntimeResult;
use heap::Heap;

use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


/// If not size is given, fallback to 256kb.
pub const DEFAULT_HEAP_CAPACITY: usize = 256 * 1024;


/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


/// Concrete struct that holds the current runtime state, heap, etc.
struct RuntimeInternal {
    heap: Heap
}


/// Type that is the Reference Counted wrapper around the actual runtime
///
/// Patterns about References Taken from:
///  https://ricardomartins.cc/2016/06/08/interior-mutability
type RuntimeRef = Rc<RefCell<RuntimeInternal>>;


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

        let runtime = RuntimeInternal {
            heap: Heap::new(size)
        };

        Runtime(Rc::new(RefCell::new(runtime)))
    }

    /// Alloc a spot for the object ref in the `Heap` for the `Runtime` this will
    /// mean that there will be at one single strong reference to the `ObjectRef`
    /// for the life of the Runtime.
    ///
    /// This gives the `Runtime` to control when the `Drop<Object>` happens
    /// and finally cleans up struct behind the `ObjectRef`.
    pub fn alloc(&mut self, reference: ObjectRef) -> RuntimeResult {
        (self.0.borrow_mut()).heap.alloc_dynamic(reference)
    }

    pub fn heap_size(&self) -> usize{
        return (self.0.borrow()).heap.size()
    }

    pub fn heap_capacity(&self) -> usize{
        return (self.0.borrow()).heap.capacity()
    }

    #[cfg(rsnek_debug)]
    pub fn debug_references(&self) {
        (self.0.borrow_mut()).heap.print_ref_counts()
    }

}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime(heap={:?})", (self.0.borrow()).heap)
    }
}