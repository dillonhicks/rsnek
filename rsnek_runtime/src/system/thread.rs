//! Where the magic happens...
use std::borrow::Borrow;
use std::cell::{Ref, Cell, RefCell};
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::convert::From;
use std::fs::File;
use std::io::{self, Read, Write};
use std::marker::Sync;
use std::ops::{Deref};

use fringe::generator::Yielder;
use fringe::{OsStack, Generator};

use ::runtime::Runtime;
use ::system::{SharedMainFnRef, MainFnRef, MainFn};

/// Optimistic definitions pf supported threading models
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum ThreadModel {
    OsThreads,
    GreenThreads,
    // WebAsm?
}

/// Thread lifecycle traits
pub trait Thread<'a> {
    /// Do the necessary things in order to initialize the thread
    fn start(&self, rt: &Runtime) -> i64;

    /// Start the execution of the thread
    fn run<'b>(&self, rt: &'b Runtime) -> i64;
}


/// Regular system thread
pub struct Pthread<'a>  {
    pub func: MainFnRef<'a>
}


impl<'a> Thread<'a> for Pthread<'a> {
    fn start(&self, rt: &Runtime) -> i64 {
        self.run(&rt)
    }

    fn run<'b>(&self, rt: &'b Runtime) -> i64 {
        let func = self.func.clone();
        func(rt)
    }

}

/// Wrapper around a sharable function pointer that can be used as the
/// entry point of a generator driven userland managed thread stack.
pub struct GreenThread<'a> {
    pub func: SharedMainFnRef<'a>
}


impl<'a> Thread<'a> for GreenThread<'a> {
    fn start(&self, rt: &Runtime) -> i64 {
        self.run(&rt)
    }

    /// Note: Since `Runtime` doesn't implement Sync or Send
    /// it cannot be passed into the context of the greenlet.
    /// It is on the roadmap to fix that so there is not the
    /// need to create 2 runtimes.
    #[allow(unused_variables)]
    fn run<'b>(&self, rt: &'b Runtime) -> i64 {
        /// Start the stack off with 4kb
        let stack = OsStack::new(1 << 12).unwrap();

        let mut gen = Generator::new(stack, move |yielder, ()| {
            let main_thread = Greenlet {
                yielder: yielder,
                func: self.func.clone()
            };
            let rt = Runtime::new();

            main_thread.start(&rt);
        });

        let mut prev: Option<i64> = None;

        /// The hallowed event loop
        loop {
            let out = gen.resume(());
            match out {
                None => { break },
                _ => (),
            };
            prev = out;
        }

        prev.unwrap_or(0)
    }
}


/// The greenlet is created by a GreenThread and represents a resumbale thread context
/// with its own stack. `func` is the entry point for the greenlet and `yielde` is the
/// libfringe special sauce that is causes the user mode context switch on
/// call to `Yielder::suspend`.
struct Greenlet<'a> {
    yielder: &'a mut Yielder<(), i64>,
    func: SharedMainFnRef<'a>,
}


impl<'a> Thread<'a> for Greenlet<'a> {
    fn start(&self, rt: &Runtime) -> i64 {
        self.run(&rt)
    }

    fn run(&self, rt: &Runtime) -> i64 {
        let func = self.func.0.clone();
        self.yielder.suspend(func(rt));
        0
    }
}

