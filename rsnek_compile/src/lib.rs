#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![allow(private_in_public)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

#![feature(const_fn)]
#![feature(associated_consts)]
#![feature(custom_attribute)]

extern crate num;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate serde_bytes;
extern crate serde_pickle;
extern crate bincode;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate regex;
#[macro_use] extern crate itertools;
#[macro_use] extern crate nom;
#[macro_use] extern crate rsnek_proc_macros;

mod token;
pub mod tokenizer;
pub mod parser;
mod ast;

pub use tokenizer as lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
