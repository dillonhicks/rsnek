/// heap.rs - memory management for the "interpreter"
use std;
use std::any::Any;
use std::rc::Rc;
use arena::TypedArena;

use error::{Error, ErrorType};
use result::RuntimeResult;

use typedef::object::ObjectRef;
use typedef::builtin::Builtin;

type Arena = Vec<ObjectRef>;


/// The dynamically growing heap space for the RSnek Runtime
///
/// Objects created dynamically for purposes of the interpreter should be alloc'd onto the
/// heap in order to benefit from the reference counting and garbage collection.
///
pub struct Heap {
    max_size: usize,
    object_count: usize,
    arena: TypedArena<ObjectRef>
}


impl Heap {
    #[inline(always)]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            max_size: capacity,
            object_count: 0,
            arena: TypedArena::new(),
        }
    }

    pub fn alloc_dynamic(&mut self, reference: ObjectRef) -> RuntimeResult {
        if self.object_count == self.max_size {
            return Err(Error(ErrorType::Runtime, "Out of Heap Space!"))
        }

        self.arena.alloc(reference.clone());
        self.object_count += 1;
        Ok(reference)
    }

    pub fn size(&self) -> usize {
        return self.object_count
    }

    #[cfg(rsnek_debug)]
    pub fn print_ref_counts(&self) {
        for objref in &self.arena {
            println!("{}: refs {}", objref, Rc::strong_count(&objref.0));
        }
    }
}

impl std::fmt::Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Heap(size={}, max={})", self.object_count, self.max_size)
    }
}