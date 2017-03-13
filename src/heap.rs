// heap.rs - memory management for the "interpreter"
use std::any::Any;
use std::rc::Rc;
use arena::TypedArena;

use error::{Error, ErrorType};
use result::RuntimeResult;

use typedef::object::ObjectRef;
use typedef::builtin::Builtin;

type Arena = Vec<ObjectRef>;


pub struct Heap {
    max_size: usize,
    arena : Arena,
    typed_arena: TypedArena<ObjectRef>
}


impl Heap {

    #[inline]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            max_size: capacity,
            arena: Arena::new(),
            typed_arena: TypedArena::new(   ),
        }
    }

    pub fn push_object(&mut self, reference: ObjectRef) -> RuntimeResult {
        if self.max_size == self.arena.len() {
            return Err(Error(ErrorType::Runtime, "Out of Heap Space!"))
        }

        self.typed_arena.alloc(reference.clone());
        Ok(reference)

//        self.arena.push(reference.clone());
//        //debug!("Heap Size: {}", self.get_size());
//        Ok(reference)
    }

    pub fn get_size(&self) -> usize {
        return self.arena.len();
    }

    pub fn print_ref_counts(&self) {
        for objref in &self.arena {
            println!("{}: refs {}", objref, Rc::strong_count(&objref.0));
        }
    }

    pub fn gc_pass(&mut self) {
        //self.arena.retain(|ref objref| Rc::strong_count(&objref.0) > 1);
    }

}