use std;
use std::borrow::Borrow;
use std::slice::Iter;
use std::str;
use std::str::FromStr;
use std::ops::{Range,RangeTo,RangeFrom,RangeFull};
use std::iter::{Enumerate,Iterator};
use std::fmt;
use std::cmp::{min,PartialEq};

use nom;
use nom::{Compare, AsChar,CompareResult,InputLength,InputIter,Slice,HexDisplay, InputTake};
use nom::{IResult, digit, multispace};

use token::{Tk, Id};


// TODO: for a Vec<TokenSlice> keep the shared buf as a serialized datablock
//  since it is actually a view and not a copy.
#[derive(Clone, Copy, Serialize, Eq)]
#[repr(C)]
pub struct TokenSlice<'a> {
    #[serde(skip_serializing)]
    buf: &'a [Tk<'a>],
    start: usize,
    end:   usize,
}


impl<'a> TokenSlice<'a> {
    pub fn new(tokens: &'a [Tk]) -> Self {
        TokenSlice {
            buf: tokens,
            start: 0,
            end: tokens.len()
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn iter(&self) -> Iter<Tk<'a>> {
        if self.len() == 0 || self.end < self.start {
            return self.buf[self.start..self.start].iter()
        }

        self.buf[self.start..self.end].iter()
    }

    pub fn len(&self) -> usize {
        println!("TRACE: {:?}", self);
        self.end - self.start
    }
}


impl<'a> InputLength for TokenSlice<'a> {
    fn input_len(&self) -> usize {
        self.len()
    }
}


impl<'a> fmt::Debug for TokenSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: &Vec<_> = &self.buf[self.start..self.end].iter()
            .map(|tk| (tk.id(), String::from_utf8_lossy(tk.bytes()).to_string()))
            .collect();

        write!(f, "TokenSlice {{start: {}, end: {}, data: {} }}",
               self.start,
               self.end,
               format!("{:?}", data))
    }
}


impl<'a> PartialEq for TokenSlice<'a> {
    fn eq(&self, other: &TokenSlice<'a>) -> bool {
        let bufs = (self.buf as *const _) == (other.buf as *const _);
        self.start == other.start && self.end == other.end && bufs
    }
}


impl<'a> PartialOrd for TokenSlice<'a> {
    fn partial_cmp(&self, other: &TokenSlice<'a>) -> Option<std::cmp::Ordering> {
        None
    }
}


impl<'a> Slice<Range<usize>> for TokenSlice<'a> {
    fn slice(&self, range:Range<usize>) -> Self {
        TokenSlice {
            buf:   self.buf,
            start: self.start + range.start,
            //FIXME: check for valid end here
            end:   range.end,
        }
    }
}


impl<'a> Slice<RangeTo<usize>> for TokenSlice<'a> {
    fn slice(&self, range:RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}


impl<'a> Slice<RangeFrom<usize>> for TokenSlice<'a> {
    fn slice(&self, range:RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end)
    }
}


impl<'a> Slice<RangeFull> for TokenSlice<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        TokenSlice {
            buf: self.buf,
            start: self.start,
            end: self.end,
        }
    }
}


impl<'a,'b> Compare<&'b[Id]> for TokenSlice<'a> {
    fn compare(&self, t: &'b [Id]) -> CompareResult {

        if self.start == self.end  {
            return CompareResult::Incomplete;
        }

        let len = self.end - self.start;
        let blen = t.len();
        let m = if len < blen {
            len
        } else {
            blen
        };


        let reduced = self.slice(..m);
        let b = &t[..m];

        for (a, b) in reduced.iter().map(Tk::id).zip(b.iter()) {
            if a != *b {
                return CompareResult::Error;
            }
        }
        if m < blen {
            CompareResult::Incomplete
        } else {
            CompareResult::Ok
        }
    }

    fn compare_no_case(&self, t: &'b [Id]) -> CompareResult {
        unimplemented!()
    }
}


// Only for u8 -> str abstraction
//
impl<'a> InputIter for TokenSlice<'a> {
    type Item     = &'a Tk<'a>;
    type RawItem  = Id;
    type Iter     = Enumerate<::std::slice::Iter<'a, Tk<'a>>>;
    type IterElem = ::std::slice::Iter<'a, Tk<'a>>;

    fn iter_indices(&self)  -> Self::Iter {
        self.buf[..].iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.buf.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::RawItem) -> bool {
        self.iter_elements().map(Tk::id).position(|b| predicate(b))
    }

    fn slice_index(&self, count:usize) -> Option<usize> {
        if self.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}

impl AsChar for Id {
    fn as_char(self) -> char {
        0 as char
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }
}

impl<'a> AsChar for &'a Tk<'a> {
    fn as_char(self) -> char {
        0 as char
    }

    fn is_alpha(self) -> bool {
        false
    }

    fn is_alphanum(self) -> bool {
        false
    }

    fn is_dec_digit(self) -> bool {
        false
    }

    fn is_hex_digit(self) -> bool {
        false
    }

    fn is_oct_digit(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
enum Ast<'a> {
    Any(TokenSlice<'a>)
}


mod tests {
    use super::*;
    use std::rc::Rc;

    use tokenizer::{Lexer};

    use serde_json;
    use serde_yaml;



    #[test]
    fn slice() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("lambda x, **y: x(y['1'], y['2'], y.get(10))".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                let sub = slice.slice(2..);
                assert_eq!(sub.len(), slice.len() - 2);
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn compare() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("a=2**3.0".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                let expect = [Id::Name, Id::Equal, Id::Number, Id::DoubleStar, Id::Number];
                assert_eq!(slice.compare(&expect), CompareResult::Ok);
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn repr_debug() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("print('pain and suffering')".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                println!("{:?}", slice);
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn repr_json() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("print('pain and suffering')".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                println!("{}", serde_json::to_string_pretty(&slice).unwrap());
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn repr_yaml() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("print('pain and suffering')".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                println!("{}", serde_yaml::to_string(&slice).unwrap());
            },
            _ => unreachable!()
        }
    }
}
