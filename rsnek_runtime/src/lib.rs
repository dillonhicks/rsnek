#![recursion_limit="4000"]
#![feature(associated_consts)]
#![feature(type_ascription)]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]
#![feature(test)]

extern crate test;

#[macro_use(slog_info, slog_log, slog_record, slog_b, slog_crit, slog_trace, slog_debug, slog_error, slog_warn, slog_kv, slog_record_static)]
extern crate slog;
#[macro_use]
extern crate slog_scope;

extern crate num;
extern crate itertools;
extern crate fringe;
extern crate rustyline;

#[macro_use]
extern crate serde_derive;
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
