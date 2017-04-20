use std::io::prelude::*;
use std::fs::File;
use std::collections::vec_deque::VecDeque;
use std::borrow::Borrow;
use std::marker::Sync;

use std::io::{self, Read, Write, BufRead};

use std::collections::HashMap;

use fringe::{OsStack, Generator};
use fringe::generator::Yielder;
use itertools::Itertools;
use num::ToPrimitive;
use serde::Serialize;

use rsnek_compile::{Compiler, fmt};
use rsnek_compile::compiler::{Instr, Value, OpCode};

use resource;
use error::Error;
use result::RuntimeResult;
use runtime::Runtime;
use traits::{NoneProvider, StringProvider, IntegerProvider, FloatProvider};
use object::method::{
    Add,
    Subtract,
    Multiply,
    Pow,
    FloorDivision,
    TrueDivision,
    BitwiseAnd,
    BitwiseOr,
    MatrixMultiply,
    LeftShift,
    RightShift,
    XOr,
    Modulus,
    IntegerCast,
    StringCast
};
use typedef::native;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;


// TODO: get actual logging or move this to a proper place
macro_rules! debug {
    ($fmt:expr) => (print!(concat!("DEBUG:", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG:", $fmt, "\n"), $($arg)*));
}

pub type Argv<'a> =  &'a [&'a str];
pub type MainFn = Fn(&Runtime) -> i64;
pub type MainFnRef<'a> = &'a Fn(&Runtime) -> (i64);


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum Mode {
    Interactive,
    Command(String),
    Module(String),
    File
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Config<'a> {
    pub mode: Mode,
    pub arguments: Argv<'a>,
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

/// Exit codes for the main interpreter loops
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
#[repr(u8)]
enum ExitCode {
    Ok = 0,
    GenericError = 1,
    NotImplemented = 254,
    Unknown = 255
}


pub struct Interpreter {}


impl Interpreter {

    // TODO: Arguments plumbing via a sys module?
    pub fn run(config: &Config) -> i64 {
        let interactive_main: MainFnRef = &python_main_interactive;
        let main = create_python_main(config.mode.clone(), config.arguments);

        let main_func = match config.mode {
            Mode::Interactive => Box::new(interactive_main),
            _ => Box::new(&(*main)),
        };

        let main_thread: Box<Thread> = match config.thread_model {
            ThreadModel::OsThreads => Box::new(Pthread {func: (*main_func).clone()}),
            ThreadModel::GreenThreads => Box::new(GreenThread {func: SharedMainFnRef((*main_func).clone())})
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

    fn exec_binop(&mut self, rt: &Runtime, opcode: OpCode, lhs: &ObjectRef, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = lhs.0.borrow();

        match opcode {
            OpCode::LogicalAnd              => Err(Error::not_implemented()),
            OpCode::LogicalOr               => Err(Error::not_implemented()),
            OpCode::BinaryAdd               => boxed.op_add(&rt, &rhs),
            OpCode::BinarySubtract          => boxed.op_sub(&rt, &rhs),
            OpCode::BinaryMultiply          => boxed.op_mul(&rt, &rhs),
            OpCode::BinaryPower             => boxed.op_pow(&rt, &rhs, &rt.int(1)),
            OpCode::BinaryTrueDivide        => boxed.op_truediv(&rt, &rhs),
            OpCode::BinaryFloorDivide       => boxed.op_floordiv(&rt, &rhs),
            OpCode::BinaryOr                => boxed.op_or(&rt, &rhs),
            OpCode::BinaryModulo            => boxed.op_mod(&rt, &rhs),
            OpCode::BinaryAnd               => boxed.op_and(&rt, &rhs),
            OpCode::BinaryMatrixMultiply    => boxed.op_matmul(&rt, &rhs),
            OpCode::BinaryXor               => boxed.op_xor(&rt, &rhs),
            OpCode::BinaryLshift            => boxed.op_lshift(&rt, &rhs),
            OpCode::BinaryRshift            => boxed.op_rshift(&rt, &rhs),
            _                               => Err(Error::runtime("THIS IS BAD VERY VERY BAD")),
        }
    }

    /// Execute exactly one instruction
    fn exec_one(&mut self, rt: &Runtime, instr: &Instr) -> Option<RuntimeResult> {
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
            (OpCode::LogicalAnd, None)              |
            (OpCode::LogicalOr, None)               |
            (OpCode::BinaryAdd, None)               |
            (OpCode::BinarySubtract, None)          |
            (OpCode::BinaryMultiply, None)          |
            (OpCode::BinaryPower, None)             |
            (OpCode::BinaryTrueDivide, None)        |
            (OpCode::BinaryFloorDivide, None)       |
            (OpCode::BinaryAnd, None)               |
            (OpCode::BinaryOr, None)                |
            (OpCode::BinaryXor, None)               |
            (OpCode::BinaryMatrixMultiply, None)    |
            (OpCode::BinaryLshift, None)            |
            (OpCode::BinaryRshift, None) => {

                let rhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => panic!("No values in value stack for binop!")
                };

                let lhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => panic!("No values in value stack for binop!")
                };

                // TODO: Give `Instr` getters
                let result = match self.exec_binop(rt, instr.tuple().0, &lhs, &rhs) {
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
            _ => Some(Err(Error::not_implemented()))
        }
    }

    ///
    fn exec(&mut self, rt: &Runtime, ins: &[Instr]) -> RuntimeResult {
        let mut objects: VecDeque<ObjectRef> = VecDeque::new();

        let result = ins.iter()
            .map(|ref instr| self.exec_one(&rt, instr))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .fold_results((&mut objects).clone(), |mut acc, i| {acc.push_back(i); acc});

        let back = objects.back();
        match (result, back) {
            (Ok(_), Some(objref)) => Ok(objref.clone()),
            (Ok(_), None)         => Ok(rt.none()),
            (Err(err), _)         => Err(err)
        }
    }

    pub fn debug_locals(&self) {
        // FIXME: Remove when parser/compiler allows for an expression of a single name
        // or maybe just hardcode that in?
        let values: Vec<String> = self.namespace.iter().map(|(key, v): (&String, &ObjectRef)| {
            let b: &Box<Builtin> = v.0.borrow();

            match b.native_str() {
                Ok(s) => format!("{}: {}", key, s),
                Err(err) => format!("{}: ??", key),
            }
        }).collect();

        debug!("\nlocals:\n----------\n{}\n", values.join("\n"));
    }
}

trait Thread<'a> {
    fn start(&self, rt: &Runtime) -> i64;
    fn run<'b>(&self, rt: &'b Runtime) -> i64;
}


struct Pthread<'a>  {
    func: MainFnRef<'a>
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
    func: SharedMainFnRef<'a>
}

impl<'a> Thread<'a> for GreenThread<'a> {
    fn start(&self, rt: &Runtime) -> i64 {
       self.run(&rt)
    }

    fn run<'b>(&self, rt: &'b Runtime) -> i64 {
        /// Start the stack off with 4kb
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

/// Wrapper around the `MainFnRef` so we have an owned
/// type on which we can specify clone semantics for
/// `Send` and `Sync` traits which allows the function reference
/// to be shared across threads.
#[derive(Clone)]
struct SharedMainFnRef<'a>(MainFnRef<'a>);

unsafe impl<'a> Sync for SharedMainFnRef<'a> {}
unsafe impl<'a> Send for SharedMainFnRef<'a> {}


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


/// Two equally valid explanations exist for this banner
///
/// 1. Print the obligatory banner to let everyone know what program is running.
/// 2. https://youtu.be/tHnA94-hTC8?t=2m47s
#[inline(always)]
fn print_banner() {
    println!("{}", resource::strings::BANNER2);
    println!("{}", resource::strings::VERSION);
    println!("{}", resource::strings::BUILD);
}


/// Write the prompt to stdout without a newline and hard flush stdout
/// for interactive mode.
#[inline(always)]
fn print_prompt() {
    print!("\n{} ", resource::strings::PROMPT);
    match io::stdout().flush() {
        Err(err) => println!("Error Flushing STDOUT: {:?}", err),
        _ => ()
    }
}


/// Create the closure with the `MainFn` signature that captures a copy
/// of the arguments sent to `create_python_main`. The closure will try to use
/// the first argument as the file to load.
fn create_python_main(mode: Mode, args: Argv) -> Box<MainFn> {

    let myargs: Box<Vec<String>> = Box::new(args.iter().map(|s| s.to_string()).collect());

    Box::new(move |rt: &Runtime| -> i64 {
        let text: String = match (mode.clone(), myargs.get(0)) {
            (Mode::Command(cmd), _) => cmd.clone(),
            (Mode::Module(module), _) => {
                debug!("Not Implemented");
                return ExitCode::NotImplemented as i64
            },
            (Mode::File, Some(path)) => {
                match File::open(&path) {
                    // TODO: Check size so we aren't going ham and trying to read a file the size
                    // of memory or something?
                    Ok(ref mut file) => {
                        let mut buf: Vec<u8> = Vec::new();
                        file.read_to_end(&mut buf);
                        // TODO: Is using lossy here a good idea?
                        String::from_utf8_lossy(&buf).to_string()
                    },
                    Err(err) => {
                        debug!("{:?}", err);
                        return match err.raw_os_error() {
                            Some(code) => code as i64,
                            _ => ExitCode::Unknown as i64
                        }
                    }
                }
            },
            _ => {
                debug!("Unable to determine input type");
                return ExitCode::Unknown as i64
            }
        };

        let mut compiler = Compiler::new();
        let mut interpreter = InterpreterState::new();
        let ins = compiler.compile_str(&text);

        let result = interpreter.exec(&rt, &(*ins));
        interpreter.debug_locals();

        let code = match result {
            Ok(objref)    => {
                debug!("result: {:?}", objref);
                ExitCode::Ok as i64
            },
            Err(err)      => {
                debug!("{:?}", err);
                ExitCode::GenericError as i64
            }
        };

        code
    })
}


use rustyline;
use rustyline::Config as RLConfig;
use rustyline::CompletionType;
use rustyline::error::ReadlineError;


/// Entry point for the interactive repl mode of the interpreter
fn python_main_interactive(rt: &Runtime) -> i64 {
    let mut compiler = Compiler::new();
    let stdin = io::stdin();

    let config = RLConfig::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl = rustyline::Editor::<()>::with_config(config);
    let mut interpreter = InterpreterState::new();

    let prompt = format!("\n{} ", resource::strings::PROMPT);

    print_banner();

    loop {
        let text = match rl.readline(&prompt){
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                line
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            _ => continue
        };

        let ins = compiler.compile_str(&text);
        println!("{}", fmt::json(&ins));

        'process_ins: for i in ins.iter() {
            match interpreter.exec_one(rt, &i) {
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

        interpreter.debug_locals();

        match io::stdout().flush() {
            Err(err) => println!("Error Flushing STDOUT: {:?}", err),
            _ => ()
        }

    }

    ExitCode::Ok as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Use to create a test case of a single line snippet of code.
    /// `assert_run!(float_add_int, "x = 1.0 + 3", ExitCode::Ok)`
    macro_rules! assert_run {
        ($name:ident, $code:expr, $status:expr) => {
            #[test]
            fn $name() {
                assert_eq!(run($code), $status as i64);
            }
        };
    }

    fn run(code: &str) -> i64 {
        let config = Config {
            mode: Mode::Command(code.to_string()),
            arguments: &[],
            thread_model: ThreadModel::OsThreads,
            logging: Logging {},
            debug_support: false,
        };

        Interpreter::run(&config)
    }

    // int + int simple binop test cases
    assert_run!(int_add, "x = 1 + 2", ExitCode::Ok);
    assert_run!(int_sub, "x = 3 - 4", ExitCode::Ok);
    assert_run!(int_mul, "x = 5 * 6", ExitCode::Ok);
    //assert_run!(int_pow, "x = 7 ** 8", ExitCode::Ok);
    //assert_run!(int_truediv, "x = 9 / 10", ExitCode::Ok);
    //assert_run!(int_floordiv, "x = 11 // 12", ExitCode::Ok);
    //assert_run!(int_and, "x = 13 & 14",  ExitCode::Ok);
    //assert_run!(int_or, "x = 14 | 15",  ExitCode::Ok);
    //assert_run!(int_xor, "x = 16 ^ 17",  ExitCode::Ok);
    assert_run!(int_matmul, "x = 18 @ 19",  ExitCode::GenericError);
    assert_run!(int_lshift, "x = 20 << 21", ExitCode::Ok);
    assert_run!(int_rshift, "x = 22 >> 23", ExitCode::Ok);
}