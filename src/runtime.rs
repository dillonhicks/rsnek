#[macro_use]
use log;

use heap::Heap;
use object::ObjectRef;
use result::RuntimeResult;
use std::any::Any;
use builtin::Builtin;
use std::rc::{Rc,Weak};
use std::cell::RefCell;

// Patterns about References Taken from:
//   https://ricardomartins.cc/2016/06/08/interior-mutability
pub type RuntimeRef = Rc<RefCell<_Runtime>>;


pub struct _Runtime {
    heap: Heap
}


pub struct Runtime(RuntimeRef);


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
            None => 256 * 1024
        };

        let runtime =_Runtime {
            heap: Heap::new(size)
        };

        Runtime(Rc::new(RefCell::new(runtime)))
    }

    pub fn push_object(&mut self, reference: ObjectRef) -> RuntimeResult {
        (self.0.borrow_mut()).heap.push_object(reference)
    }

    pub fn debug_references(&self) {
        (self.0.borrow_mut()).heap.print_ref_counts()
    }

    pub fn gc_object_refs(&mut self) {
        (self.0.borrow_mut()).heap.gc_pass()
    }


}
