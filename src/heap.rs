/// heap.rs - memory management for the "interpreter"
use std;
use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

use arena::TypedArena;

use error::{Error, ErrorType};
use result::RuntimeResult;

use object::api::Identifiable;
use typedef::objectref::ObjectRef;
use typedef::native::ObjectId;
use typedef::builtin::Builtin;


/// The dynamically growing heap space for the RSnek Runtime
///
/// Objects created dynamically for purposes of the interpreter should be alloc'd onto the
/// heap in order to benefit from the reference counting and garbage collection.
///
pub struct Heap {
    capacity: usize,
    object_count: usize,
    static_count: usize,
    arena: Vec<ObjectRef>,
}


impl Heap {
    #[inline(always)]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            capacity: capacity,
            object_count: 0,
            static_count: 0,
            arena: Vec::new(),
        }
    }

    pub fn alloc_static(&mut self, reference: ObjectRef) -> RuntimeResult {

        self.arena.push(reference.clone());

        let intern = reference.clone();
        let builtin: &Box<Builtin> = intern.0.borrow();
        let id = builtin.native_identity();

        let intern2 = reference.clone();
        let builtin2: &Box<Builtin> = intern2.0.borrow();
        let id2 = builtin2.native_identity();

        Ok(reference.clone())
    }

    pub fn alloc_dynamic(&mut self, reference: ObjectRef) -> RuntimeResult {
        if self.object_count == self.capacity {
            return Err(Error::runtime("Out of Heap Space!"));
        }

        let stored_ref = self.alloc_static(reference.clone());

        if stored_ref.is_ok() {
            self.object_count += 1;

        }
        stored_ref
    }

    pub fn size(&self) -> usize {
        return self.object_count;
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
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
        write!(f, "Heap(size={}, max={})", self.object_count, self.capacity)
    }
}

