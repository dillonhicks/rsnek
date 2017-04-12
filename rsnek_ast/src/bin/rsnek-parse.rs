use std::env;
extern crate rsnek_ast;

use rsnek_ast::parser::Parser;

fn main() {
    if let Some(filename) = env::args().nth(1) {

        let parser = Parser::new();
        parser.parse_file(filename.as_str());
    }

}