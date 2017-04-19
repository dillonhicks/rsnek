extern crate rsnek_runtime;
extern crate clap;

use clap::{Arg, App, SubCommand, ArgGroup};

use rsnek_runtime::resource::strings;
use rsnek_runtime::runtime::{Interpreter, Config, ThreadModel, Logging, Argv};


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

    let config = Config {
        // TODO: it is possible in cpython to force interactive mode after script executes
        interactive: args.is_empty(),
        arguments: args.as_slice(),
        thread_model: thread_model,
        logging: logging,
        debug_support: debug_support,
    };

    
    Interpreter::run(&config);
}
