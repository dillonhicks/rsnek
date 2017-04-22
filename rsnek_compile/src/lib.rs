#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(private_in_public)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]


#![feature(try_from)]
#![feature(const_fn)]
#![feature(associated_consts)]
#![feature(custom_attribute)]

extern crate num;
extern crate encoding;
extern crate time;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate serde_bytes;
extern crate serde_pickle;
extern crate bincode;

#[macro_use] extern crate itertools;
#[macro_use] extern crate nom;

mod token;
mod slice;
#[macro_use] mod macros;
mod traits;

mod lexer;
mod parser;
mod ast;
pub mod compiler;

pub mod util;
pub mod fmt;
pub use lexer::{Lexer, LexResult};
pub use parser::{Parser, ParserResult, ParsedAst};
pub use compiler::{Instr, Compiler};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
