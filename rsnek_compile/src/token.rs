use std::collections::VecDeque;
use std::str;
use std::str::FromStr;
use std;

use nom::{IResult,digit,multispace, newline,FindToken};
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;

use num;
use num::FromPrimitive;

use ::fmt;

pub const TK_BLOCK_START: Tk = Tk{ id: Id::BlockStart, bytes: &[], tag: Tag::None};
pub const TK_BLOCK_END: Tk = Tk{ id: Id::BlockEnd, bytes: &[], tag: Tag::None};

pub const BLOCK_START: &'static [Tk] = &[TK_BLOCK_START];
pub const BLOCK_END: &'static [Tk] = &[TK_BLOCK_END];


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Tk<'a> {
    id: Id,

    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    tag: Tag
}

impl<'a> Tk<'a> {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..]
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn tag(&self) -> Tag { self.tag }

    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(self.bytes).to_string()
    }
}


impl<'a, 'b> FindToken<&'b [Id]> for Tk<'a> {
    fn find_token(&self, input: &[Id]) -> bool {
        for &i in input.iter() {
            if self.id() == i { return true }
        }

        false
    }
}

impl<'a> Default for Tk<'a> {
    fn default() -> Self {
        Tk::new(Id::None, &[], Tag::None)
    }
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

pub fn pprint_tokens(tokens: &Vec<Tk>) {
    for (idx, t) in tokens.iter().enumerate() {
        match fmt::token(&t).as_str() {
            "" => continue,
            string => println!("{:>3}: {}", idx, string),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Error {
    Unrecognized
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[repr(usize)]
pub enum Tag {
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
    BlockStart,
    BlockEnd,

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



