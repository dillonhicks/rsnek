// Memory and shit
use result::RuntimeResult;
use object::ObjectRef;
use std::any::Any;
use builtin::Builtin;
use std::rc::Rc;
use error::{Error, ErrorType};



type Arena = Vec<ObjectRef>;


#[derive(Clone)]
pub struct Heap {
    max_size: usize,
    arena : Arena
}


impl Heap {

    #[inline]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            max_size: capacity,
            arena: Arena::new()
        }
    }

    pub fn push_object(&mut self, reference: ObjectRef) -> RuntimeResult {
        if self.max_size == self.arena.len() {
            return Err(Error(ErrorType::Runtime, "Out of Heap Space!"))
        }

        self.arena.push(reference.clone());
        //debug!("Heap Size: {}", self.get_size());
        Ok(reference)
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
        self.arena.retain(|ref objref| Rc::strong_count(&objref.0) > 1);
    }

}