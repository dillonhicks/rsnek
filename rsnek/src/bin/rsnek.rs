//! The "main" entrypoint for rsnek
//!
#[macro_use(o, kv, slog_info, slog_log, slog_record, slog_b, slog_kv, slog_record_static)]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate slog_scope;

use slog::Drain;
use std::sync::Arc;


extern crate clap;
extern crate rsnek;


use clap::{Arg, App, ArgGroup};

use rsnek::resources::strings;
use rsnek::runtime::{Interpreter, Config, Logging, Mode};
use rsnek::system::ThreadModel;


fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    /// Chan size should probably be at least the recursion limit so it doesn't overflow
    /// when a python script gets into super recursion
    let drain = slog_async::Async::new(drain).chan_size(8 * 1024).build();
    let drain = drain.filter_level(slog::Level::Trace).fuse();

    let log = slog::Logger::root(Arc::new(drain.fuse()), o!());
    let _guard = slog_scope::set_global_logger(log);

    info!("{}", format!("welcome to {}", strings::PROGRAM));

    let matches = App::new(strings::PROGRAM)
        .version(strings::VERSION)
        .author(strings::AUTHORS)
        .about(strings::ABOUT)
        .arg(Arg::with_name("cmd")
            .short("c")
            .help("program passed in as string")
            .takes_value(true))
        .arg(Arg::with_name("mod")
            .short("m")
            .help("run library module as a script")
            .takes_value(true))
        .arg(Arg::with_name("green_threads")
            .short("g")
            .long("green-threads")
            .help("Sets the threading model")
            .takes_value(false))
        .arg(Arg::with_name("args")
            .index(1)
            .multiple(true))
        .group(
            ArgGroup::with_name("input")
                .args(&["cmd", "mod",]))
        .get_matches();


    // Parse Args
    let matched_args = matches.values_of_lossy("args");

    let args: Vec<&str> = matched_args.iter()
        .flat_map(|v| v.iter())
        .map(|s| s.as_str())
        .collect();

    let logging = Logging{};
    let debug_support = false;

    let thread_model = match matches.is_present("green_threads") {
        true => ThreadModel::GreenThreads,
        false => ThreadModel::OsThreads
    };

    let ms = (matches.value_of("cmd"), matches.value_of("mod"), args.get(0));

    let mode = match ms {
        (Some(cmd), None, None)     => Mode::Command(cmd.to_string()),
        (None, Some(module), None)  => Mode::Module(module.to_string()),
        (None, None, Some(_))       => Mode::File,
        (None, None, None)          => Mode::Interactive,
        _                           => {
            println!("{}", matches.usage());
            return
        }
    };

    // Build Interpreter Config
    let config = Config {
        // TODO: {T89} force interactive mode after script executes
        mode: mode,
        arguments: args.as_slice(),
        thread_model: thread_model,
        logging: logging,
        debug_support: debug_support,
    };


    Interpreter::run(&config);
}
