#![feature(associated_consts)]
#![feature(type_ascription)]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]
#![feature(test)]
#![feature(fn_traits)]

extern crate test;
extern crate num;
extern crate itertools;
extern crate fringe;
extern crate rustyline;

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate rsnek_compile;

#[macro_use]
mod macros;



mod builtin;
mod traits;
mod object;
mod result;
mod error;
mod typedef;

pub mod resource;

#[macro_use]
pub mod runtime;
