use serde::Serialize;
use serde_json;

use token::{Tk, Id};


/// Take some input that implements `serde::Serialize` and convert it to
/// pretty json format.
pub fn json<'a, T: Serialize>(input: &'a T) -> String {
    match serde_json::to_string_pretty(&input) {
        Ok(string) => string,
        Err(err) => format!("{:?}", err)
    }
}

/// Take a token and output it in tabular format using text alignments
/// for debugging. The string will take the form:
/// `Id        String           Tag`
pub fn token(t: &Tk) -> String {
    match t.id() {
        Id::Space       |
        Id::Tab         |
        Id::BlockStart  |
        Id::BlockEnd    |
        // TODO: {T90} Cleanup token text formatting
        Id::Newline     => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), format!("{:?}", String::from_utf8_lossy(t.bytes())), format!("{:?}", t.tag())),
        _ => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), String::from_utf8_lossy(t.bytes()), format!("{:?}", t.tag()))
    }
}

/// Like `fmt::token` except take a slice of tokens. optionally filtering the spaces,
/// and join the result of the calls to `fmt::token` with the index of the token and a
/// newlines to get a big token table.
pub fn tokens(tokens: &[Tk], filter_spaces: bool) -> String {
    let result: Vec<String> = tokens.iter().enumerate().map(|(idx, t)| {
        match (filter_spaces, t.id()) {
            (true, Id::Space) => "".to_string(),
            _ => format!("{:>3}: {}", idx, token(t))
        }
    }).filter(|s| !s.is_empty()).collect();

    result.join("\n")
}
