#![feature(associated_consts)]
#![feature(rustc_private)]

extern crate num;
extern crate arena;


#[macro_use]
pub mod log;
pub mod typedef;
pub mod opcode;
pub mod instruction;
pub mod result;
pub mod heap;
pub mod runtime;
pub mod error;
