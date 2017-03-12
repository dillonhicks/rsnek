#![feature(associated_consts)]

extern crate num;


#[macro_use]
pub mod log;

pub mod opcode;
pub mod instruction;
pub mod object;
pub mod integer;
pub mod result;
pub mod builtin;
pub mod arena;
pub mod runtime;
pub mod float;
pub mod error;
