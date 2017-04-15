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


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
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
    String,
    Newline,
    Space,
    Operator,
    Symbol,
    ErrorMarker,


    Keyword,
    None,

    // Strings
    Comment,


    RawString,
    ByteString,
    FormatString,



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
        match t.id() {
            Id::Space => continue,
            Id::Newline => println!("{:>3}: {:>15} {:^20}{:>10}", idx, format!("{:?}", t.id()), "\\n", format!("{:?}", t.tag())),
            _ => println!("{:>3}: {:>15} {:^20}{:>10}", idx, format!("{:?}", t.id()), String::from_utf8_lossy(t.bytes()), format!("{:?}", t.tag()))

        }
    }
}

// end //


#[cfg(feature = "none")]
mod _old {
    pub type ByteVector = Vec<u8>;

    #[derive(Debug)]
    pub struct Token {
        pub id: Id,
        pub data: ByteVector
    }


    pub trait NewToken<T> {
        fn new(id: Id, value: T) -> Self;
    }

    impl Token {}

    impl NewToken<u8> for Token {
        fn new(id: Id, value: u8) -> Self {
            Token {
                id: id,
                data: vec![value]
            }
        }
    }

    impl NewToken<(u8, u8)> for Token {
        fn new(id: Id, value: (u8, u8)) -> Self {
            Token {
                id: id,
                data: vec![value.0, value.1]
            }
        }
    }


    impl NewToken<(u8, u8, u8)> for Token {
        fn new(id: Id, value: (u8, u8, u8)) -> Self {
            Token {
                id: id,
                data: vec![value.0, value.1, value.2]
            }
        }
    }

    impl<'a> NewToken<&'a [u8]> for Token {
        fn new(id: Id, value: &[u8]) -> Self {
            Token {
                id: id,
                data: value.to_vec()
            }
        }
    }

    impl NewToken<ByteVector> for Token {
        fn new(id: Id, value: ByteVector) -> Self {
            Token {
                id: id,
                data: value
            }
        }
    }

    impl NewToken<()> for Token {
        #[allow(unused_variables)]
        fn new(id: Id, value: ()) -> Self {
            NewToken::new(id, Vec::new())
        }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    #[repr(usize)]
    pub enum Id {
        ENDMARKER = 0,
        NAME = 1,
        NUMBER = 2,
        STRING = 3,
        NEWLINE = 4,
        INDENT = 5,
        DEDENT = 6,
        LPAR = 7,
        RPAR = 8,
        LSQB = 9,
        RSQB = 10,
        COLON = 11,
        COMMA = 12,
        SEMI = 13,
        PLUS = 14,
        MINUS = 15,
        STAR = 16,
        SLASH = 17,
        VBAR = 18,
        AMPER = 19,
        LESS = 20,
        GREATER = 21,
        EQUAL = 22,
        DOT = 23,
        PERCENT = 24,
        LBRACE = 25,
        RBRACE = 26,
        EQEQUAL = 27,
        NOTEQUAL = 28,
        LESSEQUAL = 29,
        GREATEREQUAL = 30,
        TILDE = 31,
        CIRCUMFLEX = 32,
        LEFTSHIFT = 33,
        RIGHTSHIFT = 34,
        DOUBLESTAR = 35,
        PLUSEQUAL = 36,
        MINEQUAL = 37,
        STAREQUAL = 38,
        SLASHEQUAL = 39,
        PERCENTEQUAL = 40,
        AMPEREQUAL = 41,
        VBAREQUAL = 42,
        CIRCUMFLEXEQUAL = 43,
        LEFTSHIFTEQUAL = 44,
        RIGHTSHIFTEQUAL = 45,
        DOUBLESTAREQUAL = 46,
        DOUBLESLASH = 47,
        DOUBLESLASHEQUAL = 48,
        AT = 49,
        ATEQUAL = 50,
        RARROW = 51,
        ELLIPSIS = 52,

        OP = 53,
        AWAIT = 54,
        ASYNC = 55,
        ERRORTOKEN = 56
    }


    impl Id {
        pub fn new(value: usize) -> Option<Self> {
            Id::from_usize(value)
        }

        pub const fn offset() -> usize {
            256
        }

        fn terminal(&self) -> bool {
            (*self as usize) < Id::offset()
        }
    }

    impl num::FromPrimitive for Id {
        fn from_i64(n: i64) -> Option<Self> {
            match n {
                0 => Some(Id::ENDMARKER),
                1 => Some(Id::NAME),
                2 => Some(Id::NUMBER),
                3 => Some(Id::STRING),
                4 => Some(Id::NEWLINE),
                5 => Some(Id::INDENT),
                6 => Some(Id::DEDENT),
                7 => Some(Id::LPAR),
                8 => Some(Id::RPAR),
                9 => Some(Id::LSQB),
                10 => Some(Id::RSQB),
                11 => Some(Id::COLON),
                12 => Some(Id::COMMA),
                13 => Some(Id::SEMI),
                14 => Some(Id::PLUS),
                15 => Some(Id::MINUS),
                16 => Some(Id::STAR),
                17 => Some(Id::SLASH),
                18 => Some(Id::VBAR),
                19 => Some(Id::AMPER),
                20 => Some(Id::LESS),
                21 => Some(Id::GREATER),
                22 => Some(Id::EQUAL),
                23 => Some(Id::DOT),
                24 => Some(Id::PERCENT),
                25 => Some(Id::LBRACE),
                26 => Some(Id::RBRACE),
                27 => Some(Id::EQEQUAL),
                28 => Some(Id::NOTEQUAL),
                29 => Some(Id::LESSEQUAL),
                30 => Some(Id::GREATEREQUAL),
                31 => Some(Id::TILDE),
                32 => Some(Id::CIRCUMFLEX),
                33 => Some(Id::LEFTSHIFT),
                34 => Some(Id::RIGHTSHIFT),
                35 => Some(Id::DOUBLESTAR),
                36 => Some(Id::PLUSEQUAL),
                37 => Some(Id::MINEQUAL),
                38 => Some(Id::STAREQUAL),
                39 => Some(Id::SLASHEQUAL),
                40 => Some(Id::PERCENTEQUAL),
                41 => Some(Id::AMPEREQUAL),
                42 => Some(Id::VBAREQUAL),
                43 => Some(Id::CIRCUMFLEXEQUAL),
                44 => Some(Id::LEFTSHIFTEQUAL),
                45 => Some(Id::RIGHTSHIFTEQUAL),
                46 => Some(Id::DOUBLESTAREQUAL),
                47 => Some(Id::DOUBLESLASH),
                48 => Some(Id::DOUBLESLASHEQUAL),
                49 => Some(Id::AT),
                50 => Some(Id::ATEQUAL),
                51 => Some(Id::RARROW),
                52 => Some(Id::ELLIPSIS),
                53 => Some(Id::OP),
                54 => Some(Id::AWAIT),
                55 => Some(Id::ASYNC),
                56 => Some(Id::ERRORTOKEN),
                _ => None
            }
        }

        fn from_u64(n: u64) -> Option<Self> {
            match n {
                0 => Some(Id::ENDMARKER),
                1 => Some(Id::NAME),
                2 => Some(Id::NUMBER),
                3 => Some(Id::STRING),
                4 => Some(Id::NEWLINE),
                5 => Some(Id::INDENT),
                6 => Some(Id::DEDENT),
                7 => Some(Id::LPAR),
                8 => Some(Id::RPAR),
                9 => Some(Id::LSQB),
                10 => Some(Id::RSQB),
                11 => Some(Id::COLON),
                12 => Some(Id::COMMA),
                13 => Some(Id::SEMI),
                14 => Some(Id::PLUS),
                15 => Some(Id::MINUS),
                16 => Some(Id::STAR),
                17 => Some(Id::SLASH),
                18 => Some(Id::VBAR),
                19 => Some(Id::AMPER),
                20 => Some(Id::LESS),
                21 => Some(Id::GREATER),
                22 => Some(Id::EQUAL),
                23 => Some(Id::DOT),
                24 => Some(Id::PERCENT),
                25 => Some(Id::LBRACE),
                26 => Some(Id::RBRACE),
                27 => Some(Id::EQEQUAL),
                28 => Some(Id::NOTEQUAL),
                29 => Some(Id::LESSEQUAL),
                30 => Some(Id::GREATEREQUAL),
                31 => Some(Id::TILDE),
                32 => Some(Id::CIRCUMFLEX),
                33 => Some(Id::LEFTSHIFT),
                34 => Some(Id::RIGHTSHIFT),
                35 => Some(Id::DOUBLESTAR),
                36 => Some(Id::PLUSEQUAL),
                37 => Some(Id::MINEQUAL),
                38 => Some(Id::STAREQUAL),
                39 => Some(Id::SLASHEQUAL),
                40 => Some(Id::PERCENTEQUAL),
                41 => Some(Id::AMPEREQUAL),
                42 => Some(Id::VBAREQUAL),
                43 => Some(Id::CIRCUMFLEXEQUAL),
                44 => Some(Id::LEFTSHIFTEQUAL),
                45 => Some(Id::RIGHTSHIFTEQUAL),
                46 => Some(Id::DOUBLESTAREQUAL),
                47 => Some(Id::DOUBLESLASH),
                48 => Some(Id::DOUBLESLASHEQUAL),
                49 => Some(Id::AT),
                50 => Some(Id::ATEQUAL),
                51 => Some(Id::RARROW),
                52 => Some(Id::ELLIPSIS),
                53 => Some(Id::OP),
                54 => Some(Id::AWAIT),
                55 => Some(Id::ASYNC),
                56 => Some(Id::ERRORTOKEN),
                _ => None
            }
        }
    }

    impl num::ToPrimitive for Id {
        fn to_i64(&self) -> Option<i64> {
            let id = *self as usize;
            Some(id as i64)
        }

        fn to_u64(&self) -> Option<u64> {
            let id = *self as usize;
            Some(id as u64)
        }
    }
}