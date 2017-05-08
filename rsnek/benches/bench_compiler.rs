#![feature(test)]
extern crate test;
extern crate python_ast;
extern crate librsnek;
extern crate itertools;

use std::borrow::Borrow;

use itertools::Itertools;
use test::Bencher;
use python_ast::fmt;
use python_ast::{Ast, Lexer, LexResult, Parser, ParserResult};

use librsnek::compiler::{Compiler, CompilerResult};


pub fn setup<'a>(text: &str) ->  Ast {
    let lexer = Lexer::new();
    let mut parser = Parser::new();

    let tokens = match lexer.tokenize2(text.as_bytes()) {
        LexResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
        _ => unreachable!()
    };

    match parser.parse_tokens(&tokens) {
        ParserResult::Ok(ref result) if result.remaining_tokens.len() == 0 => {
            result.ast.clone()
        },
        result => panic!("\n\nERROR: {}\n\n", fmt::json(&result))
    }
}


#[cfg(test)]
mod stmt_funcdef {
    use super::*;

    macro_rules! func_args (
        ($name:ident, $N:expr) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let ast = setup(&format!(r#"
def function_{}_args({}):
    x = 1
"#, $N, (0..$N).map(|i| format!("arg{}", i)).join(", ")));

                b.iter(|| Compiler::new().compile_ast(&ast).unwrap());

            }

        );
    );

    func_args!(args_0,      0);
    func_args!(args_1,      1);
    func_args!(args_4,      4);
    func_args!(args_16,     16);
    func_args!(args_64,     64);
    func_args!(args_256,    256);
    func_args!(args_1024,   1024);
    func_args!(args_4096,   4046);
}


