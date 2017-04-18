
extern crate rsnek_runtime;
extern crate rsnek_compile;

use std::borrow::Borrow;
use std::io::{self, Read, Write, BufRead};

use rsnek_runtime::typedef::objectref::ObjectRef;
use rsnek_runtime::typedef::builtin::Builtin;
use rsnek_runtime::object::method::{StringCast, Add};
use rsnek_runtime::error::Error;

use rsnek_runtime::result::RuntimeResult;
use rsnek_runtime::runtime::{NoneProvider, StringProvider, IntegerProvider, Runtime, Interpreter, ThreadModel};
use rsnek_compile::{Compiler, fmt};
use rsnek_compile::compiler::{Instr, Value, OpCode};

mod strings;



#[inline(always)]
fn print_banner() {
    println!("{}", strings::BANNER);
    println!("{}", strings::VERSION);
    println!("{}", strings::BUILD);
}


#[inline(always)]
fn print_prompt() {
    print!("\n{}", strings::PROMPT);
    io::stdout().flush().unwrap();
}

use std::collections::HashMap;

struct InterpreterState {
    interactive: bool,
    namespace: HashMap<String, ObjectRef>,
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
                    _ => unimplemented!()
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
                let lhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => panic!("No values in value stack for binadd!")
                };
                let rhs = match self.stack.pop() {
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



fn echo(rt: &Runtime) -> i64 {
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

const ECHOREF: &'static Fn(&Runtime) -> i64 = &echo;

fn main() {
    Interpreter::start(&echo, ThreadModel::System)
}
