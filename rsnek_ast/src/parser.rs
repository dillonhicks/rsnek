use nom::{IResult,digit,multispace, newline};
use itertools::Itertools;
use std::collections::VecDeque;
use token::Id;
// Parser definition

use std::str;
use std::str::FromStr;

//struct Parser;
use std;
//
//impl Parser {
//    fn new() -> Self {
//        Parser {}
//    }
//
//    fn parse(&self, text: &'static [u8]) -> IResult<&[u8], i64>{
//        expr(text)
//    }
//}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PythonSource<'a> {
//    pub encoding: Option<&'a [u8]>,
//    pub shebang: Option<&'a [u8]>,
    pub lines: Vec<Line<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line<'a> {
    tokens: Vec<Token<'a>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a[u8]),
    Unknown(&'a[u8]),
    WhiteSpace(&'a [u8]),
    NewLine(&'a [u8]),
    Number(&'a [u8]),
    Operator(&'a [u8]),
//    Other(Id, &'a [u8]),
}

//impl<'a> Token<'a> {
//    pub fn new(id: Id, bytes: &'a [u8]) -> Self{
//        Token::Other(id, bytes)
//    }
//}

named!(parse_python <PythonSource>, do_parse!(
//        opt!(multispace) >>
//        encoding: opt!(encoding) >>
//        shebang: opt!(shebang) >>
        tokens:  many0!(start_line) >>
        (PythonSource {
//            encoding: encoding,
//            shebang: shebang,
            lines: tokens,
        })
 ));


named!(encoding <&[u8]>, do_parse! (
        tag!("#") >>
        not!(char!('!')) >>
        content: take_until!("\n") >>
        (content)
));

named!(shebang <&[u8]>, do_parse!(
        tag!("#!") >>
        content: take_until!("\n") >>
        (content)

));


named!(start_line <Line>, do_parse!(
    tokens: many0!(line) >>
    (Line {
        tokens: tokens,
        })
));

named!(line <Token>, do_parse!(
    token: alt_complete!(space | endline | operator | number | identifier | unknown) >>
    (token)
));

named!(number <Token>, do_parse!(
    nl: digit >>
    (Token::Number(nl))
));

named!(endline <Token>, do_parse!(
    nl: tag!("\n") >>
    (Token::NewLine(nl))
));

named!(space <Token>, do_parse!(
    sp: is_a!(" ") >>
    (Token::WhiteSpace(sp))
));

named!(unknown <Token>, do_parse!(
    content: is_not!("\n") >>
    (Token::Unknown(content))
));

named!(identifier <Token>, do_parse!(
    ident: re_bytes_find_static!(r"[a-zA-Z_]([[:word:]]*)") >>
    (Token::Identifier(ident))
));

named!(operator <Token>, do_parse!(
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
            tag!(r"@")) >>
       (Token::Operator(op))
));


// We parse any expr surrounded by parens, ignoring all whitespaces around those
//named!(parens<i64>, ws!(delimited!( tag!("("), expr, tag!(")") )) );

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
//named!(factor<i64>, alt!(
//map_res!(
//map_res!(
//ws!(digit),
//str::from_utf8
//),
//FromStr::from_str
//)
//| parens
//)
//);
//
//// We read an initial factor and for each time we find
//// a * or / operator followed by another factor, we do
//// the math by folding everything
//named!(term <i64>, do_parse!(
//init: factor >>
//res:  fold_many0!(
//pair!(alt!(
//}tag!("*") | tag!("/")), factor),
//        init,
//        |acc, (op, val): (&[u8], i64)| {
//            if (op[0] as char) == '*' { acc * val } else { acc / val }
//        }
//    ) >>
//    (res)
//  )
//);
//
//named!(expr <i64>, do_parse!(
//    init: term >>
//    res:  fold_many0!(
//        pair!(alt!(tag!("+") | tag!("-")), term),
//        init,
//        |acc, (op, val): (&[u8], i64)| {
//            if (op[0] as char) == '+' { acc + val } else { acc - val }
//        }
//    ) >>
//    (res)
//  )
//);


#[cfg(test)]
mod _api{
    use super::*;
//
//    #[test]
//    fn examples() {
//        let parser = Parser::new();
//
//        println!("NOM: {:?}", parser.parse(b"3 + 12 + 888"));
//        assert_eq!(parser.parse(b"1+2"),         IResult::Done(&b""[..], 3));
//        assert_eq!(parser.parse(b"12+6-4+3"),    IResult::Done(&b""[..], 17));
//        assert_eq!(parser.parse(b"1+2*3+4"),     IResult::Done(&b""[..], 11));
//
//        assert_eq!(parser.parse(b"(2)"),         IResult::Done(&b""[..], 2));
//        assert_eq!(parser.parse(b"2*(3+4)"),     IResult::Done(&b""[..], 14));
//        assert_eq!(parser.parse(b"2*2/(5-1)+3"), IResult::Done(&b""[..], 4));
//    }

    #[test]
    fn module() {
        println!("{:?}", start_line(
r#"x += 24354353
  y = 3
  Q -> c"#.as_bytes()));
    }
}
