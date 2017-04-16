use std::collections::VecDeque;
use std::str;
use std::str::FromStr;
use std;

use nom::{IResult,digit,multispace, newline};
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use num;
use num::FromPrimitive;


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Tk<'a> {
    id: Id,

    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    tag: Tag
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Error {
    Unrecognized
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Tag {
    Keyword,
    None,
    Ident,

    // Number
    N(Num),
    S(Str),
    O(Op),
    K(Kw),
    W(Ws),
    M(Sym),
    E(Error),

    // Symbols
    //  Note: The (Left|Right) angle brackets are used as LESS and GREATER
    //  operators as well.

    // ( [ { <
    Paren(Dir),
    Bracket(Dir),
    Brace(Dir),
    Angle(Dir),
    Arrow(Dir),


    Colon,
    Comma,
    Semicolon,
    Backslash,


    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Amp,
    Tilde,
    At,
    Dot,
    Percent,
    Caret,
    Equal,
    LeftShiftEqual,
    RightShiftEqual,
    DoubleSlashEqual,
    DoubleStarEqual,
    Ellipsis,
    DoubleEqual,
    NotEqual,
    LessOrEqual,
    LeftShift,
    GreaterOrEqual,
    RightShift,
    PipeEqual,
    PercentEqual,
    AmpEqual,
    DoubleSlash,
    PlusEqual,
    MinusEqual,
    RightArrow,

    DoubleStar,
    StarEqual,
    SlashEqual,
    CaretEqual,
    AtEqual,

//    SEMI,
//    PLUS,
//    MINUS,
//    STAR,
//    SLASH,
//    VBAR,
//    AMPER,
//    LESS,
//    GREATER,
//    EQUAL,
//    DOT,
//    PERCENT,
//    LBRACE,
//    RBRACE,
//    EQEQUAL,
//    NOTEQUAL,
//    LESSEQUAL,
//    GREATEREQUAL,
//    TILDE,
//    CIRCUMFLEX,
//    LEFTSHIFT,
//    RIGHTSHIFT,
//    DOUBLESTAR,
//    PLUSEQUAL,
//    MINEQUAL,
//    STAREQUAL,
//    SLASHEQUAL,
//    PERCENTEQUAL,
//    AMPEREQUAL,
//    VBAREQUAL,
//    CIRCUMFLEXEQUAL,
//    LEFTSHIFTEQUAL,
//    RIGHTSHIFTEQUAL,
//    DOUBLESTAREQUAL,
//    DOUBLESLASH,
//    DOUBLESLASHEQUAL,
//    AT,
//    ATEQUAL,
//    RARROW,
//    ELLIPSIS,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Dir {
    L,
    R
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Sym {
    // -> =>
    Arrow,
    BigArrow,

    // ) ] } >
    RightParen,
    RightBracket,
    RightBrace,
    RightAngle,

    // ( [ { <
    LeftArrow,
    LeftAngle,
    LeftParen,
    LeftBracket,
    LeftBrace,

    // : , ; \
    Colon,
    Comma,
    Semicolon,
    Backslash,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Ws {
    // \n
    Newline,
    // ' '
    Space,
    // \t
    Tab,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Kw {
    False,
    None,
    True,
    And,
    As,
    Assert,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Amp,
    Tilde,
    At,
    Dot,
    Percent,
    Caret,
    Equal,
    LeftShiftEqual,
    RightShiftEqual,
    DoubleSlashEqual,
    DoubleStarEqual,
    Ellipsis,
    DoubleEqual,
    NotEqual,
    LessOrEqual,
    LeftShift,
    GreaterOrEqual,
    RightShift,
    PipeEqual,
    PercentEqual,
    AmpEqual,
    DoubleSlash,
    PlusEqual,
    MinusEqual,
    RightArrow,
    DoubleStar,
    StarEqual,
    SlashEqual,
    CaretEqual,
    AtEqual,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Num {
    Int,
    Hex,
    Binary,
    Octal,
    Float,
    Complex,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Str {
    Ascii,
    Unicode,
    Bytes,
    Comment,
    Raw,
    Format
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Id {
    Name,
    Number,
    Newline,
    Space,
    Tab,
    Operator,
    Symbol,
    ErrorMarker,
    Keyword,

    // Strings
    Comment,
    String,
    RawString,
    ByteString,
    FormatString,


    // Numbers
    Int,
    Hex,
    Binary,
    Octal,
    Float,
    Complex,

    // Keywords
    False,
    True,
    None,
    And,
    As,
    Assert,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,

    // Symbols
    //  Note: The (Left|Right) angle brackets are used as LESS and GREATER
    //  operators as well.

    // ( [ { <
    LeftParen,
    LeftBracket,
    LeftBrace,
    LeftAngle,

    // ) ] } >
    RightParen,
    RightBracket,
    RightBrace,
    RightAngle,

    Colon,
    Comma,
    Semicolon,
    Backslash,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Amp,
    Tilde,
    At,
    Dot,
    Percent,
    Caret,
    Equal,
    LeftShiftEqual,
    RightShiftEqual,
    DoubleSlashEqual,
    DoubleStarEqual,
    Ellipsis,
    DoubleEqual,
    NotEqual,
    LessOrEqual,
    LeftShift,
    GreaterOrEqual,
    RightShift,
    PipeEqual,
    PercentEqual,
    AmpEqual,
    DoubleSlash,
    PlusEqual,
    MinusEqual,
    RightArrow,
    DoubleStar,
    StarEqual,
    SlashEqual,
    CaretEqual,
    AtEqual,

}


pub trait New<'a> {
    fn new(id: Id, bytes: &'a [u8], tag: Tag) -> Self;
}



impl<'a> New<'a> for Tk<'a> {
    fn new(id: Id, bytes: &'a [u8], tag: Tag) -> Self {
        Tk {
            id: id,
            bytes: bytes,
            tag: tag
        }
    }
}

impl<'a> Tk<'a> {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn tag(&self) -> Tag { self.tag }

    pub fn Identifier(bytes: &'a[u8]) -> Self { New::new( Id::Name,  bytes, Tag::None)}
    pub fn Space(bytes: &'a [u8]) -> Self { New::new( Id::Space,  bytes, Tag::None)}
    pub fn NewLine(bytes: &'a [u8]) -> Self { New::new( Id::Newline,  bytes, Tag::None)}
    pub fn Number(bytes: &'a [u8]) -> Self { New::new( Id::Number,  bytes, Tag::None)}
    pub fn Operator(bytes: &'a [u8]) -> Self { New::new( Id::Operator,  bytes, Tag::None)}
    pub fn Symbol(bytes: &'a [u8]) -> Self { New::new( Id::Symbol,  bytes, Tag::None)}

}

pub fn pprint_tokens(tokens: &Vec<Tk>) {
    for (idx, t) in tokens.iter().enumerate() {
        match format_token(&t) {
            Some(token) => println!("{:>3}: {}", idx, token),
            None => continue
        }
    }
}

pub fn format_token(t: &Tk) -> Option<String> {

    let formatted = match t.id() {
        Id::Space => return None,
        Id::Tab |
        Id::Newline => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), format!("{:?}", String::from_utf8_lossy(t.bytes())), format!("{:?}", t.tag())),
        _ => format!("{:>15} {:^20}{:>10}", format!("{:?}", t.id()), String::from_utf8_lossy(t.bytes()), format!("{:?}", t.tag()))
    };

    Some(formatted)
}

pub fn format_tokens(tokens: &Vec<Tk>) -> String {
    let result: Vec<String> = tokens.iter().enumerate().map(|(idx, t)| {
        match format_token(&t) {
            Some(token) => format!("{:>3}: {}", idx, token),
            None => "".to_string()
        }
    }).filter(String::is_empty).collect();

    result.join("\n")
}

