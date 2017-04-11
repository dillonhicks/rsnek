use nom::{IResult,digit,multispace,space, newline};
use itertools::Itertools;
use std::collections::VecDeque;

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
    pub encoding: Option<&'a [u8]>,
    pub shebang: Option<&'a [u8]>,
    pub lines: Vec<Line<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line<'a> {
    indent: usize,
    tokens: Vec<Token<'a>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a[u8]),
    Unknown(&'a[u8])
}

named!(parse_python <PythonSource>, do_parse!(
        opt!(multispace) >>
        encoding: opt!(encoding) >>
        shebang: opt!(shebang) >>
        tokens:  many0!(start_line) >>
        (PythonSource {
            encoding: encoding,
            shebang: shebang,
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
    tag!("\n") >>
    spaces: opt!(is_a!(" ")) >>
    tokens: many0!(line) >>
    (Line {
        indent: if spaces.is_some() {spaces.unwrap().len()} else {0},
        tokens: tokens.iter().flat_map(|x| x.iter()).map(|t| t.clone()).collect(),
        })
));

named!(line <VecDeque<Token>>, do_parse!(
    tokens: alt!(identifier | until_eol) >>
    (tokens)
));

named!(until_eol <VecDeque<Token>>, do_parse!(
    content: opt!(is_not!("\n")) >>
    (VecDeque::from([Token::Unknown(match content {Some(c) => c, None => &[]})].to_vec()))
));


named!(identifier <VecDeque<Token>>, do_parse!(
    ident: re_bytes_find_static!(r"[a-zA-Z_][[:word:]]*") >>
    rest: until_eol >>
    ({let mut b = VecDeque::from(rest); b.push_front(Token::Identifier(ident)); b})
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
        println!("{:?}", parse_python(r#"#!/usr/bin/env python
                          k
            potato
k
   k


dj;salkfj;asdjkfsd;alkfj;dl










































   x = 3
"#.as_bytes()));
    }
}
