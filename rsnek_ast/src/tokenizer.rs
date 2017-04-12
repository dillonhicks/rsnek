use std::collections::VecDeque;
use std::str;
use std::str::FromStr;
use std;
use std::rc::Rc;

use nom::{IResult,digit,multispace, newline};
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use num;

use token::{Id, Tk};

pub struct Tokenizer;

impl Tokenizer {
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
        identifier |
        unknown) >>
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

named!(unknown <Tk>, do_parse!(
    content: is_not!("\n") >>
    (Tk::Unknown(content))
));

named!(identifier <Tk>, do_parse!(
    ident: re_bytes_find_static!(r"[a-zA-Z_]([[:word:]]*)") >>
    ({
        match is_keyword(ident) {
            Some(id) => Tk::new(id, ident),
            None => Tk::Identifier(ident)
        }
    })
));

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
        tag!("\\") |
        tag!("\"") |
        tag!("'")
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
    fn simple() {
        let value = start_line(r"x = 1".as_bytes()).unwrap();
        pprint(&value.1);
    }

    fn module() {
        let value = start_line(
            r#"x += 24354353
  y = 3
  Q -> c
  x = [1, 2, 3, 4, 5];
  global KLINGON
  \

class Potato(Mike):
    def __init__(self):
        self.thing = 4
        self.more_things = 5

    def is_couch(self):
        return 'duh'

  "#.as_bytes()).unwrap();

        pprint(&value.1);

        println!("{:?}", value.1);
        println!("{}", serde_json::to_string(&value.1).unwrap());
        println!("{}", unsafe {String::from_utf8_unchecked(serde_pickle::to_vec(&value.1, true).unwrap())});
    }
}
