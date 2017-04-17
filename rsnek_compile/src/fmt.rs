use serde::Serialize;
use serde_json;


use ast::Ast;
use token::{Tk, Id};

pub fn pretty<'a, T: Serialize>(input: &'a T) -> String {
    match serde_json::to_string_pretty(&input) {
        Ok(string) => string,
        Err(err) => format!("{:?}", err)
    }
}

pub fn token(t: &Tk) -> String {
    match t.id() {
        Id::Space |
        Id::Tab   |
        Id::Newline => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), format!("{:?}", String::from_utf8_lossy(t.bytes())), format!("{:?}", t.tag())),
        _ => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), String::from_utf8_lossy(t.bytes()), format!("{:?}", t.tag()))
    }
}


pub fn tokens(tokens: &[Tk], filter_spaces: bool) -> String {
    let result: Vec<String> = tokens.iter().enumerate().map(|(idx, t)| {
        match (filter_spaces, t.id()) {
            (true, Id::Space) => "".to_string(),
            _ => format!("{:>3}: {}", idx, token(t))
        }
    }).filter(|s| !s.is_empty()).collect();

    result.join("\n")
}


pub fn ast<'a>(input: &Ast<'a>) -> String {
    pretty(&input)
}
