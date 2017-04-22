extern crate rsnek_compile;
extern crate nom;
extern crate time;

use std::io::prelude::*;
use std::io;
use std::env;
use std::borrow::Borrow;

use nom::{IResult, Needed};
use time::Duration;

use rsnek_compile::{LexResult, Lexer, Parser, ParserResult, Compiler};
use rsnek_compile::fmt;
use rsnek_compile::util::Micros;

const BANNER: &'static str = r#"
rsnek ast demo program
rustc 1.18.0-nightly (7627e3d31 2017-04-16)"#;

const PROMPT: &'static str = "§> ";


#[inline(always)]
fn print_banner() {
    println!("{}", BANNER);
}


#[inline(always)]
fn print_prompt() {
    print!("\n{}", PROMPT);
    io::stdout().flush().unwrap();
}


pub fn timed_tokenize<'a>(bytes: &'a [u8]) -> (Micros, LexResult<'a>) {
    let mut result = IResult::Incomplete(Needed::Unknown);
    let lexer = Lexer::new();
    let duration = match Duration::span(|| result = lexer.tokenize2(bytes))
        .num_microseconds() {
        Some(usecs) => Micros(usecs),
        None => Micros(-1)
    };

    (duration, result)
}


fn repl_loop() {
    let mut parser = Parser::new();
    let compiler = Compiler::new();

    let stdin = io::stdin();

    print_banner();
    print_prompt();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(t) => t,
            _ => {
                print_prompt();
                continue
            }
        };

        let (time, tokens) = match timed_tokenize(text.as_bytes()) {
            (t, IResult::Done(left, ref tokens)) if left.len() == 0 => {(t, tokens.clone())},
                _ => unreachable!()
        };

        println!("Lexer::tokenize took {}", time);
        println!("Tokens({}):\n----------------------------------------\n{}\n",
                 tokens.len(), fmt::tokens(&tokens, true));

        match parser.parse_tokens(&tokens) {
            ParserResult::Ok(ref result) => {
                println!("Ast\n, {}", fmt::json(&result));
                compiler.compile_ast(result.ast.borrow());
            },
            result => println!("\n\nERROR: {:#?}\n\n", result)
        }

        print_prompt()
    }
}

fn main() {

    if let Some(filename) = env::args().nth(1) {
        let parser = Parser::new();

        parser.parse_file(filename.as_str());
    } else {
        repl_loop()
    }

}