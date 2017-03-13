#![feature(associated_consts)]
#![feature(rustc_private)]

extern crate num;
extern crate arena;


#[macro_use]
pub mod log;

pub mod opcode;
pub mod instruction;
pub mod object;
pub mod integer;
pub mod result;
pub mod builtin;
pub mod heap;
pub mod runtime;
pub mod float;
pub mod error;
pub mod map;
pub mod list;
pub mod string;
pub mod tuple;
pub mod dictionary;