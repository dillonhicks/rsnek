#![feature(test)]

extern crate test;
extern crate num;
extern crate python_ast;
extern crate nom;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use test::Bencher;

use python_ast::Lexer;


fn text(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}


#[bench]
fn lex_warnings(b: &mut Bencher) {
    let txt = text("./benches/python3.6/warnings.py");
    println!("Text Size: {}", txt.len());

    let lexer = Lexer::new();

    b.iter(|| {
       lexer.tokenize(&txt.as_bytes());
    });

}


#[bench]
fn lex_topics(b: &mut Bencher) {
    let mut txt = text("./benches/python3.6/topics.py");
    txt = (1..5).map(|_| txt.clone()).collect();
    println!("Text Size: {}", txt.len());

    let lexer = Lexer::new();

    b.iter(|| {
        lexer.tokenize(&txt.as_bytes());
    });
}
