use std::borrow::Borrow;

use fringe::{OsStack, Generator};
use fringe::generator::Yielder;
use num::ToPrimitive;

use traits::IntegerProvider;
use object::method::{Add, IntegerCast};
use runtime::Runtime;

use typedef::builtin::Builtin;

pub enum ThreadModel {
    System,
    Generator,
}

pub struct Interpreter;


impl Interpreter {

    pub fn start(model: ThreadModel) {
        match model {
            ThreadModel::System => Interpreter::_with_system_threads(),
            ThreadModel::Generator => Interpreter::_with_uthreads(),
        };
    }

    fn _with_system_threads() {
        let rt = Runtime::new();
        let main_thread = Thread{};

        let out = main_thread.start(&rt);
        println!("{:?}", out);
    }

    fn _with_uthreads() {
        println!("Hello RSNEK!");

        let stack = OsStack::new(1 << 16).unwrap();
        let mut gen = Generator::new(stack, move |yielder, ()| {
            let main_thread = Greenlet {
                yielder: yielder
            };
            let rt = Runtime::new();

            main_thread.start(&rt)
        });

        let mut prev: Option<i64> = None;
        let mut loops=  0;
        loop {
            let out = gen.resume(());
            match out {
                None => {break},
                _ => (),
            };
            loops += 1;
            prev = out;
        }
        println!("{:?} {}", prev, loops);
    }
}

struct Thread;

impl Thread {
    fn start(&self, rt: &Runtime) -> i64 {
        self.run(&rt)
    }

    fn run(&self, rt: &Runtime) -> i64 {
        let mut value = rt.int(0);

        for datum in 1..10000000 {
            let newvalue;
            {
                let boxed: &Box<Builtin> = value.0.borrow();
                newvalue = boxed.op_add(&rt, &rt.int(datum)).unwrap();
            }
            value = newvalue;
        }

       let boxed: &Box<Builtin> = value.0.borrow();
       let sum = boxed.native_int().unwrap();
        sum.to_i64().unwrap()
    }
}


struct Greenlet<'a> {
    yielder: &'a mut Yielder<(), i64>
}


impl<'a> Greenlet<'a> {
    fn start(&self, rt: &Runtime) {
        self.run(&rt)
    }

    fn run(&self, rt: &Runtime) {
        let mut value = rt.int(0);

        for datum in 1..10000000 {
            let newvalue;
            {
                let boxed: &Box<Builtin> = value.0.borrow();
                newvalue = boxed.op_add(&rt, &rt.int(datum)).unwrap();
            }
            value = newvalue;

            //self.yielder.suspend(datum);
        }

        let boxed: &Box<Builtin> = value.0.borrow();
        let sum = boxed.native_int().unwrap();

        self.yielder.suspend(sum.to_i64().unwrap());
    }
}

