//! Compiles ASTs into Interpreter Recognizable Instructions
mod compiler;
mod graph;
mod symbol;
mod scope;

pub use self::compiler::{Compiler, CompilerResult, Context};
