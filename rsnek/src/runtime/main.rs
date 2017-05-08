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
use itertools::Itertools;
use num::Zero;
use rustyline::CompletionType;
use rustyline::Config as RLConfig;
use rustyline::error::ReadlineError;
use rustyline;

use python_ast::fmt;

use ::api::result::Error;
use ::api::result::ObjectResult;
use ::api::RtObject;
use ::compiler::Compiler;
use ::modules::builtins::{Type, logical_and, logical_or};
use ::resources::strings;
use ::runtime::config::Mode;
use ::runtime::Interpreter;
use ::runtime::{OpCode, Runtime};
use ::system::primitives as rs;
use ::system::primitives::SignatureBuilder;
use ::system::primitives::{Native, Instr, FuncType};
use ::system::{Argv, ExitCode, MainFnRef, MainFn, SharedMainFnRef};

/// Two equally valid explanations exist for this banner
///
/// 1. Print the obligatory banner to let everyone know what program is running.
/// 2. https://youtu.be/tHnA94-hTC8?t=2m47s
#[inline(always)]
fn print_banner() {
    info!("\n{}", strings::BANNER2);
    info!("{}", strings::VERSION);
    info!("{}", strings::BUILD);
}


/// Create the closure with the `MainFn` signature that captures a copy
/// of the arguments sent to `create_python_main`. The closure will try to use
/// the first argument as the file to load.
pub fn create_python_main(mode: Mode, args: Argv) -> Box<MainFn> {

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
        let mut interpreter = Interpreter::new(&rt);

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
pub fn python_main_interactive(rt: &Runtime) -> i64 {

    let config = RLConfig::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl = rustyline::Editor::<()>::with_config(config);
    let mut interpreter = Interpreter::new(&rt);
    let mut prompt_count = 0;

    print_banner();

    'repl: loop {
        match io::stdout().flush() {
            Err(err) => error!("Error Flushing STDOUT: {:?}", err),
            _ => ()
        }


        prompt_count += 1;
        info!("In[{}] {} ", prompt_count, strings::PROMPT);

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


        let mut compiler = Compiler::new();
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

        interpreter.log_state();
    }

    ExitCode::Ok as i64
}
