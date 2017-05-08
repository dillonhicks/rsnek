//! rsnek - An Implementation of Python in Rust
//!
#![feature(associated_consts)]
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]
#![feature(range_contains)]
#![feature(test)]
#![feature(try_from)]
#![feature(type_ascription)]
#![recursion_limit="4000"]

// Third Party Extern Crates
#[macro_use(slog_info, slog_log, slog_record, slog_b, slog_crit, slog_trace, slog_debug, slog_error, slog_warn, slog_kv, slog_record_static)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate fringe;
extern crate itertools;
extern crate num;
extern crate num_bigint;
extern crate rustyline;
extern crate serde;
extern crate test;

// First Party Extern Crates
extern crate python_ast;

// Private Modules for $crate
#[macro_use]
mod macros;
mod api;
mod modules;
mod objects;

// Public modules for $crate
pub mod compiler;
pub mod system;
pub mod resources;
pub mod runtime;
