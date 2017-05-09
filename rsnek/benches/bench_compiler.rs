#![feature(test)]
extern crate test;
extern crate python_ast;
extern crate rsnek;
extern crate itertools;

use std::borrow::Borrow;

use itertools::Itertools;
use test::Bencher;
use python_ast::fmt;
use python_ast::{Ast, Lexer, LexResult, Parser, ParserResult};

use rsnek::compiler::{Compiler, CompilerResult};


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

macro_rules! compiler_benchmark (
    ($name:ident, $code:expr) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let ast = setup($code);
            b.iter(|| Compiler::new().compile_ast(&ast).unwrap());
        }
    );
);


#[cfg(test)]
mod stmt_funcdef {
    use super::*;

    macro_rules! def_function (
        ($name:ident, $N:expr) => (
            compiler_benchmark!($name, &format!(r#"
def function_{}_args({}):
    x = 1
"#, $N, (0..$N).map(|i| format!("arg{}", i)).join(", ")));
        );
    );

    def_function!(args_0000,      0);
    def_function!(args_0001,      1);
    def_function!(args_0002,      2);
    def_function!(args_0003,      3);
    def_function!(args_0004,      4);
    def_function!(args_0005,      5);
    def_function!(args_0006,      6);
    def_function!(args_0007,      7);
    def_function!(args_0008,      8);
    def_function!(args_0016,     16);
    def_function!(args_0064,     64);
    def_function!(args_0256,    256);
    def_function!(args_1024,   1024);
    def_function!(args_4096,   4046);

    fn format_nested_funcdef(curr: usize, max: usize) -> String {
        let indent  = "    ".repeat(curr);

        let begin  = indent.clone() + &format!("def nested_{}():", curr);
        let mut body = String::new();
        if curr < max {
            body = format_nested_funcdef(curr + 1, max);
        }
        let end = indent + "    return 1";

        format!("{}\n{}{}\n", begin, body, end)
    }
    
    macro_rules! def_nested_function (
        ($name:ident, $N:expr) => (
            compiler_benchmark!($name, &format_nested_funcdef(0, $N));
        );
    );

    def_nested_function!(nested_funcs_0000,      0);
    def_nested_function!(nested_funcs_0001,      1);
    def_nested_function!(nested_funcs_0002,      2);
    def_nested_function!(nested_funcs_0003,      3);
    def_nested_function!(nested_funcs_0004,      4);
    def_nested_function!(nested_funcs_0005,      5);
    def_nested_function!(nested_funcs_0006,      6);
    def_nested_function!(nested_funcs_0007,      7);
    def_nested_function!(nested_funcs_0008,      8);
    def_nested_function!(nested_funcs_0016,     16);

}


#[cfg(test)]
mod stmt_expr_call {
    use super::*;

    macro_rules! call_function (
        ($name:ident, $N:expr) => (
            compiler_benchmark!($name, &format!(r#"
function({})
"#, (0..$N).map(|i| format!("{}", i)).join(", ")));
        );
    );

    call_function!(args_0000,      0);
    call_function!(args_0001,      1);
    call_function!(args_0002,      2);
    call_function!(args_0003,      3);
    call_function!(args_0004,      4);
    call_function!(args_0005,      5);
    call_function!(args_0006,      6);
    call_function!(args_0007,      7);
    call_function!(args_0008,      8);
    call_function!(args_0016,     16);
    call_function!(args_0064,     64);
    call_function!(args_0256,    256);
    call_function!(args_1024,   1024);
    call_function!(args_4096,   4046);

}



#[cfg(test)]
mod stmt_assign {
    use super::*;

    macro_rules! assign(
        ($name:ident, $code:expr) => (
            compiler_benchmark!($name, $code);
        );
    );

    assign!(x_to_bool,      r#"x = None"#);
    assign!(x_to_int,       r#"x = 1"#);
    assign!(x_to_float,     r#"x = 1.0"#);
    assign!(x_to_str,       r#"x = 'string cheese'"#);
    assign!(x_to_list,      r#"x = [a, b, c, d]"#);
    assign!(x_to_dict,      r#"x = {"green": "eggs", "and": "ham"}"#);
    assign!(x_to_call,      r#"x = globals()"#);
    assign!(x_to_name,      r#"x = variable_name"#);
    assign!(x_to_binop,     r#"x = 12345 + 67890"#);

}


#[cfg(test)]
mod expr_attr {
    use super::*;

    macro_rules! chained_attr(
        ($name:ident, $N:expr) => (
            compiler_benchmark!($name, &format!(r#"
self.{}
"#, (0..$N).map(|i| format!("attr{}", i)).join(".")));
        );
    );

    chained_attr!(attrs_0001,      1);
    chained_attr!(attrs_0002,      2);
    chained_attr!(attrs_0003,      3);
    chained_attr!(attrs_0004,      4);
    chained_attr!(attrs_0005,      5);
    chained_attr!(attrs_0006,      6);
    chained_attr!(attrs_0007,      7);
    chained_attr!(attrs_0008,      8);
    chained_attr!(attrs_0016,     16);
    chained_attr!(attrs_0064,     64);
    chained_attr!(attrs_0256,    256);
}

#[cfg(test)]
mod expr_binop {
    use super::*;

    macro_rules! chained_binop(
        ($name:ident, $N:expr) => (
            compiler_benchmark!($name, &format!(r#"
{}
"#, (0..$N).map(|i| format!("var{}", i)).join(" + ")));
        );
    );

    chained_binop!(ops_0002,      2);
    chained_binop!(ops_0003,      3);
    chained_binop!(ops_0004,      4);
    chained_binop!(ops_0005,      5);
    chained_binop!(ops_0006,      6);
    chained_binop!(ops_0007,      7);
    chained_binop!(ops_0008,      8);
    chained_binop!(ops_0016,     16);
    chained_binop!(ops_0064,     64);
}

