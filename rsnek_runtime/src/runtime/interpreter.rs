use std::ops::Deref;
use std::fs::File;
use std::collections::vec_deque::VecDeque;
use std::borrow::Borrow;
use std::marker::Sync;
use std::io::{self, Read, Write};
use std::collections::HashMap;

use fringe::{OsStack, Generator};
use fringe::generator::Yielder;
use itertools::Itertools;
use rustyline;
use rustyline::Config as RLConfig;
use rustyline::CompletionType;
use rustyline::error::ReadlineError;
#[allow(unused_imports)]
use serde::Serialize;

use rsnek_compile::{Compiler, fmt};
use rsnek_compile::compiler::{Instr, Value, OpCode};

use resource;
use error::Error;
use result::RuntimeResult;
use runtime::Runtime;
use traits::{
    NoneProvider,
    StringProvider,
    CodeProvider,
    IntegerProvider,
    FloatProvider,
    TupleProvider,
    DictProvider,
    BooleanProvider,
};
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
    StringCast,
    Call
};
use typedef::native;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;


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
    SyntaxError = 2,
    NotImplemented = 3,
}


pub struct Interpreter {}


impl Interpreter {

    // TODO: {T100} Arguments plumbing via a sys module?
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
    // TODO: {T100} Change namespace to be PyDict or PyModule or PyObject or something
    namespace: HashMap<native::String, ObjectRef>,
    stack: Vec<ObjectRef>,
}

impl InterpreterState {
    pub fn new() -> Self {
        InterpreterState {
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

    fn force_pop(&mut self) -> Option<ObjectRef> {
        self.stack.pop()
    }

    /// Execute exactly one instruction
    fn exec_one(&mut self, rt: &Runtime, instr: &Instr) -> Option<RuntimeResult> {
       // println!("exec: {:?}", instr);

        match instr.tuple() {
            (OpCode::LoadConst, Some(value)) => {
                let objref = match value {
                    Value::Str(string) => rt.str(string),
                    Value::Int(i) => rt.int(i),
                    Value::Float(f) => rt.float(f),
                    Value::Bool(b) => rt.bool(b),
                    Value::Complex(_) => {
                        let msg = format!("Complex not impelmented! {}#{}", file!(), line!());
                        return Some(Err(Error::runtime(&msg)));
                    },
                    // TODO: {T100} This should be created in compiler.rs
                    Value::Code(args, c) =>
                        rt.code(native::Code {
                        co_name: native::String::default(),
                        co_names: args.iter().map(String::clone).collect(),
                        co_varnames: native::Tuple::new(),
                        co_code: c.to_vec(),
                        co_consts: native::Tuple::new(),
                    }),
                };

                self.stack.push(objref);
                None
            },
            (OpCode::StoreName, Some(value)) => {
                let name = match value {
                    Value::Str(string) => string,
                    _ => return Some(Err(
                        Error::runtime("Attempt to store a non string named value!")))
                };

                match self.stack.pop() {
                    Some(objref) => self.namespace.insert(name, objref),
                    None => return Some(Err(
                        Error::runtime("No values in value stack to store!")))
                };

                None
            },
            (OpCode::LoadName, Some(value)) => {
                let name = match value {
                    Value::Str(string) => string,
                    _ => return Some(Err(
                        Error::runtime("Attempt to load a non string named value!")))
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
                    None => return Some(Err(Error::runtime(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

                let lhs = match self.stack.pop() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::runtime(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

                // TODO: {T100} Give `Instr` getters
                let result = match self.exec_binop(rt, instr.tuple().0, &lhs, &rhs) {
                    Ok(objref) => objref,
                    err => return Some(err)
                };

                self.stack.push(result.clone());
                None
            },
            (OpCode::CallFunction, None) => {
                let func = match self.stack.pop() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::runtime("No values in value stack for call!")))
                };

                // TODO: {T100} this is obviously wrong, need a convention to get min number
                // of args and restore stack context for function calls and shiz.
                let mut args: Vec<ObjectRef> = Vec::new();
                args.append(&mut self.stack); // forgive me,


                let boxed: &Box<Builtin> = func.0.borrow();
                let result = match boxed.deref(){
                    &Builtin::Function(_) => {
                        boxed.op_call(&rt, &rt.tuple(args), &rt.tuple(vec![]), &rt.dict(native::None()))
                    },
                    &Builtin::Code(ref code) => {
                        if args.len() < code.value.0.co_names.len() {
                            return Some(Err(Error::typerr(
                                &format!("Expected {} args got {}", code.value.0.co_names.len(),
                                         args.len()))));
                        }

                        // Because overwriting the global namespace with function args is always a good decision....
                        for argname in code.value.0.co_names.iter() {
                            // unwrap here should be safe because of the previous check
                            self.namespace.insert(argname.clone(), args.pop().unwrap());
                        }
                        // Put back the args we didnt consume
                        self.stack.append(&mut args);

                        let mut objects: VecDeque<ObjectRef> = VecDeque::new();

                        let result = code.value.0.co_code.iter()
                            .map(|ref instr| self.exec_one(&rt, instr))
                            .filter(Option::is_some)
                            .map(Option::unwrap)
                            .fold_results((&mut objects).clone(), |mut acc, i| {acc.push_back(i); acc});

                        let back = objects.back();

                        match (result, back) {
                            (Ok(_), Some(objref)) => Ok(objref.clone()),
                            (Ok(_), None)         => Ok(rt.none()),
                            (Err(err), _)         =>  Err(err)
                        }
                    },
                    _ => Err(Error::typerr(&format!("line {}",line!())))
                };

                match result {
                    Ok(objref) => self.stack.push(objref),
                    other => return Some(other),
                };

                None
            }
            (OpCode::ReturnValue, None) => {
                // TODO: {T100} jump out of current frame
                None
            },
            (OpCode::MakeFunction, None) => {
                match self.stack.pop() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::runtime(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

//                let code = match self.stack.pop() {
//                    Some(objref) => objref,
//                    None => panic!("No values in value stack for {:?}!", instr.tuple().0)
//                };
//
//
//                self.namespace.insert(name.to_string(), code);
                None
            }
            (OpCode::PopTop, None) => {

                    match self.stack.pop() {
                        Some(objref) => Some(Ok(objref)),
                        None => None
                    }

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

    pub fn log_state (&self) {
        // FIXME: Remove when parser/compiler allows for an expression of a single name
        // or maybe just hardcode that in?
        let stack: String = self.stack.iter().enumerate().map(|(idx, objref)| {
            let b: &Box<Builtin> = objref.0.borrow();
            match b.native_str() {
                Ok(s) => format!("{}: {} = {}", idx, b.debug_name(), s),
                Err(_) => format!("{}: {} = ??", idx, b.debug_name()),
            }
        }).collect::<Vec<String>>().join("\n");

        let values: Vec<String> = self.namespace.iter().map(|(key, v): (&String, &ObjectRef)| {
            let b: &Box<Builtin> = v.0.borrow();

            match b.native_str() {
                Ok(s) => format!("{}: {} = {}", key, b.debug_name(), s),
                Err(_) => format!("{}: {} = ??", key, b.debug_name()),
            }
        }).collect();


        let state = format!("\nstack:\n{}\n\nlocals:\n----------\n{}\n", stack, values.join("\n"));
        debug!("Interpreter"; "State" => state);
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
    info!("\n{}", resource::strings::BANNER2);
    info!("{}", resource::strings::VERSION);
    info!("{}", resource::strings::BUILD);
}


/// Create the closure with the `MainFn` signature that captures a copy
/// of the arguments sent to `create_python_main`. The closure will try to use
/// the first argument as the file to load.
fn create_python_main(mode: Mode, args: Argv) -> Box<MainFn> {

    let myargs: Box<Vec<String>> = Box::new(args.iter().map(|s| s.to_string()).collect());

    Box::new(move |rt: &Runtime| -> i64 {
        let text: String = match (mode.clone(), myargs.get(0)) {
            (Mode::Command(cmd), _) => cmd.clone(),
            (Mode::Module(_), _) => {
                error!("Not Implemented"; "mode" => "-m <module>");
                return ExitCode::NotImplemented as i64
            },
            (Mode::File, Some(path)) => {
                match File::open(&path) {
                    // TODO: {T100} Check size so we aren't going ham and trying to read a file the size
                    // of memory or something?
                    Ok(ref mut file) => {
                        let mut buf: Vec<u8> = Vec::new();
                        match file.read_to_end(&mut buf) {
                            Err(err) => {
                                debug!("{:?}", err);

                                return match err.raw_os_error() {
                                    Some(code) => code as i64,
                                    _ => ExitCode::GenericError as i64
                                };
                            },
                            _ => {}
                        };
                        // TODO: {T100} Is using lossy here a good idea?
                        String::from_utf8_lossy(&buf).to_string()
                    },
                    Err(err) => {
                        debug!("{:?}", err);
                        return match err.raw_os_error() {
                            Some(code) => code as i64,
                            _ => ExitCode::GenericError as i64
                        }
                    }
                }
            },
            _ => {
                debug!("Unable to determine input type");
                return ExitCode::GenericError as i64
            }
        };

        let mut compiler = Compiler::new();
        let mut interpreter = InterpreterState::new();
        // TODO: {T100} use scope resolution in the future
        // Manually load the builtin print function into the interpreter namespace
        // since rsnek does not have a concept of modules at this time.
        interpreter.namespace.insert(String::from("print"), rt.get_builtin("print"));
        interpreter.namespace.insert(String::from("len"), rt.get_builtin("len"));
        interpreter.namespace.insert(String::from("type"), rt.get_builtin("type"));
        interpreter.namespace.insert(String::from("str"), rt.get_builtin("str"));
        interpreter.namespace.insert(String::from("int"), rt.get_builtin("int"));

        let ins = match compiler.compile_str(&text) {
            Ok(ins) => ins,
            Err(_) => {
                error!("SyntaxError: Unable to compile input");
                return ExitCode::SyntaxError as i64
            },
        };

        if let Some(path) = myargs.get(0) {
            let outpath = [path, "compiled"].join(".");

            match File::create(&outpath) {
                Ok(ref mut file) => {
                   match file.write(fmt::json(&ins).as_bytes()) {
                       Err(err) => {
                           debug!("{:?}", err);
                       }
                       _ => {}
                   };
                },
                Err(err) => {
                    debug!("{:?}", err);
                }
            }
        }

        let result = interpreter.exec(&rt, &(*ins));

        let code = match result {
            Ok(_)    => {
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


/// Entry point for the interactive repl mode of the interpreter
fn python_main_interactive(rt: &Runtime) -> i64 {
    let mut compiler = Compiler::new();

    let config = RLConfig::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl = rustyline::Editor::<()>::with_config(config);
    let mut interpreter = InterpreterState::new();

    // TODO: {T100} use scope resolution in the future
    // Manually load the builtin print function into the interpreter namespace
    // since rsnek does not have a concept of modules at this time.
    interpreter.namespace.insert(String::from("print"), rt.get_builtin("print"));
    interpreter.namespace.insert(String::from("len"), rt.get_builtin("len"));
    interpreter.namespace.insert(String::from("type"), rt.get_builtin("type"));
    interpreter.namespace.insert(String::from("str"), rt.get_builtin("str"));
    interpreter.namespace.insert(String::from("int"), rt.get_builtin("int"));

    let mut lineno = 0;

    print_banner();

    'repl: loop {
        match io::stdout().flush() {
            Err(err) => error!("Error Flushing STDOUT: {:?}", err),
            _ => ()
        }


        lineno += 1;
        info!("In[{}] {} ", lineno, resource::strings::PROMPT);

        let text = match rl.readline(&"") {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                line
            },
            Err(ReadlineError::Interrupted) => {
                warn!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                warn!("CTRL-D");
                break;
            }
            _ => continue
        };



        let ins = match compiler.compile_str(&text) {
            Ok(ins) => ins,
            Err(_) => {
                error!("SyntaxError"; "message" => "Unable to compile input");
                continue 'repl
            },
        };

        'process_ins: for i in ins.iter() {
            match interpreter.exec_one(rt, &i) {
                Some(Err(err)) => {
                    error!("{:?}Error", err.0; "message" => err.1.clone());
                    break 'process_ins
                },

                _ => continue 'process_ins
            }

        }

        // Force pop in interactive mode to clear return values?
        match interpreter.force_pop() {
            Some(objref) => {
                if objref != rt.none() {
                    let boxed: &Box<Builtin> = objref.0.borrow();
                    let string = match boxed.native_str() {
                        Ok(s) => s,
                        Err(err) => format!("{:?}Error: {}", err.0, err.1),
                    };
                    info!("\nOut[{}]: {}\n\n", lineno, string);
                }
            }
            _ => {},
        }

        interpreter.log_state();
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