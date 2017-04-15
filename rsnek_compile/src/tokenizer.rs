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
    digits : switch!(peek!(take!(2)),
        b"0x" => call!(sublex_hex)            |
        b"0b" => call!(sublex_bin)            |
        b"0o" => call!(sublex_octal)          |
        _     => alt_complete!(
                    call!(sublex_float)   => { |r: &'a[u8]| (&r[..], Tag::N(Num::Float))    } |
                    call!(sublex_complex) => { |r: &'a[u8]| (&r[..], Tag::N(Num::Complex))  } |
                    call!(digit)          => { |r: &'a[u8]| (&r[..], Tag::N(Num::Int))      } )
                                              ) >>
    (Tk::new(Id::Number, digits.0, digits.1))
));


named!(sublex_hex <(&[u8], Tag)>, do_parse!(
     num: recognize!(preceded!(tag!("0x"), hex_digit)) >>
    ((num, Tag::N(Num::Hex)))
));


named!(sublex_bin <(&[u8], Tag)>, do_parse!(
     num:  recognize!(preceded!(tag!("0b"), many1!(one_of!("01")))) >>
    ((num, Tag::N(Num::Binary)))
));


named!(sublex_octal <(&[u8], Tag)>, do_parse!(
     num: recognize!(preceded!(tag!("0o"), hex_digit)) >>
    ((num, Tag::N(Num::Octal)))
));


named!(sublex_float <&[u8]>, do_parse!(
     num: recognize!(delimited!(digit, tag!("."), digit)) >>
    (num)
));


named!(sublex_complex <&[u8]>, do_parse!(
     num: recognize!(preceded!(digit, tag!("j"))) >>
    (num)
));


named!(endline <Tk>, do_parse!(
    nl: tag!("\n") >>
    (Tk::new(Id::Newline,  nl, Tag::W(Ws::Newline)))
));


named!(space <Tk>, do_parse!(
    sp: tag!(" ") >>
    (Tk::new(Id::Space, sp, Tag::W(Ws::Space)))
));


named!(error_marker <Tk>, do_parse!(
    content: take!(1) >>
    (Tk::new(Id::ErrorMarker, content, Tag::None))
));

named!(identifier <Tk>, do_parse!(
    name: call!(ident) >>
    (match is_keyword(name) {
            Some(tag) => Tk::new(Id::Name, name, tag),
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
    ((result, Tag::S(Str::Comment)))
));

named!(sublex_rprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::S(Str::Raw)))
));

named!(sublex_bprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::S(Str::Bytes)))
));

named!(sublex_fprefix_string <(&[u8], Tag)>, do_parse!(
    result: sublex_prefixed_string >>
    ((result, Tag::S(Str::Format)))
));

named!(sublex_plain_squote_string <(&[u8], Tag)>, do_parse!(
    result: sublex_squote_string >>
    ((result, Tag::S(Str::Unicode)))
));

named!(sublex_plain_dquote_string <(&[u8], Tag)>, do_parse!(
    result: sublex_dquote_string >>
    ((result, Tag::S(Str::Unicode)))
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
        tag!("(")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::LeftParen))       } |
        tag!("[")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::LeftBracket))     } |
        tag!("{")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::LeftBrace))       } |
        tag!("}")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::RightBrace))      } |
        tag!("]")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::RightBracket))    } |
        tag!(")")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::RightParen))      } |
        tag!(",")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::Comma))           } |
        tag!(";")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::Semicolon))       } |
        tag!(":")   => { |r: &'a[u8]| (&r[..], Tag::M(Sym::Colon))           } |
        tag!("\\")  => { |r: &'a[u8]| (&r[..], Tag::M(Sym::Backslash))       }) >>
    (Tk::new(Id::Symbol, result.0, result.1))
));


named!(operator <Tk>, do_parse!(
    result : alt_complete!(
            // r: &'a [u8] are the bytes captured by the tag on the LHS.
            // that is mapped with the closure to a tuple of the slice of the
            // result mapped to its appropriate tag.
            tag!(r"<<=") => { |r: &'a[u8]| (&r[..], Tag::O(Op::LeftShiftEqual))    } |
            tag!(r">>=") => { |r: &'a[u8]| (&r[..], Tag::O(Op::RightShiftEqual))   } |
            tag!(r"**=") => { |r: &'a[u8]| (&r[..], Tag::O(Op::DoubleStarEqual))   } |
            tag!(r"//=") => { |r: &'a[u8]| (&r[..], Tag::O(Op::DoubleSlashEqual))  } |
            tag!(r"...") => { |r: &'a[u8]| (&r[..], Tag::O(Op::Ellipsis))          } |
            tag!(r"==") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::DoubleEqual))       } |
            tag!(r"!=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::NotEqual))          } |
            tag!(r"<>") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::NotEqual))          } |
            tag!(r"<=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::LessOrEqual))       } |
            tag!(r"<<") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::LeftShift))         } |
            tag!(r">=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::GreaterOrEqual))    } |
            tag!(r">>") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::RightShift))        } |
            tag!(r"+=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::PlusEqual))         } |
            tag!(r"-=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::MinusEqual))        } |
            tag!(r"->") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::RightArrow))        } |
            tag!(r"**") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::DoubleStar))        } |
            tag!(r"*=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::StarEqual))         } |
            tag!(r"//") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::DoubleSlash))       } |
            tag!(r"/=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::SlashEqual))        } |
            tag!(r"|=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::PipeEqual))         } |
            tag!(r"%=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::PercentEqual))      } |
            tag!(r"&=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::AmpEqual))          } |
            tag!(r"^=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::CaretEqual))        } |
            tag!(r"@=") =>  { |r: &'a[u8]| (&r[..], Tag::O(Op::AtEqual))           } |
            tag!(r"+") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Plus))              } |
            tag!(r"-") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Minus))             } |
            tag!(r"*") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Star))              } |
            tag!(r"/") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Slash))             } |
            tag!(r"|") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Pipe))              } |
            tag!(r"&") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Amp))               } |
            tag!(r"<") =>   { |r: &'a[u8]| (&r[..], Tag::M(Sym::LeftAngle))        } |
            tag!(r">") =>   { |r: &'a[u8]| (&r[..], Tag::M(Sym::RightAngle))        } |
            tag!(r"=") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Equal))             } |
            tag!(r"%") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Percent))           } |
            tag!(r"^") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Caret))             } |
            tag!(r"~") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Tilde))             } |
            tag!(r"@") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::At))                } |
            tag!(r".") =>   { |r: &'a[u8]| (&r[..], Tag::O(Op::Dot))               }) >>
       (Tk::new(Id::Operator, result.0, result.1))
));


fn is_keyword(bytes: &[u8]) -> Option<Tag> {
    let string = match str::from_utf8(bytes) {
        Ok(string) => string,
        err => return None
    };
    
    match string {
        "False"     => Some(Tag::K(Kw::False)),
        "None"      => Some(Tag::K(Kw::None)),
        "True"      => Some(Tag::K(Kw::True)),
        "and"       => Some(Tag::K(Kw::And)),
        "as"        => Some(Tag::K(Kw::As)),
        "assert"    => Some(Tag::K(Kw::Assert)),
        "break"     => Some(Tag::K(Kw::Break)),
        "class"     => Some(Tag::K(Kw::Class)),
        "continue"  => Some(Tag::K(Kw::Continue)),
        "def"       => Some(Tag::K(Kw::Def)),
        "del"       => Some(Tag::K(Kw::Del)),
        "elif"      => Some(Tag::K(Kw::Elif)),
        "else"      => Some(Tag::K(Kw::Else)),
        "except"    => Some(Tag::K(Kw::Except)),
        "finally"   => Some(Tag::K(Kw::Finally)),
        "for"       => Some(Tag::K(Kw::For)),
        "from"      => Some(Tag::K(Kw::From)),
        "global"    => Some(Tag::K(Kw::Global)),
        "if"        => Some(Tag::K(Kw::If)),
        "import"    => Some(Tag::K(Kw::Import)),
        "in"        => Some(Tag::K(Kw::In)),
        "is"        => Some(Tag::K(Kw::Is)),
        "lambda"    => Some(Tag::K(Kw::Lambda)),
        "nonlocal"  => Some(Tag::K(Kw::Nonlocal)),
        "not"       => Some(Tag::K(Kw::Not)),
        "or"        => Some(Tag::K(Kw::Or)),
        "pass"      => Some(Tag::K(Kw::Pass)),
        "raise"     => Some(Tag::K(Kw::Raise)),
        "return"    => Some(Tag::K(Kw::Return)),
        "try"       => Some(Tag::K(Kw::Try)),
        "while"     => Some(Tag::K(Kw::While)),
        "with"      => Some(Tag::K(Kw::With)),
        "yield"     => Some(Tag::K(Kw::Yield)),
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
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Double quoted strings containing single quotes are ok
        let value = tokenize_bytes(r#"  "Dillon's String!"  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Single quoted strings containing double quotes are ok
        let value = tokenize_bytes(r#"  'Thing"s and stuff'   "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Triple double quoted multiline
        let value = tokenize_bytes(
r#"  """Line 0
Line 1
Line 2
Line 3
Line 4"""
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Triple double quoted multiline
        let value = tokenize_bytes(
            r#"  '''alpha
beta
delta
gamma'''
"#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Quoted keywords should still be strings
        let value = tokenize_bytes(r#"  'def'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        // Unicode
        let value = tokenize_bytes(r#"  "שּׂθשּׂઊ" "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Unicode));

        let value = tokenize_bytes(r#"  r'things'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Raw));

        let value = tokenize_bytes(r#"  b'\x94\x54'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Bytes));

        let value = tokenize_bytes(r#"  f'{number}'  "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Format));

        let value = tokenize_bytes(
            r#"  # Never compromise, even in the face of armageddon "#.trim().as_bytes()).unwrap();
        assert_token(&value, Id::String, Tag::S(Str::Comment));
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

  "#.as_bytes();

        let value = tokenize_bytes(input).unwrap();
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

