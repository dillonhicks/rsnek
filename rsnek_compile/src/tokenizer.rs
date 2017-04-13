use std;
use std::str;
use std::str::FromStr;
use std::rc::Rc;
use std::fmt::Debug;
use std::collections::VecDeque;
use std::ops::{Range,RangeFrom,RangeTo};

use nom::{IResult,digit,multispace, newline, Needed};
use nom::{AsChar, InputLength, InputIter, Compare, CompareResult, Slice, ErrorKind};

use num;
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use token::{Id, Tk, pprint_tokens, New, Tag};


pub struct Lexer;

impl Lexer {
    /// Convert a slice of bytes into a r
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
        space       |
        endline     |
        symbol      |
        operator    |
        number      |
        string      |
        identifier  |
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
    (Tk::new(Id::ErrorMarker, content, Tag::None))
));

named!(identifier <Tk>, do_parse!(
    name: call!(ident) >>
    (match is_keyword(name) {
            Some(tag) => Tk::new(Id::Name, name, tag),
            None => Tk::new(Id::Name, name, Tag::None)
        }
    )
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
    (Tk::new(Id::String, result.0, result.1))
));


named!(sublex_string <(&[u8], Tag)>, do_parse!(
    result: switch!(peek!(take!(1)),
        b"#" =>  call!(sublex_comment_string)       |
        b"'" =>  call!(sublex_plain_squote_string)  |
        b"\"" => call!(sublex_plain_dquote_string)  |
        b"r" =>  call!(sublex_rprefix_string)       |
        b"b" =>  call!(sublex_bprefix_string)       |
        b"f" =>  call!(sublex_fprefix_string)       ) >>
    (result)
));

named!(sublex_comment_string <(&[u8], Tag)>, do_parse!(
    result: preceded!(take!(1), is_not!("\n")) >>
    ((result, Tag::Comment))
));

named!(sublex_rprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::RawString))
));

named!(sublex_bprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::ByteString))
));

named!(sublex_fprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::FormatString))
));

named!(sublex_plain_squote_string <(&[u8], Tag)>, do_parse!(
    result: sublex_squote_string >>
    ((result, Tag::None))
));

named!(sublex_plain_dquote_string <(&[u8], Tag)>, do_parse!(
    result: sublex_dquote_string >>
    ((result, Tag::None))
));


named!(sublex_prefixed_string <&[u8]>,  do_parse!(
    result: preceded!(
        take!(1),
        call!(sublex_string)) >>
        (result.0)
));

named!(sublex_squote_string <&[u8]>, do_parse!(
    result: switch!(peek!(take!(3)),
        b"'''" => recognize!(delimited!(take!(3), take_until!("'''"), take!(3))) |
        _ => recognize!(delimited!(tag!("'"), take_until!("'"), tag!("'")))
    ) >>
    (result)
));


named!(sublex_dquote_string <&[u8]>, do_parse!(
    result: switch!(peek!(take!(3)),
        b"\"\"\"" => recognize!(delimited!(take!(3), take_until!("\"\"\""), take!(3))) |
        _ => recognize!(delimited!(tag!("\""), take_until!("\""), tag!("\"")))
    ) >>
    (result)
));


named!(symbol <Tk>, do_parse!(
    result: alt!(
        tag!("(")   => { |r: &'a[u8]| (&r[..], Tag::LeftParen)      } |
        tag!("[")   => { |r: &'a[u8]| (&r[..], Tag::LeftBracket)    } |
        tag!("{")   => { |r: &'a[u8]| (&r[..], Tag::LeftBrace)      } |
        tag!("}")   => { |r: &'a[u8]| (&r[..], Tag::RightBrace)     } |
        tag!("]")   => { |r: &'a[u8]| (&r[..], Tag::RightBracket)   } |
        tag!(")")   => { |r: &'a[u8]| (&r[..], Tag::RightParen)     } |
        tag!(",")   => { |r: &'a[u8]| (&r[..], Tag::Comma)          } |
        tag!(";")   => { |r: &'a[u8]| (&r[..], Tag::Semicolon)      } |
        tag!(":")   => { |r: &'a[u8]| (&r[..], Tag::Colon)          } |
        tag!("\\")  => { |r: &'a[u8]| (&r[..], Tag::Backslash)      }) >>
    (Tk::new(Id::Symbol, result.0, result.1))
));


named!(operator <Tk>, do_parse!(
    result : alt_complete!(
            // r: &'a [u8] are the bytes captured by the tag on the LHS.
            // that is mapped with the closure to a tuple of the slice of the
            // result mapped to its appropriate tag.
            tag!(r"<<=") => { |r: &'a[u8]| (&r[..], Tag::LeftShiftEqual)    } |
            tag!(r">>=") => { |r: &'a[u8]| (&r[..], Tag::RightShiftEqual)   } |
            tag!(r"**=") => { |r: &'a[u8]| (&r[..], Tag::DoubleStarEqual)   } |
            tag!(r"//=") => { |r: &'a[u8]| (&r[..], Tag::DoubleSlashEqual)  } |
            tag!(r"...") => { |r: &'a[u8]| (&r[..], Tag::Ellipsis)          } |
            tag!(r"==") =>  { |r: &'a[u8]| (&r[..], Tag::DoubleEqual)       } |
            tag!(r"!=") =>  { |r: &'a[u8]| (&r[..], Tag::NotEqual)          } |
            tag!(r"<>") =>  { |r: &'a[u8]| (&r[..], Tag::NotEqual)          } |
            tag!(r"<=") =>  { |r: &'a[u8]| (&r[..], Tag::LessOrEqual)       } |
            tag!(r"<<") =>  { |r: &'a[u8]| (&r[..], Tag::LeftShift)         } |
            tag!(r">=") =>  { |r: &'a[u8]| (&r[..], Tag::GreaterOrEqual)    } |
            tag!(r">>") =>  { |r: &'a[u8]| (&r[..], Tag::RightShift)        } |
            tag!(r"+=") =>  { |r: &'a[u8]| (&r[..], Tag::PlusEqual)         } |
            tag!(r"-=") =>  { |r: &'a[u8]| (&r[..], Tag::MinusEqual)        } |
            tag!(r"->") =>  { |r: &'a[u8]| (&r[..], Tag::RightArrow)        } |
            tag!(r"**") =>  { |r: &'a[u8]| (&r[..], Tag::DoubleStar)        } |
            tag!(r"*=") =>  { |r: &'a[u8]| (&r[..], Tag::StarEqual)         } |
            tag!(r"//") =>  { |r: &'a[u8]| (&r[..], Tag::DoubleSlash)       } |
            tag!(r"/=") =>  { |r: &'a[u8]| (&r[..], Tag::SlashEqual)        } |
            tag!(r"|=") =>  { |r: &'a[u8]| (&r[..], Tag::PipeEqual)         } |
            tag!(r"%=") =>  { |r: &'a[u8]| (&r[..], Tag::PercentEqual)      } |
            tag!(r"&=") =>  { |r: &'a[u8]| (&r[..], Tag::AmpEqual)          } |
            tag!(r"^=") =>  { |r: &'a[u8]| (&r[..], Tag::CaretEqual)        } |
            tag!(r"@=") =>  { |r: &'a[u8]| (&r[..], Tag::AtEqual)           } |
            tag!(r"+") =>   { |r: &'a[u8]| (&r[..], Tag::Plus)              } |
            tag!(r"-") =>   { |r: &'a[u8]| (&r[..], Tag::Minus)             } |
            tag!(r"*") =>   { |r: &'a[u8]| (&r[..], Tag::Star)              } |
            tag!(r"/") =>   { |r: &'a[u8]| (&r[..], Tag::Slash)             } |
            tag!(r"|") =>   { |r: &'a[u8]| (&r[..], Tag::Pipe)              } |
            tag!(r"&") =>   { |r: &'a[u8]| (&r[..], Tag::Amp)               } |
            tag!(r"<") =>   { |r: &'a[u8]| (&r[..], Tag::LeftAngle)         } |
            tag!(r">") =>   { |r: &'a[u8]| (&r[..], Tag::RightAngle)        } |
            tag!(r"=") =>   { |r: &'a[u8]| (&r[..], Tag::Equal)             } |
            tag!(r"%") =>   { |r: &'a[u8]| (&r[..], Tag::Percent)           } |
            tag!(r"^") =>   { |r: &'a[u8]| (&r[..], Tag::Caret)             } |
            tag!(r"~") =>   { |r: &'a[u8]| (&r[..], Tag::Tilde)             } |
            tag!(r"@") =>   { |r: &'a[u8]| (&r[..], Tag::At)                } |
            tag!(r".") =>   { |r: &'a[u8]| (&r[..], Tag::Dot)               }) >>
       (Tk::new(Id::Operator, result.0, result.1))
));

fn is_keyword(bytes: &[u8]) -> Option<Tag> {
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
        "yield"    => Some(Tag::Keyword),
        _ => None
    }
}



#[cfg(test)]
mod _api{
    use super::*;
    use serde_yaml;
    use serde_json;
    use serde_pickle;
    use bincode;

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

    fn assert_token(value: &(&[u8], Vec<Tk>), id: Id, tag: Tag) {
        pprint_tokens(&value.1);
        assert_eq!(value.1.len(), 1);
        let ref token = value.1[0];
        assert_eq!(token.id(), id);
        assert_eq!(token.tag(), tag);
    }

    #[test]
    fn tk_string() {

        // just "abc"
        let value = start_line(r#"  "abc"  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Double quoted strings containing single quotes are ok
        let value = start_line(r#"  "Dillon's String!"  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Single quoted strings containing double quotes are ok
        let value = start_line(r#"  'Thing"s and stuff'   "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Triple double quoted multiline
        let value = start_line(
r#"  """Line 0
Line 1
Line 2
Line 3
Line 4"""
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Triple double quoted multiline
        let value = start_line(
            r#"  '''alpha
beta
delta
gamma'''
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Quoted keywords should still be strings
        let value = start_line(r#"  'def'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Unicode
        let value = start_line(r#"  "שּׂθשּׂઊ" "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        let value = start_line(r#"  r'things'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::RawString);

        let value = start_line(r#"  b'\x94\x54'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::ByteString);

        let value = start_line(r#"  f'{number}'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::FormatString);

        let value = start_line(
            r#"  # Never compromise, even in the face of armageddon "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::Comment);
    }


    #[test]
    fn tk_name() {
        let value = start_line(r#" _hello "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Name, Tag::None);
    }

    #[test]
    fn tk_keyword() {
        let value = start_line(r#" def "#.trim().as_bytes()).unwrap();
    }

    #[test]
    fn expr_x_eq_1() {
        let value = start_line(r#"x = 1"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn module() {
        let input = r#"x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'
        x += 24354353
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

    # this is a comment
    def is_couch(self):
        return 'duh'

  "#.as_bytes();

        let value = start_line(input).unwrap();
        pprint_tokens(&value.1);

//        println!("{:?}", value.1);
        let json = serde_json::to_string_pretty(&value.1).unwrap();
//        println!("{}", unsafe {String::from_utf8_unchecked(serde_pickle::to_vec(&value.1, true).unwrap())});
        let i = bincode::serialize(&value.1, bincode::Infinite).unwrap();
        println!("bincode size: {}", i.len());
        println!("input size: {}", input.len());
        println!("json size: {}", json.len());
    }
}
