use std::borrow::Borrow;
use std::marker::Sync;

use std::io::{self, Read, Write, BufRead};

use std::collections::HashMap;

use fringe::{OsStack, Generator};
use fringe::generator::Yielder;
use num::ToPrimitive;
use serde::Serialize;

use rsnek_compile::{Compiler, fmt};
use rsnek_compile::compiler::{Instr, Value, OpCode};

use resource;
use error::Error;
use result::RuntimeResult;
use runtime::Runtime;
use traits::{NoneProvider, StringProvider, IntegerProvider, FloatProvider};
use object::method::{Add, IntegerCast, StringCast};
use typedef::native;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Config {
    pub interactive: bool,
    pub debug_support: bool,
    pub thread_model: ThreadModel,
    pub logging: Logging
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Logging;


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum ThreadModel {
    OsThreads,
    GreenThreads,
}


pub struct Interpreter {
    state: InterpreterState
}



impl Interpreter {

    pub fn run(config: &Config) -> i64 {
        let f = &python_main_interactive;

        let main_func = match config.interactive {
            true => Box::new(f),
            false => unimplemented!()
        };

        let main_thread: Box<Thread> = match config.thread_model {
            ThreadModel::OsThreads => Box::new(Pthread {func: (*main_func).clone()}),
            ThreadModel::GreenThreads => Box::new(GreenThread {func: FnWrapper((*main_func).clone())})
        };

        let rt = &Runtime::new();
        main_thread.start(&rt)
    }

}


struct InterpreterState {
    interactive: bool,
    // TODO: Change namespace to be PyDict or PyModule or PyObject or something
    namespace: HashMap<native::String, ObjectRef>,
    stack: Vec<ObjectRef>,
}

impl InterpreterState {
    pub fn new() -> Self {
        InterpreterState {
            interactive: true,
            namespace: HashMap::new(),
            stack: Vec::new(),
        }
    }

    fn exec(&mut self, rt: &Runtime, instr: &Instr) -> Option<RuntimeResult> {
        match instr.tuple() {
            (OpCode::LoadConst, Some(value)) => {
                let objref = match value {
                    Value::Str(string) => rt.str(string),
                    Value::Int(i) => rt.int(i),
                    Value::Float(f) => rt.float(f),
                };

                self.stack.push(objref);
                None
            },
            (OpCode::StoreName, Some(value)) => {
                let name = match value {
                    Value::Str(string) => string,
                    _ => panic!("Attempt to store a non string named value!")
                };

                match self.stack.pop() {
                    Some(objref) => self.namespace.insert(name, objref),
                    None => panic!("No values in value stack to store!")
                };


                None
            },
            (OpCode::LoadName, Some(value)) => {
                let name = match value {
                    Value::Str(string) => string,
                    _ => panic!("Attempt to load a non string named value!")
                };

                match self.namespace.get(&name) {
                    Some(objref) => {
                        self.stack.push(objref.clone());
                    },
                    None => return Some(Err(Error::name(&name)))
                };

                None
            },
            (OpCode::BinaryAdd, None) => {
                let rhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => panic!("No values in value stack for binadd!")
                };

                let lhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => panic!("No values in value stack for binadd!")
                };

                let boxed: &Box<Builtin> = lhs.0.borrow();
                let result = match boxed.op_add(&rt, &rhs) {
                    Ok(objref) => objref,
                    err => return Some(err)
                };

                self.stack.push(result.clone());
                None
            },
            (OpCode::ReturnValue, None) => {
                let retval = match self.stack.pop() {
                    Some(objref) => objref,
                    None => return None
                };

                Some(Ok(retval))
            }
            _ => unimplemented!()
        }
    }
}

trait Thread<'a> {
    fn start(&self, rt: &Runtime) -> i64;
    fn run<'b>(&self, rt: &'b Runtime) -> i64;
}

struct Pthread<'a>  {
    func: &'a Fn(&Runtime) -> (i64)
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


struct GreenThread<'a> {
    func: FnWrapper<'a>
}

impl<'a> Thread<'a> for GreenThread<'a> {
    fn start(&self, rt: &Runtime) -> i64 {
       self.run(&rt)
    }

    fn run<'b>(&self, rt: &'b Runtime) -> i64 {
        let stack = OsStack::new(1 << 16).unwrap();

        let mut gen = Generator::new(stack, move |yielder, ()| {
            let main_thread = Greenlet {
                yielder: yielder,
                func: self.func.clone()
            };
            let rt = Runtime::new();

            main_thread.start(&rt);
        });

        let mut prev: Option<i64> = None;
        let mut loops = 0;
        loop {
            let out = gen.resume(());
            match out {
                None => { break },
                _ => (),
            };
            loops += 1;
            prev = out;
        }

        prev.unwrap_or(0)
    }
}

#[derive(Clone)]
struct FnWrapper<'a>(&'a Fn(&Runtime) -> (i64));

unsafe impl<'a> Sync for FnWrapper<'a> {}
unsafe impl<'a> Send for FnWrapper<'a> {}


struct Greenlet<'a> {
    yielder: &'a mut Yielder<(), i64>,
    func: FnWrapper<'a>,
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


#[inline(always)]
fn print_banner() {
    println!("{}", resource::strings::BANNER2);
    println!("{}", resource::strings::VERSION);
    println!("{}", resource::strings::BUILD);
}


#[inline(always)]
fn print_prompt() {
    print!("\n{} ", resource::strings::PROMPT);
    io::stdout().flush().unwrap();
}


fn python_main_interactive(rt: &Runtime) -> i64 {
    let compiler = Compiler::new();
    let stdin = io::stdin();

    let mut interpreter = InterpreterState::new();

    print_banner();
    print_prompt();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(t) => t,
            _ => continue
        };


        let ins = compiler.compile_str(&text);
        //println!("{}", fmt::json(&ins));

        'process_ins: for i in ins.iter() {
            match interpreter.exec(rt, &i) {
                Some(Ok(objref)) => {
                    let boxed: &Box<Builtin> = objref.0.borrow();
                    let string = match boxed.native_str() {
                        Ok(s) => s,
                        Err(err) => format!("{:?}Error: {}", err.0, err.1),
                    };

                },
                Some(Err(err)) => {
                    println!("{:?}Error: {}", err.0, err.1);
                    break 'process_ins
                },

                _ => continue 'process_ins
            }
        }

        let values: Vec<String> = interpreter.namespace.iter().map(|(key, v): (&String, &ObjectRef)| {
            let b: &Box<Builtin> = v.0.borrow();

            match b.native_str() {
                Ok(s) => format!("{}: {}", key, s),
                Err(err) => format!("{}: ??", key),
            }
        }).collect();

        println!("\nlocals:\n----------\n{}", values.join("\n"));

        print_prompt();

    }


    0
}

fn green_test_1(rt: &Runtime) -> i64 {

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