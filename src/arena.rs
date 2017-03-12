// Memory and shit
use result::{Result, InterpreterError};
use object::Object;
use std::any::Any;
use snektype::BuiltinType;
use std::rc::Rc;

#[derive(Clone)]
pub struct Heap {
    max_size: usize,
    arena : Vec<Rc<BuiltinType>>
}

impl Heap {

    #[inline]
    pub fn new(capacity: usize) -> Heap {
        Heap {
            max_size: capacity,
            arena: Vec::new()
        }
    }

    pub fn reserve(&mut self, store: Rc<BuiltinType>) -> Result<Rc<BuiltinType>> {
        if self.max_size == self.arena.len() {
            return Err(InterpreterError {message: "Out of Heap Space!"})
        }

        self.arena.push(store.clone());
        Ok(store)
    }
}