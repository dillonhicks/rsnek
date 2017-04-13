use std::env;
extern crate rsnek_compile;

use rsnek_compile::parser::Parser;

fn main() {
    if let Some(filename) = env::args().nth(1) {

        let parser = Parser::new();
        parser.parse_file(filename.as_str());
    }

}