use std;
use std::str;
use std::str::FromStr;
use std::rc::Rc;
use std::fmt::Debug;
use std::collections::VecDeque;
use std::ops::{Range,RangeFrom,RangeTo};

use nom::{IResult, hex_digit, oct_digit, digit, multispace, newline, Needed};
use nom::{AsChar, InputLength, InputIter, Compare, CompareResult, Slice, ErrorKind};

use num;
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use token::{Id, Tk, pprint_tokens, New, Tag, Str, Num, Dir, Op, Kw, Sym, Ws};
use keyword;


pub struct Lexer;

impl Lexer {
    /// Convert a slice of bytes into a r
    pub fn tokenize(bytes: &[u8]) -> Rc<IResult<&[u8], Vec<Tk>>> {
        let result = tokenize_bytes(bytes);
        Rc::new(result)
    }
}


named!(pub tokenize_bytes <Vec<Tk>>, do_parse!(
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
    tuple : alt_complete!(
            call!(sublex_hex)     => { |r: &'a[u8]| (&r[..], Tag::N(Num::Float))    } |
            call!(sublex_bin)     => { |r: &'a[u8]| (&r[..], Tag::N(Num::Float))    } |
            call!(sublex_octal)   => { |r: &'a[u8]| (&r[..], Tag::N(Num::Float))    } |
            call!(sublex_float)   => { |r: &'a[u8]| (&r[..], Tag::N(Num::Float))    } |
            call!(sublex_complex) => { |r: &'a[u8]| (&r[..], Tag::N(Num::Complex))  } |
            call!(digit)          => { |r: &'a[u8]| (&r[..], Tag::N(Num::Int))      } ) >>
    (Tk::new(Id::Number, tuple.0, tuple.1))
));


named!(sublex_hex <&[u8]>, recognize!(preceded!(tag!("0x"), hex_digit)));
named!(sublex_bin <&[u8]>, recognize!(preceded!(tag!("0b"), many1!(one_of!("01")))));
named!(sublex_octal <&[u8]>, recognize!(preceded!(tag!("0o"), hex_digit)));
named!(sublex_float <&[u8]>, recognize!(delimited!(digit, tag!("."), digit)));
named!(sublex_complex <&[u8]>, recognize!(preceded!(digit, tag!("j"))));

named!(endline <Tk>, do_parse!(
    nl: tag!("\n") >>
    (Tk::new(Id::Newline,  nl, Tag::W(Ws::Newline)))
));


named!(space <Tk>, do_parse!(
    tk: alt!(
        tag!(" ")  => { |r: &'a[u8]| (Tk::new(Id::Space, r, Tag::None)) } |
        tag!("\t") => { |r: &'a[u8]| (Tk::new(Id::Tab, r, Tag::None))   } ) >>
    (tk)
));


named!(error_marker <Tk>, do_parse!(
    content: take!(1) >>
    (Tk::new(Id::ErrorMarker, content, Tag::None))
));

named!(identifier <Tk>, do_parse!(
    name: call!(ident) >>
    (match as_keyword(name) {
            Some(tag) => tag,
            None => Tk::new(Id::Name, name, Tag::Ident)
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
    token: switch!(peek!(take!(1)),
        b"#" =>  call!(sublex_comment_string)       |
        b"'" =>  call!(sublex_string)               |
        b"\"" => call!(sublex_string)               |
        b"r" =>  call!(sublex_rprefix_string)       |
        b"b" =>  call!(sublex_bprefix_string)       |
        b"f" =>  call!(sublex_fprefix_string)       ) >>
    (token)
));


named!(sublex_comment_string <Tk>, do_parse!(
    bytes: preceded!(take!(1), is_not!("\n")) >>
    (Tk::new(Id::Comment, bytes, Tag::None))
));

named!(sublex_rprefix_string <Tk>, do_parse!(
    bytes: sublex_prefixed_string_u8 >>
    (Tk::new(Id::RawString, bytes, Tag::None))
));

named!(sublex_bprefix_string <Tk>, do_parse!(
    bytes: sublex_prefixed_string_u8 >>
    (Tk::new(Id::ByteString, bytes, Tag::None))
));

named!(sublex_fprefix_string <Tk>, do_parse!(
    bytes: sublex_prefixed_string_u8 >>
    (Tk::new(Id::FormatString, bytes, Tag::None))
));

named!(sublex_prefixed_string_u8 <&[u8]>,  do_parse!(
    result: preceded!(
        take!(1),
        call!(sublex_string_u8)) >>
        (result)
));


named!(sublex_squote_string_u8 <&[u8]>, do_parse!(
    bytes: switch!(peek!(take!(3)),
        b"'''" => recognize!(delimited!(take!(3), take_until!("'''"), take!(3))) |
        _ => recognize!(delimited!(tag!("'"), take_until!("'"), tag!("'")))
    ) >>
    (bytes)
));


named!(sublex_dquote_string_u8 <&[u8]>, do_parse!(
    bytes: switch!(peek!(take!(3)),
        b"\"\"\"" => recognize!(delimited!(take!(3), take_until!("\"\"\""), take!(3))) |
        _ => recognize!(delimited!(tag!("\""), take_until!("\""), tag!("\"")))
    ) >>
    (bytes)
));

named!(sublex_string_u8 <&[u8]>, do_parse!(
    bytes: switch!(peek!(take!(1)),
        b"'" =>  call!(sublex_squote_string_u8)  |
        b"\"" => call!(sublex_dquote_string_u8)  ) >>
    (bytes)
));

named!(sublex_string <Tk>, do_parse!(
    bytes: sublex_string_u8 >>
    (Tk::new(Id::String, bytes, Tag::None))
));

named!(symbol <Tk>, do_parse!(
    token: alt!(
        tag!("(")   => { |r: &'a[u8]| (Tk::new(Id::LeftParen, r, Tag::None))       } |
        tag!("[")   => { |r: &'a[u8]| (Tk::new(Id::LeftBracket, r, Tag::None))     } |
        tag!("{")   => { |r: &'a[u8]| (Tk::new(Id::LeftBrace, r, Tag::None))       } |
        tag!("}")   => { |r: &'a[u8]| (Tk::new(Id::RightBrace, r, Tag::None))      } |
        tag!("]")   => { |r: &'a[u8]| (Tk::new(Id::RightBracket, r, Tag::None))    } |
        tag!(")")   => { |r: &'a[u8]| (Tk::new(Id::RightParen, r, Tag::None))      } |
        tag!(",")   => { |r: &'a[u8]| (Tk::new(Id::Comma, r, Tag::None))           } |
        tag!(";")   => { |r: &'a[u8]| (Tk::new(Id::Semicolon, r, Tag::None))       } |
        tag!(":")   => { |r: &'a[u8]| (Tk::new(Id::Colon, r, Tag::None))           } |
        tag!("\\")  => { |r: &'a[u8]| (Tk::new(Id::Backslash, r, Tag::None))       }) >>
    (token)
));


named!(operator <Tk>, do_parse!(
    token : alt_complete!(
            // r: &'a [u8] are the bytes captured by the tag on the LHS.
            // that is mapped with the closure to a tuple of the slice of the
            // result mapped to its appropriate tag.
            tag!(r"<<=") => { |r: &'a[u8]| (Tk::new(Id::LeftShiftEqual, r, Tag::None))    } |
            tag!(r">>=") => { |r: &'a[u8]| (Tk::new(Id::RightShiftEqual, r, Tag::None))   } |
            tag!(r"**=") => { |r: &'a[u8]| (Tk::new(Id::DoubleStarEqual, r, Tag::None))   } |
            tag!(r"//=") => { |r: &'a[u8]| (Tk::new(Id::DoubleSlashEqual, r, Tag::None))  } |
            tag!(r"...") => { |r: &'a[u8]| (Tk::new(Id::Ellipsis, r, Tag::None))          } |
            tag!(r"==") =>  { |r: &'a[u8]| (Tk::new(Id::DoubleEqual, r, Tag::None))       } |
            tag!(r"!=") =>  { |r: &'a[u8]| (Tk::new(Id::NotEqual, r, Tag::None))          } |
            tag!(r"<>") =>  { |r: &'a[u8]| (Tk::new(Id::NotEqual, r, Tag::None))          } |
            tag!(r"<=") =>  { |r: &'a[u8]| (Tk::new(Id::LessOrEqual, r, Tag::None))       } |
            tag!(r"<<") =>  { |r: &'a[u8]| (Tk::new(Id::LeftShift, r, Tag::None))         } |
            tag!(r">=") =>  { |r: &'a[u8]| (Tk::new(Id::GreaterOrEqual, r, Tag::None))    } |
            tag!(r">>") =>  { |r: &'a[u8]| (Tk::new(Id::RightShift, r, Tag::None))        } |
            tag!(r"+=") =>  { |r: &'a[u8]| (Tk::new(Id::PlusEqual, r, Tag::None))         } |
            tag!(r"-=") =>  { |r: &'a[u8]| (Tk::new(Id::MinusEqual, r, Tag::None))        } |
            tag!(r"->") =>  { |r: &'a[u8]| (Tk::new(Id::RightArrow, r, Tag::None))        } |
            tag!(r"**") =>  { |r: &'a[u8]| (Tk::new(Id::DoubleStar, r, Tag::None))        } |
            tag!(r"*=") =>  { |r: &'a[u8]| (Tk::new(Id::StarEqual, r, Tag::None))         } |
            tag!(r"//") =>  { |r: &'a[u8]| (Tk::new(Id::DoubleSlash, r, Tag::None))       } |
            tag!(r"/=") =>  { |r: &'a[u8]| (Tk::new(Id::SlashEqual, r, Tag::None))        } |
            tag!(r"|=") =>  { |r: &'a[u8]| (Tk::new(Id::PipeEqual, r, Tag::None))         } |
            tag!(r"%=") =>  { |r: &'a[u8]| (Tk::new(Id::PercentEqual, r, Tag::None))      } |
            tag!(r"&=") =>  { |r: &'a[u8]| (Tk::new(Id::AmpEqual, r, Tag::None))          } |
            tag!(r"^=") =>  { |r: &'a[u8]| (Tk::new(Id::CaretEqual, r, Tag::None))        } |
            tag!(r"@=") =>  { |r: &'a[u8]| (Tk::new(Id::AtEqual, r, Tag::None))           } |
            tag!(r"+") =>   { |r: &'a[u8]| (Tk::new(Id::Plus, r, Tag::None))              } |
            tag!(r"-") =>   { |r: &'a[u8]| (Tk::new(Id::Minus, r, Tag::None))             } |
            tag!(r"*") =>   { |r: &'a[u8]| (Tk::new(Id::Star, r, Tag::None))              } |
            tag!(r"/") =>   { |r: &'a[u8]| (Tk::new(Id::Slash, r, Tag::None))             } |
            tag!(r"|") =>   { |r: &'a[u8]| (Tk::new(Id::Pipe, r, Tag::None))              } |
            tag!(r"&") =>   { |r: &'a[u8]| (Tk::new(Id::Amp, r, Tag::None))               } |
            tag!(r"<") =>   { |r: &'a[u8]| (Tk::new(Id::LeftAngle, r, Tag::None))         } |
            tag!(r">") =>   { |r: &'a[u8]| (Tk::new(Id::RightAngle, r, Tag::None))        } |
            tag!(r"=") =>   { |r: &'a[u8]| (Tk::new(Id::Equal, r, Tag::None))             } |
            tag!(r"%") =>   { |r: &'a[u8]| (Tk::new(Id::Percent, r, Tag::None))           } |
            tag!(r"^") =>   { |r: &'a[u8]| (Tk::new(Id::Caret, r, Tag::None))             } |
            tag!(r"~") =>   { |r: &'a[u8]| (Tk::new(Id::Tilde, r, Tag::None))             } |
            tag!(r"@") =>   { |r: &'a[u8]| (Tk::new(Id::At, r, Tag::None))                } |
            tag!(r".") =>   { |r: &'a[u8]| (Tk::new(Id::Dot, r, Tag::None))               }) >>
       (token)
));


fn as_keyword(bytes: &[u8]) -> Option<Tk> {
    let string = match str::from_utf8(bytes) {
        Ok(string) => string,
        err => return None
    };

    match string {
        "False"     => Some(Tk::new(Id::False, bytes, Tag::None)),
        "None"      => Some(Tk::new(Id::None, bytes, Tag::None)),
        "True"      => Some(Tk::new(Id::True, bytes, Tag::None)),
        "and"       => Some(Tk::new(Id::And, bytes, Tag::None)),
        "as"        => Some(Tk::new(Id::As, bytes, Tag::None)),
        "assert"    => Some(Tk::new(Id::Assert, bytes, Tag::None)),
        "break"     => Some(Tk::new(Id::Break, bytes, Tag::None)),
        "class"     => Some(Tk::new(Id::Class, bytes, Tag::None)),
        "continue"  => Some(Tk::new(Id::Continue, bytes, Tag::None)),
        "def"       => Some(Tk::new(Id::Def, bytes, Tag::None)),
        "del"       => Some(Tk::new(Id::Del, bytes, Tag::None)),
        "elif"      => Some(Tk::new(Id::Elif, bytes, Tag::None)),
        "else"      => Some(Tk::new(Id::Else, bytes, Tag::None)),
        "except"    => Some(Tk::new(Id::Except, bytes, Tag::None)),
        "finally"   => Some(Tk::new(Id::Finally, bytes, Tag::None)),
        "for"       => Some(Tk::new(Id::For, bytes, Tag::None)),
        "from"      => Some(Tk::new(Id::From, bytes, Tag::None)),
        "global"    => Some(Tk::new(Id::Global, bytes, Tag::None)),
        "if"        => Some(Tk::new(Id::If, bytes, Tag::None)),
        "import"    => Some(Tk::new(Id::Import, bytes, Tag::None)),
        "in"        => Some(Tk::new(Id::In, bytes, Tag::None)),
        "is"        => Some(Tk::new(Id::Is, bytes, Tag::None)),
        "lambda"    => Some(Tk::new(Id::Lambda, bytes, Tag::None)),
        "nonlocal"  => Some(Tk::new(Id::Nonlocal, bytes, Tag::None)),
        "not"       => Some(Tk::new(Id::Not, bytes, Tag::None)),
        "or"        => Some(Tk::new(Id::Or, bytes, Tag::None)),
        "pass"      => Some(Tk::new(Id::Pass, bytes, Tag::None)),
        "raise"     => Some(Tk::new(Id::Raise, bytes, Tag::None)),
        "return"    => Some(Tk::new(Id::Return, bytes, Tag::None)),
        "try"       => Some(Tk::new(Id::Try, bytes, Tag::None)),
        "while"     => Some(Tk::new(Id::While, bytes, Tag::None)),
        "with"      => Some(Tk::new(Id::With, bytes, Tag::None)),
        "yield"     => Some(Tk::new(Id::Yield, bytes, Tag::None)),
        _           => None
    }
}



#[cfg(test)]
mod _api{
    use super::*;
    use serde_yaml;
    use serde_json;
    use serde_pickle;
    use bincode;

    fn assert_token(value: &(&[u8], Vec<Tk>), id: Id, tag: Tag) {
        pprint_tokens(&value.1);
        assert_eq!(value.1.len(), 1);
        let ref token = value.1[0];
        assert_eq!(token.id(), id);
        assert_eq!(token.tag(), tag);
    }

    #[test]
    fn tk_space() {
        let value = tokenize_bytes(r#" "#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_number() {
        let value = tokenize_bytes(r#"12345"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Number, Tag::N(Num::Int));

        let value = tokenize_bytes(r#"12.34"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Number, Tag::N(Num::Float));

        let value = tokenize_bytes(r#"0x2345"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Number, Tag::N(Num::Hex));

        let value = tokenize_bytes(r#"0o2345"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Number, Tag::N(Num::Octal));

        let value = tokenize_bytes(r#"0b0101"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Number, Tag::N(Num::Binary));
    }

    #[test]
    fn tk_symbol() {
        let value = tokenize_bytes(r#":"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_operator() {
        let value = tokenize_bytes(r#"+="#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn tk_string() {

        // just "abc"
        let value = tokenize_bytes(r#"  "abc"  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Double quoted strings containing single quotes are ok
        let value = tokenize_bytes(r#"  "Dillon's String!"  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Single quoted strings containing double quotes are ok
        let value = tokenize_bytes(r#"  'Thing"s and stuff'   "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Triple double quoted multiline
        let value = tokenize_bytes(
r#"  """Line 0
Line 1
Line 2
Line 3
Line 4"""
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Triple double quoted multiline
        let value = tokenize_bytes(
            r#"  '''alpha
beta
delta
gamma'''
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Quoted keywords should still be strings
        let value = tokenize_bytes(r#"  'def'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        // Unicode
        let value = tokenize_bytes(r#"  "שּׂθשּׂઊ" "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::None);

        let value = tokenize_bytes(r#"  r'things'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::RawString, Tag::None);

        let value = tokenize_bytes(r#"  b'\x94\x54'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::ByteString, Tag::None);

        let value = tokenize_bytes(r#"  f'{number}'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::FormatString, Tag::None);

        let value = tokenize_bytes(
            r#"  # Never compromise, even in the face of armageddon "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Comment, Tag::None);
    }


    #[test]
    fn tk_name() {
        let value = tokenize_bytes(r#" _hello "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::Name, Tag::Ident);
    }

    #[test]
    fn tk_keyword() {
        let value = tokenize_bytes(r#" def "#.trim().as_bytes()).unwrap();
    }

    #[test]
    fn expr_x_eq_1() {
        let value = tokenize_bytes(r#"x = 1"#.as_bytes()).unwrap();
        pprint_tokens(&value.1);
    }

    #[test]
    fn module() {
        let input = [r#"x += 24354353
  y = 3
  Q -> c
  x = [1, 2, 3, 4, 5];
  \t
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
  "#.as_bytes(), "\t".as_bytes()].join(&(' ' as u8)).into_boxed_slice();

        let value = tokenize_bytes(&(*input)).unwrap();
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

