use std::ops::{Deref};
use std::fs::File;
use std::collections::vec_deque::VecDeque;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::marker::Sync;
use std::io::{self, Read, Write};
use std::collections::HashMap;

use fringe::{OsStack, Generator};
use fringe::generator::Yielder;
use itertools::Itertools;
use num::Zero;
use rustyline;
use rustyline::Config as RLConfig;
use rustyline::CompletionType;
use rustyline::error::ReadlineError;

use rsnek_compile::fmt;

use ::builtin::{logical_and, logical_or};
use ::compiler::Compiler;
use ::error::Error;
use ::object::method::{
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
    Call,
    BooleanCast,
    Equal,
};
use ::opcode::OpCode;
use ::resource;
use ::result::RuntimeResult;
use ::runtime::Runtime;
use ::traits::{
    NoneProvider,
    StringProvider,
    CodeProvider,
    IntegerProvider,
    FloatProvider,
    TupleProvider,
    ListProvider,
    DictProvider,
    BooleanProvider,
    FrameProvider,
    FunctionProvider
};
use ::typedef::native::{self, Native, Instr, FuncType};
use ::typedef::native::SignatureBuilder;
use ::typedef::builtin::Builtin;
use ::typedef::objectref::ObjectRef;


const RECURSION_LIMIT: usize = 256;

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
    rt: Runtime,
    // TODO: {T100} Change namespace to be PyDict or PyModule or PyObject or something
    ns: HashMap<native::String, ObjectRef>,
    // (frame, stack)
    frames: VecDeque<(ObjectRef, RefCell<native::List>)>,
}

impl InterpreterState {
    pub fn new(rt: &Runtime) -> Self {

        // Create the initial frame objects that
        // represent the __main__ entry point.
        let main_code = native::Code {
            co_name: String::from("__main__"),
            co_names: Vec::new(),
            co_varnames: Vec::new(),
            co_code: Vec::new(),
        };

        let main_frame = native::Frame {
            f_back: rt.none(),
            f_code: rt.code(main_code),
            f_builtins: rt.none(),
            f_lasti: native::Integer::zero(),
            blocks: VecDeque::new()
        };

        let mut frames = VecDeque::new();
        frames.push_back((rt.frame(main_frame), RefCell::new(native::List::new())));

        let mut istate = InterpreterState {
            rt: rt.clone(),
            ns: HashMap::new(),
            frames: frames
        };

        // TODO: {T100} use scope resolution in the future
        // Manually load the builtin print function into the interpreter namespace
        // since rsnek does not have a concept of modules at this time.
        istate.ns.insert(String::from("print"), rt.get_builtin("print"));
        istate.ns.insert(String::from("len"), rt.get_builtin("len"));
        istate.ns.insert(String::from("type"), rt.get_builtin("type"));
        istate.ns.insert(String::from("str"), rt.get_builtin("str"));
        istate.ns.insert(String::from("int"), rt.get_builtin("int"));
        istate.ns.insert(String::from("any"), rt.get_builtin("any"));
        istate.ns.insert(String::from("all"), rt.get_builtin("all"));
        istate.ns.insert(String::from("list"), rt.get_builtin("list"));

        istate
    }

    fn pop_frame(&mut self)  {
        trace!("Interpreter"; "action" => "pop_frame", "idx" => self.frames.len() - 1);

        self.frames.pop_back();

        if self.frames.len() == 0 {
            Error::system(&format!(
                "Interpreter has no call frames, this is a bug; file: {}, line: {}",
                file!(), line!())).log()
        }

    }

    fn push_frame(&mut self, func: &ObjectRef) -> Result<usize, Error>{
        if self.frames.len() + 1 == RECURSION_LIMIT {
            return Err(Error::recursion())
        }

        let f_back = match self.frames.back() {
            Some(&(ref f_back, _)) => f_back.clone(),
            None => return Err(Error::system(&format!(
                "Interpreter has no call frames, this is a bug; file: {}, line: {}",
                file!(), line!())))
        };

        let new_frame = self.rt.frame(
            native::Frame {
                f_back: f_back,
                f_code: func.clone(),
                f_builtins: self.rt.none(),
                f_lasti: native::Integer::zero(),
                blocks: VecDeque::new()
            }
        );

        trace!("Interpreter"; "action" => "push_frame", "idx" => self.frames.len());
        self.frames.push_back((new_frame, RefCell::new(native::List::new())));

        Ok(self.frames.len())
    }

    fn push_stack(&mut self, objref: &ObjectRef)  {
        match self.frames.back() {
            Some(&(_, ref stack)) => stack.borrow_mut().push(objref.clone()),
            None => Error::system(&format!(
                "Interpreter has no call frames, this is a bug; file: {}, line: {}",
                file!(), line!())).log()
        }
    }

    fn pop_stack(&mut self) -> Option<ObjectRef> {
        match self.frames.back() {
            Some(&(_, ref stack)) => stack.borrow_mut().pop(),
            None => {
                let error = Error::system(&format!(
                    "Interpreter has no call frames, this is a bug; file: {}, line: {}",
                    file!(), line!()));
                error.log();
                panic!("{}", error);
            }
        }
    }

    fn stack_view(&self) -> Ref<native::List> {
        match self.frames.back() {
            Some(&(_, ref stack)) => stack.borrow(),
            None => {
                let error = Error::system(&format!(
                    "Interpreter has no call frames, this is a bug; file: {}, line: {}",
                    file!(), line!()));
                error.log();
                panic!("{}", error);
            }
        }
    }

    fn frame_view(&self) -> Box<[ObjectRef]> {
        let mut frames: native::List = native::List::new();

        for &(ref frame, _) in self.frames.iter() {
            frames.push(frame.clone())
        }

        frames.into_boxed_slice()
    }


    /// Used in lieu of an actual exception handling mechanism,
    /// clear all frames back to __main__ and clear that frame's
    /// value stack.
    fn clear_traceback(&mut self) {
        self.frames.truncate(1);
        self.frames[0].1.borrow_mut().clear();
    }


    fn exec_binop(&mut self, rt: &Runtime, opcode: OpCode, lhs: &ObjectRef, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = lhs.0.borrow();
        match opcode {
            OpCode::CompareEqual            => boxed.op_eq(&rt, &rhs),
            OpCode::LogicalAnd              => logical_and(rt, lhs, rhs),
            OpCode::LogicalOr               => logical_or(rt, lhs, rhs),
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
            opcode                           => Err(Error::system(
                &format!("Unhandled binary operation {:?}, this is a bug!", opcode))),
        }
    }

    fn force_pop(&mut self) -> Option<ObjectRef> {
        self.pop_stack()
    }

    /// Execute exactly one instruction
    fn exec_one(&mut self, rt: &Runtime, instr: &Instr) -> Option<RuntimeResult> {
        trace!("Interpreter"; "action" => "exec_one", "instr" => format!("{:?}", instr));

        match instr.tuple() {
            (OpCode::LoadConst, Some(value)) => {
                let objref = match value {
                    Native::Str(string) => rt.str(string),
                    Native::Int(i) => rt.int(i),
                    Native::Float(f) => rt.float(f),
                    Native::Bool(b) => rt.bool(b),
                    Native::Complex(_) => {
                        return Some(Err(Error::system(&format!(
                            "Interpreter does not implement Complex values; file: {}, line: {}",
                            file!(), line!()))))
                    },
                    Native::Code(code) => {

                        let func = native::Func {
                            name: code.co_name.clone(),
                            module: String::from("<compiled-module>"),
                            signature: code.co_varnames.as_slice().as_args(),
                            callable: native::FuncType::Code(code),
                        };

                        rt.function(func)
                    },
                    Native::None => rt.none(),
                    Native::Count(_) |
                    Native::List(_) => return Some(Err(Error::system(
                        &format!("Malformed LoadConst instruction {:?}, this is a bug!; file: {}; line: {}",
                                 instr, file!(), line!())))),
                };

                self.push_stack(&objref);
                None
            },
            (OpCode::StoreName, Some(value)) => {
                let name = match value {
                    Native::Str(string) => string,
                    _ => return Some(Err(
                        Error::runtime("Attempt to store a non string named value!")))
                };

                match self.pop_stack() {
                    Some(objref) => self.ns.insert(name, objref),
                    None => return Some(Err(
                        Error::runtime("No values in value stack to store!")))
                };

                None
            },
            (OpCode::LoadName, Some(value)) => {
                let name = match value {
                    Native::Str(string) => string,
                    _ => return Some(Err(
                        Error::runtime("Attempt to load a non string named value!")))
                };


                let to_push = match self.ns.get(&name) {
                    Some(objref) => objref.clone(),
                    None => return Some(Err(Error::name(&name)))
                };

                self.push_stack(&to_push);

                None
            },
            (OpCode::CompareEqual, None)            |
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

                let rhs = match self.pop_stack() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::system(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

                let lhs = match self.pop_stack() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::system(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

                // TODO: {T100} Give `Instr` getters
                let result = match self.exec_binop(rt, instr.tuple().0, &lhs, &rhs) {
                    Ok(objref) => objref,
                    err => return Some(err)
                };

                self.push_stack(&result);
                None
            },
            (OpCode::CallFunction, Some(Native::Count(arg_count))) => {
                let mut args: VecDeque<ObjectRef> = VecDeque::new();
                for _ in 0..(arg_count + 1) {
                    if self.stack_view().is_empty() {
                        return Some(Err(Error::system(
                            "Value stack did not contain enough values for function call!")));
                    }

                    args.push_front(self.pop_stack().unwrap());
                }

                let func = match args.pop_front() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::system("No values in value stack for call!")))
                };

                let boxed: &Box<Builtin> = func.0.borrow();
                let result = match boxed.deref(){
                    &Builtin::Function(ref pyfunc) => {
                        match pyfunc.value.0.callable {
                            FuncType::None => panic!(),
                            FuncType::Native(_) |
                            FuncType::Wrapper(_) => {
                                let pos_args = args.into_iter().collect::<Vec<ObjectRef>>();
                                match self.push_frame(&func) {
                                    Err(err) => Err(err),
                                    Ok(_) => {
                                        match boxed.op_call(&rt,
                                                            &rt.tuple(pos_args),
                                                            &rt.tuple(vec![]),
                                                            &rt.dict(native::None())) {

                                            Ok(next_tos) => {
                                                self.pop_frame();
                                                Ok(next_tos)
                                            },
                                            Err(err) => Err(err)
                                        }
                                    }
                                }
                            },
                            FuncType::Code(ref code) => {
                                // TODO: Fix this, this kind of stuff should
                                //   be part of a frame/scope
                                // Because overwriting the global namespace
                                //   with function args is always a good decision....
                                for (name, value) in code.co_names.iter().zip(args.iter()) {
                                    // unwrap here should be safe because of the previous check
                                    debug!("Namespace"; "action" => "insert", "key" => name, "value" => value.to_string());
                                    self.ns.insert(name.clone(), value.clone());
                                }

                                let ins = code.co_code.clone().into_iter().collect::<Vec<_>>();

                                match self.push_frame(&func) {
                                    Err(err) => Err(err),
                                    Ok(_) => {
                                        match self.exec(&rt, &ins) {
                                            Ok(_) => {
                                                let next_tos = match self.pop_stack() {
                                                    Some(objref) => objref,
                                                    None => self.rt.none()
                                                };
                                                self.pop_frame();
                                                Ok(next_tos)
                                            },
                                            Err(err) => Err(err),
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => Err(
                        Error::system(
                            &format!("{} {}; file: {}, line: {}",
                                     "Interpreter does not implement function calls ",
                                     "on non function types",
                                     file!(), line!())))
                };

                match result {
                    Ok(objref) => {
                        trace!("Interpreter"; "action" => "push_stack", "object" => format!("{:?}", objref.to_string()));
                        self.push_stack(&objref)
                     },
                    Err(err) => {
                        // TODO: When there is exception handling, this is a prime place to
                        // do the jump to the excepthandler
                        return Some(Err(err))
                    },
                };

                None
            }
            (OpCode::ReturnValue, None) => {
                None
            },
            (OpCode::MakeFunction, None) => {
                match self.pop_stack() {
                    Some(objref) => objref,
                    None => return Some(Err(Error::runtime(
                        &format!("No values in value stack for {:?}!", instr.tuple().0))))
                };

//                let code = match self.pop_stack() {
//                    Some(objref) => objref,
//                    None => panic!("No values in value stack for {:?}!", instr.tuple().0)
//                };
//
//
//                self.namespace.insert(name.to_string(), code);
                None
            },
            (OpCode::BuildList, Some(Native::Count(count))) => {
                // TODO: Change to list when impl'd
                let mut elems: native::Tuple = native::Tuple::new();
                for _ in 0..count {
                    if self.stack_view().is_empty() {
                        return Some(Err(Error::system(
                            "Value stack did not contain enough values for function call!")));
                    }

                    elems.insert(0, self.pop_stack().unwrap());
                }

                let objref = rt.list(elems);
                trace!("Interpreter"; "action" => "push_stack", "object" => format!("{:?}", objref));
                self.push_stack(&objref);
                Some(Ok(rt.none()))
            }
            (OpCode::PopTop, None) => {
                    match self.pop_stack() {
                        Some(objref) => Some(Ok(objref)),
                        None => None
                    }

            },
            (OpCode::AssertCondition, Some(Native::Count(arg_count))) => {
                let mut args: Vec<ObjectRef> = Vec::new();
                for _ in 0..arg_count {
                    if self.stack_view().is_empty() {
                        return Some(Err(Error::system(
                            "Value stack did not contain enough values for Assertion!")));
                    }

                    args.push(self.pop_stack().unwrap());
                }

                let test: ObjectRef;
                let mut message = "".to_string();

                match args.len() {
                    2 => {
                        test = args.pop().unwrap();
                        message = args.pop().unwrap().to_string();
                    },
                    1 => {
                        test = args.pop().unwrap();
                    },
                    _ => return Some(Err(Error::system(
                        "Value stack did not contain an expected number of values!")))
                }

                let boxed: &Box<Builtin> = test.0.borrow();
                let result = match boxed.op_bool(rt){
                    Ok(objref) => objref,
                    Err(err) => return Some(Err(err))
                };

                trace!("Interpreter"; "action" => "assert", "test_expr" => result.to_string());

                match result == rt.bool(true) {
                    true => None,
                    false => Some(Err(Error::assertion(&message)))
                }
            }
            _ => Some(Err(Error::not_implemented()))
        }
    }

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

    // TODO: This might be a decent thing to be part of Frame since
    // they should have all of the f_back pointers... maybe?
    pub fn format_traceback(&self) -> String {
        let frames: Box<[ObjectRef]> = self.frame_view();

        let names_result = (*frames).iter()
            .rev()
            .map(|ref f| {
                let boxed: &Box<Builtin> = f.0.borrow();

                match boxed.deref() {
                    &Builtin::Frame(ref pyframe) => Ok(pyframe.value.0.f_code.clone()),
                    other => Err(Error::system(
                        &format!("{} is not a Frame object", other.debug_name())))
                }
            }).map_results(|ref code| {
                let boxed: &Box<Builtin> = code.0.borrow();

                match boxed.deref() {
                    &Builtin::Code(ref pycode) => Ok(format!(
                        "<{} at {:?}>", pycode.value.0.co_name.clone(), (pycode as *const _))),
                    &Builtin::Function(ref pyfunc) => Ok(format!(
                        "<{} at {:?} - {}>", pyfunc.name(), (pyfunc as *const _), pyfunc.module())),
                    other => Err(Error::system(
                        &format!("{} is not a Code or Func object", other.debug_name())))
                }
            })
            .fold_results(
                Vec::new(),
                |mut acc, r| {acc.push(r.unwrap()); acc});

        match names_result {
            Ok(names) => format!("Traceback (most recent call first): \n>>> {}",
                                 names.join("\n>>> ")),
            Err(err) => format!("{:?}", err)
        }
    }

    pub fn log_state (&self) {
        // FIXME: Remove when parser/compiler allows for an expression of a single name
        // or maybe just hardcode that in?
        let stack: String = self.stack_view().iter().enumerate().map(|(idx, objref)| {
            let b: &Box<Builtin> = objref.0.borrow();
            match b.native_str() {
                Ok(s) => format!("{}: {} = {}", idx, b.debug_name(), s),
                Err(_) => format!("{}: {} = ??", idx, b.debug_name()),
            }
        }).collect::<Vec<String>>().join("\n");

        let values: Vec<String> = self.ns.iter().map(|(key, v): (&String, &ObjectRef)| {
            let b: &Box<Builtin> = v.0.borrow();

            match b.native_str() {
                Ok(ref s) if s.len() > 100  => format!("{}: {} = {}...", key, b.debug_name(), &s[..100]),
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
        let mut interpreter = InterpreterState::new(&rt);

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
            Ok(_) => {
                ExitCode::Ok as i64
            },
            Err(err) => {
                error!("{}", interpreter.format_traceback());
                error!("{:?}Error", err.0; "message" => err.1.clone());
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
    let mut interpreter = InterpreterState::new(&rt);
    let mut prompt_count = 0;

    print_banner();

    'repl: loop {
        match io::stdout().flush() {
            Err(err) => error!("Error Flushing STDOUT: {:?}", err),
            _ => ()
        }


        prompt_count += 1;
        info!("In[{}] {} ", prompt_count, resource::strings::PROMPT);

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
            Err(err) => {
                err.log();
                continue 'repl
            },
        };

        'process_ins: for i in ins.iter() {
            match interpreter.exec_one(rt, &i) {
                Some(Err(err)) => {

                    error!("{}", interpreter.format_traceback());
                    err.log();
                    interpreter.clear_traceback();
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
                    info!("\nOut[{}]: {}\n\n", prompt_count, string);
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
    use test::Bencher;
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

    // Simple equality
    assert_run!(one_equals_one,
        "assert 1 == 1, 'something is totally fucked'",
        ExitCode::Ok);

    // Sanity checks about and & or
    assert_run!(logical_and_01, r#"
f = 1 and 2
assert f, '1 and 2 failed unexpectedly'
"#, ExitCode::Ok);

    assert_run!(logic_and_02, r#"
f = 'true' and None
assert f, 'this should fail'
    "#, ExitCode::GenericError);

    assert_run!(logic_or_01, r#"
f = None or [1,2,3]
assert f, 'None or [1,2,3] failed unexpectedly'
    "#, ExitCode::Ok);

    assert_run!(logic_or_02, r#"
f = None or False
assert f, 'None or False failed as expected'
    "#, ExitCode::GenericError);

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


    // create and call a function
    assert_run!(func_01, r#"
def can_i_haz():
    return 'tests?'

result = can_i_haz()
assert result == 'tests?'
"#, ExitCode::Ok);

    assert_run!(func_02, r#"
def now_with_100_percent_more_args(a):
    return a * a

result = now_with_100_percent_more_args(13)
expected = 13 * 13
assert result == expected
"#, ExitCode::Ok);

    assert_run!(func_03, r#"
BIG_FACTOR = 314159

def biglyify_string(a):
    return a * BIG_FACTOR

tiny_string = 'abc'
big_string = biglyify_string(tiny_string)
print(len(big_string))
"#, ExitCode::Ok);

    assert_run!(list_01, r#"l = [4, 'hello', 34.22, None, True, False]"#, ExitCode::Ok);

    assert_run!(list__bool__01, r#"l = [1234]
assert l, "None empty list failed unexpectedly"
"#, ExitCode::Ok);

    assert_run!(list__bool__02, r#"l = []
assert l, "None empty list failed unexpectedly"
"#, ExitCode::GenericError);

    assert_run!(list__len__01, r#"
l = []
assert len(l), 'Should fail'
"#, ExitCode::GenericError);

    assert_run!(list__len__02, r#"
l = ['string of destiny']
assert len(l), 'Should not fail'
"#, ExitCode::Ok);

    assert_run!(list__mul__01, r#"
l = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] * len('I declare this list to be bigger!')
assert len(l), 'Should not fail'
"#, ExitCode::Ok);



    #[bench]
    fn print(b: &mut Bencher) {
        let rt = Runtime::new();
        let mut compiler = Compiler::new();
        let mut interpreter = InterpreterState::new(&rt);

        let code = "print(print(print(print(print(1)))))";
        let ins = match compiler.compile_str(&code) {
            Ok(ins) => ins,
            Err(_) => {
                panic!("SyntaxError: Unable to compile input");
            },
        };

        b.iter(|| {
            interpreter.exec(&rt, &(*ins)).unwrap()
        });
    }

}