use arena::Heap;
use object::Object;
use result::Result;
use std::any::Any;
use snektype::BuiltinType;
use std::rc::Rc;


#[derive(Clone)]
pub struct Runtime {
    heap: Heap
}


impl Runtime {
    #[inline]
    pub fn new(heap_size: Option<usize>) -> Runtime {
        let size = match heap_size {
            Some(i) => i,
            None => 4096
        };

        return Runtime {
            heap: Heap::new(size)
        }
    }

    pub fn reserve(&mut self, store: Rc<BuiltinType>) -> Result<Rc<BuiltinType>> {
        self.heap.reserve(store)
    }

}