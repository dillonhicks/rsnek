use std::fmt::{self, Display, Formatter};
use nom::FindToken;

use serde::ser::{SerializeStruct,Serialize, Serializer};
use serde_bytes;

use ::slice::TkSlice;

//pub const TAB_BYTES:     &'static [u8] = &[9];
pub const NEWLINE_BYTES: &'static [u8] = &[10];
pub const SPACE_BYTES:   &'static [u8] = &[32];

//pub const TK_TAB: Tk        = Tk::const_(Id::Tab,     TAB_BYTES,     Tag::W(Ws::Tab));
pub const TK_NEWLINE: Tk    = Tk::const_(Id::Newline, NEWLINE_BYTES, Tag::W(Ws::Newline));
pub const TK_SPACE: Tk      = Tk::const_(Id::Space,   SPACE_BYTES,   Tag::W(Ws::Space));

//pub const TAB:      &'static [Tk]       = &[TK_TAB];
pub const NEWLINE:  &'static [Tk]       = &[TK_NEWLINE];
pub const SPACE:    &'static [Tk]       = &[TK_SPACE];


/// Attempt to make an owned token to get out of lifetime hell. I found myself
/// in trouble after trying to rewrite and inject values into the token slice
/// in the parsing phase. This was to figure out block scopes and such since
/// something something whitespace scoping.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct OwnedTk {
    id: Id,
    bytes: Vec<u8>,
    tag: Tag
}

impl<'a> From<&'a Tk<'a>> for OwnedTk {
    fn from(tk: &'a Tk<'a>) -> Self {
        OwnedTk {
            id: tk.id,
            bytes: tk.bytes.to_vec(),
            tag: tk.tag
        }
    }
}

impl<'a> From<&'a TkSlice<'a>> for OwnedTk {
    fn from(tkslice: &'a TkSlice<'a>) -> Self {
        let tk = tkslice.as_token();
        OwnedTk {
            id: tk.id,
            bytes: tk.bytes.to_vec(),
            tag: tk.tag
        }
    }
}

impl OwnedTk {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn tag(&self) -> Tag { self.tag }

    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(self.bytes()).to_string()
    }

}


impl Serialize for OwnedTk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("OwnedTk", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("value", &self.as_string())?;
        //state.serialize_field("tag", &self.tag)?;
        state.end()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Tk<'a> {
    id: Id,

    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    tag: Tag
}


impl<'a> Tk<'a> {

    /// Alternative constructor for const Tk definitions so there is not a need
    /// to make the struct fields public just to declare a const Tk in another
    /// module.
    #[inline(always)]
    pub const fn const_(id: Id, bytes: &'static[u8], tag: Tag) -> Tk {
        Tk {
            id: id,
            bytes: bytes,
            tag: tag
        }
    }

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

    N(Num),
    S(Str),
    O(Op),
    W(Ws),
    M(Sym),
    E(Error),

    Note(&'static str)
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

    // Artificially created by the preprocessors
    BlockStart,
    BlockEnd,
    ExprStart,
    ExprEnd,
    LineContinuation,

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
    Async,
    Await,
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
    NotIn,
    IsNot,
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

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Id::{:?}", self)
    }
}

