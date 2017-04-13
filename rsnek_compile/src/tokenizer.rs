
use std::collections::VecDeque;
use std::str;
use std::str::FromStr;
use std;
use std::rc::Rc;

use nom::{IResult,digit,multispace, newline, Needed};
use std::fmt::Debug;
//use nom::internal::*;
//use nom::internal::IResult::*;
use nom::ErrorKind;
use nom::{AsChar,InputLength,InputIter};
use nom::{Compare,CompareResult,Slice};

use std::ops::{Range,RangeFrom,RangeTo};

use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use num;

use token::{Id, Tk, pprint_tokens};

pub struct Lexer;

impl Lexer {
    pub fn tokenize(bytes: &[u8]) -> Rc<IResult<&[u8], Vec<Tk>>> {
       let result = start_line(bytes);
        Rc::new(result)
    }
}

named!(start_line <Vec<Tk>>, do_parse!(
    tokens: many0!(line) >>
    (tokens)
));

named!(line <Tk>, do_parse!(
    token: alt_complete!(
        space |
        endline |
        symbol |
        operator |
        number |
        string |
        identifier |
        error_marker) >>
    (token)
));

named!(number <Tk>, do_parse!(
    digits : digit >>
    (Tk::Number(digits))
));

named!(endline <Tk>, do_parse!(
    nl: tag!("\n") >>
    (Tk::NewLine(nl))
));

named!(space <Tk>, do_parse!(
    sp: is_a!(" ") >>
    (Tk::Space(sp))
));

named!(error_marker <Tk>, do_parse!(
    content: take!(1) >>
    ({
        println!("error: {:?}", content);
        Tk::new(Id::ErrorMarker, content)
    })
));

named!(identifier <Tk>, do_parse!(
    name: call!(ident) >>
    ({
        println!("identifier: {:?}", String::from_utf8_lossy(name));
        match is_keyword(name) {
            Some(id) => Tk::new(id, name),
            None => Tk::Identifier(name)
        }
    })
));

pub trait Ident: where Self: AsChar {
    fn is_ident_start(&self) -> bool;
    fn is_ident(&self) -> bool;
}


impl Ident for u8 {
    fn is_ident_start(&self) -> bool {
        self.is_alpha() || (self.as_char() == '_' )
    }

    fn is_ident(&self) -> bool {
        self.is_alphanum() || (self.as_char() == '_' )
    }
}


/// Recognizes one or more numerical and alphabetic characters: 0-9a-zA-Z
pub fn ident(input: &[u8]) -> IResult<&[u8],&[u8]> {

    let input_length = input.input_len();

    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }

    for (idx, item) in input.iter_indices() {
        match (idx, item.is_ident_start(), item.is_ident()) {
            (0, true, _) => continue,
            (0, false, _) => return IResult::Error(error_position!(ErrorKind::AlphaNumeric, input)),
            (_, _, true) => continue,
            _ => return IResult::Done(input.slice(idx..), input.slice(0..idx))
        }
    }

    IResult::Done(input.slice(input_length..), input)
}


named!(string <Tk>, do_parse!(
    result: call!(sublex_string) >>
    (Tk::new(Id::String, result))
));


named!(sublex_string <&[u8]>, do_parse!(
    result: switch!(peek!(take!(1)),
        b"'" => call!(sublex_squote_string)   |
        b"\"" => call!(sublex_dquote_string)  |
        b"r" => call!(sublex_prefixed_string) |
        b"b" => call!(sublex_prefixed_string) |
        b"f" => call!(sublex_prefixed_string)
    ) >>
    (result)
));


named!(sublex_prefixed_string <&[u8]>,  do_parse!(
    result: preceded!(
        take!(1),
        call!(sublex_string)) >>
        (result)
));


named!(sublex_squote_string <&[u8]>, do_parse!(
    result: switch!(peek!(take!(3)),
        b"'''" => recognize!(delimited!(take!(3), take_until!("'''"), take!(3))) |
        _ => recognize!(delimited!(tag!("'"), take_until!("'"), tag!("'")))
    ) >>
    (result)
));

named!(sublex_dquote_string <&[u8]>, do_parse!(
    //result: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
    result: switch!(peek!(take!(3)),
        b"\"\"\"" => recognize!(delimited!(take!(3), take_until!("\"\"\""), take!(3))) |
        _ => recognize!(delimited!(tag!("\""), take_until!("\""), tag!("\"")))
    ) >>
    (result)
));


//
//named!(std_string <&[u8]>, do_parse!(
//    start: tag!("\"")     >>
//    middle: is_not!("\"\n")  >>
//    end: tag!("\"") >>
//    ([start, middle, end].concat())
//));
//
//
//named!(r_string <&[u8]>, do_parse!(
//    start: tag!("r\"")     >>
//    middle: is_not!("\"\n")  >>
//    end: tag!("\"") >>
//    ([start, middle, end].concat())
//));
//
//
//named!(b_string <&[u8]>, do_parse!(
//    start: tag!("r\"")     >>
//    middle: is_not!("\"\n")  >>
//    end: tag!("\"") >>
//    ([start, middle, end].concat())
//));
//
//named!(f_string <&[u8]>, do_parse!(
//    start: tag!("r\"")     >>
//    middle: is_not!("\"\n")  >>
//    end: tag!("\"") >>
//    ([start, middle, end].concat())
//));


named!(symbol <Tk>, do_parse!(
    sym: alt!(
        tag!("(") |
        tag!("[") |
        tag!("{") |
        tag!("}") |
        tag!("]") |
        tag!(")") |
        tag!(",") |
        tag!(";") |
        tag!(":") |
        tag!("\\")
//        tag!("\"") |
//        tag!("'")
    ) >>
    (Tk::Symbol(sym))
));


named!(operator <Tk>, do_parse!(
    op: alt_complete!(
            tag!(r"<<=") |
            tag!(r">>=") |
            tag!(r"**=") |
            tag!(r"//=") |
            tag!(r"...") |
            tag!(r"==") |
            tag!(r"!=") |
            tag!(r"<>") |
            tag!(r"<=") |
            tag!(r"<<") |
            tag!(r">=") |
            tag!(r">>") |
            tag!(r"+=") |
            tag!(r"-=") |
            tag!(r"->") |
            tag!(r"**") |
            tag!(r"*=") |
            tag!(r"//") |
            tag!(r"/=") |
            tag!(r"|=") |
            tag!(r"%=") |
            tag!(r"&=") |
            tag!(r"^=") |
            tag!(r"@=") |
            tag!(r"(r") |
            tag!(r")") |
            tag!(r"[") |
            tag!(r"]") |
            tag!(r":") |
            tag!(r",") |
            tag!(r";") |
            tag!(r"+") |
            tag!(r"-") |
            tag!(r"*") |
            tag!(r"/") |
            tag!(r"|") |
            tag!(r"&") |
            tag!(r"<") |
            tag!(r">") |
            tag!(r"=") |
            tag!(r".") |
            tag!(r"%") |
            tag!(r"{") |
            tag!(r"}") |
            tag!(r"^") |
            tag!(r"~") |
            tag!(r"@") |
            tag!(r".")) >>
       (Tk::Operator(op))
));

fn is_keyword(bytes: &[u8]) -> Option<Id> {
    let string = match str::from_utf8(bytes) {
        Ok(string) => string,
        err => return None
    };

    match string {
        "False"    |
        "None"     |
        "True"     |
        "and"      |
        "as"       |
        "assert"   |
        "break"    |
        "class"    |
        "continue" |
        "def"      |
        "del"      |
        "elif"     |
        "else"     |
        "except"   |
        "finally"  |
        "for"      |
        "from"     |
        "global"   |
        "if"       |
        "import"   |
        "in"       |
        "is"       |
        "lambda"   |
        "nonlocal" |
        "not"      |
        "or"       |
        "pass"     |
        "raise"    |
        "return"   |
        "try"      |
        "while"    |
        "with"     |
        "yield"    => Some(Id::Keyword),
        _ => None
    }
}



#[cfg(test)]
mod _api{
    use super::*;
    use serde_yaml;
    use serde_json;
    use serde_pickle;

    #[test]
    fn tk_space() {
        let value = start_line(r#" "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_number() {
        let value = start_line(r#"12345"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_symbol() {
        let value = start_line(r#":"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_operator() {
        let value = start_line(r#"+="#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_string() {
        let value = start_line(r#"  "abc'"  "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_squote_string() {
        let value = start_line(r#"  'def'  "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_string_unicode() {
        let value = start_line(r#"  "שּׂθשּׂઊ" "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_name() {
        let value = start_line(r#" _hello "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_keyword() {
        let value = start_line(r#" def "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn expr_x_eq_1() {
        let value = start_line(r#"x = 1"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn module() {
        let value = start_line(
            r#"x += 24354353
  y = 3
  Q -> c
  x = [1, 2, 3, 4, 5];
  global KLINGON
  \
θθθ
θclass Potato(Mike):
    def __init__(self):
        self.thing = 4
        self.more_things = 5

    def is_couch(self):
        return 'duh'

  "#.as_bytes()).unwrap();

        pprint_tokens(&value.1);

//        println!("{:?}", value.1);
//        println!("{}", serde_json::to_string(&value.1).unwrap());
//        println!("{}", unsafe {String::from_utf8_unchecked(serde_pickle::to_vec(&value.1, true).unwrap())});
    }
}
