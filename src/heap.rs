/// heap.rs - memory management for the "interpreter"
use std;
use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::borrow::Borrow;

use arena::TypedArena;

use error::{Error, ErrorType};
use result::RuntimeResult;

use object::model::PyBehavior;
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
    object_count: Cell<usize>,
    index: RefCell<HashMap<ObjectId, ObjectRef>>,
    arena: RefCell<Vec<ObjectRef>>,
}


impl Heap {
    #[inline(always)]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            capacity: capacity,
            object_count: Cell::new(0),
            index: RefCell::new(HashMap::new()),
            arena: RefCell::new(Vec::new()),
        }
    }

    pub fn alloc_static(&self, reference: ObjectRef) -> RuntimeResult {

        self.arena.borrow_mut().push(reference.clone());

        let intern = reference.clone();
        let builtin: &Box<Builtin> = intern.0.borrow();
        let id = builtin.deref().native_identity();

        self.index.borrow_mut().insert(id, reference.clone());
        Ok(reference.clone())
    }

    pub fn alloc_dynamic(&self, reference: ObjectRef) -> RuntimeResult {
        if self.object_count.get() == self.capacity {
            return Err(Error::runtime("Out of Heap Space!"));
        }

        let stored_ref = self.alloc_static(reference.clone());

        if stored_ref.is_ok() {
            self.object_count.set(self.object_count.get() + 1);
        }

        stored_ref
    }

    pub fn find_object(&self, id: ObjectId) -> RuntimeResult {
        match self.index.borrow().get(&id) {
            Some(value) => Ok(value.clone()),
            None => {
                println!("Id: {:?}", id as *const u64);
                Err(Error::runtime("Could not find object with id"))
            }
        }
    }

    pub fn size(&self) -> usize {
        return self.object_count.get();
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    #[cfg(rsnek_debug)]
    pub fn print_ref_counts(&self) {
        for objref in &self.arena.borrow() {
            println!("{}: refs {}", objref, Rc::strong_count(&objref.0));
        }
    }
}

impl std::fmt::Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Heap(size={}, max={})", self.object_count.get(), self.capacity)
    }
}

