#![feature(const_fn)]
#![feature(associated_consts)]
#![feature(custom_attribute)]
#![feature(box_syntax)]

#[macro_use(slog_o, slog_trace, slog_log, slog_error, slog_record, slog_b, slog_warn, slog_kv, slog_record_static)]
extern crate slog;
#[macro_use]
extern crate slog_scope;

extern crate num;
extern crate time;
extern crate itertools;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_bytes;

#[macro_use]
extern crate nom;

mod token;
mod slice;
#[macro_use]
mod macros;

mod doc;
mod traits;
mod lexer;
mod parser;
mod ast;
mod preprocessor;

pub mod util;
pub mod fmt;

pub use ast::{Ast, Module, Stmt, Expr, Op};
pub use token::{Tk, OwnedTk, Id, Tag, Num};
pub use lexer::{Lexer, LexResult};
pub use parser::{Parser, ParserResult, ParsedAst};
