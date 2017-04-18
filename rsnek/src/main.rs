extern crate rsnek_runtime;
extern crate clap;

use clap::{Arg, App, SubCommand};
use rsnek_runtime::resource::strings;
use rsnek_runtime::runtime::{Interpreter, Config, ThreadModel, Logging};


fn main() {
    // TODO: Add yaml spec for the args later
    let matches = App::new(strings::PROGRAM)
        .version(strings::VERSION)
        .author(strings::AUTHORS)
        .about(strings::ABOUT)
        .arg(Arg::with_name("green_threads")
            .short("g")
            .long("green-threads")
            .help("Sets the threading model")
            .takes_value(false))
        .get_matches();

    let interactive = true;
    let logging = Logging{};
    let debug_support = false;

    let thread_model = match matches.is_present("green_threads") {
        true => ThreadModel::GreenThreads,
        false => ThreadModel::OsThreads
    };

    let config = Config {
        interactive: interactive,
        thread_model: thread_model,
        logging: logging,
        debug_support: debug_support,
    };

    Interpreter::run(&config);
}
