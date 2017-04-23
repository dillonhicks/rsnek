extern crate rsnek_runtime;
extern crate clap;
extern crate log;
extern crate log4rs;

use log::LogLevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};

use clap::{Arg, App, ArgGroup};

use rsnek_runtime::resource::strings;
use rsnek_runtime::runtime::{Interpreter, Config, ThreadModel, Logging, Mode};


fn main() {

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

    // Setup Logging
    let stdout = ConsoleAppender::builder().build();

    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LogLevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();


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
