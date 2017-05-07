#![feature(associated_consts)]
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]
#![feature(range_contains)]
#![feature(test)]
#![feature(try_from)]
#![feature(type_ascription)]
#![recursion_limit="4000"]

#[macro_use(slog_info, slog_log, slog_record, slog_b, slog_crit, slog_trace, slog_debug, slog_error, slog_warn, slog_kv, slog_record_static)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
#[macro_use]
extern crate serde_derive;

extern crate fringe;
extern crate itertools;
extern crate num;
extern crate num_bigint;
extern crate rustyline;
extern crate serde;
extern crate test;

extern crate rsnek_compile;

#[macro_use]
mod macros;
mod api;
mod compiler;
mod modules;
mod objects;
#[macro_use]
pub mod runtime;
mod system;
mod traits;
pub mod resources;
