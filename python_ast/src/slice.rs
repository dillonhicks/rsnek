use std::slice::Iter;
use std::str;
use std::ops::{Range, RangeTo, RangeFrom, RangeFull};
use std::iter::{Enumerate, Iterator};
use std::cmp::Ordering;

use nom::{Compare, AsChar, CompareResult, InputLength, InputIter, Slice, FindToken};


use ::token::{Tk, Id, OwnedTk};

#[derive(Clone, Debug, Copy, Serialize, Ord, Eq, PartialEq)]
pub struct TkSlice<'a>(pub &'a [Tk<'a>]);

impl<'a> TkSlice<'a> {
    pub fn iter(&self) -> Iter<Tk<'a>> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Convert the slice back into a token assuming it is a
    /// slice of exactly len 1.
    pub fn as_token(&self) -> Tk<'a> {
        assert_eq!(self.len(), 1);
        self.0[0].clone()
    }

    /// Convert the slice back into a token assuming it is a
    /// slice of exactly len 1.
    pub fn as_owned_token(&self) -> OwnedTk {
        assert_eq!(self.len(), 1);
        OwnedTk::from(&self.0[0])
    }

    pub fn tokens(&self) -> &'a [Tk<'a>] {
        &self.0[..]
    }

    pub fn as_owned_tokens(&self) -> Vec<OwnedTk> {
        self.iter().map(OwnedTk::from).collect()
    }


    /// Convert a token slice to a string
    pub fn as_string(&self) -> String {
        self.iter()
            .map(Tk::as_string)
            .collect::<Vec<String>>()
            .concat()
    }
}



impl<'a> InputLength for TkSlice<'a> {
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Slice<Range<usize>> for TkSlice<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        TkSlice(&self.0[range.start..range.end])
    }
}


impl<'a> Slice<RangeTo<usize>> for TkSlice<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        TkSlice(&self.0[0..range.end])
    }
}


impl<'a> Slice<RangeFrom<usize>> for TkSlice<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        TkSlice(&self.0[range.start..])
    }
}


impl<'a> Slice<RangeFull> for TkSlice<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        TkSlice(&self.0[..])
    }
}

impl<'a,'b> Compare<&'b[Id]> for TkSlice<'a> {
    fn compare(&self, t: &'b [Id]) -> CompareResult {

        let len = self.0.len();
        let blen = t.len();
        let m = if len < blen {
            len
        } else {
            blen
        };
        let reduced = &self.0[..m];
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
        self.compare(t)
    }
}

impl<'a> PartialOrd for TkSlice<'a> {
    fn partial_cmp(&self, other: &TkSlice) -> Option<Ordering> {
        match (self.0.len(), other.0.len()) {
            (lhs, rhs) if lhs < rhs => Some(Ordering::Less),
            (lhs, rhs) if rhs < lhs => Some(Ordering::Greater),
            _ => {
                self.iter().zip(other.iter())
                    .map(|(t1, t2)| {
                        t1.partial_cmp(t2)
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .filter(|r: &Ordering| *r != Ordering::Equal)
                    .next().or_else(|| Some(Ordering::Equal))
            }
        }
    }
}


impl<'a> InputIter for TkSlice<'a> {
    type Item     = &'a Tk<'a>;
    type RawItem  = Id;
    type Iter     = Enumerate<::std::slice::Iter<'a, Tk<'a>>>;
    type IterElem = ::std::slice::Iter<'a, Tk<'a>>;

    fn iter_indices(&self)  -> Self::Iter {
        self.0.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter()
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




impl<'a> FindToken<TkSlice<'a>> for Id {
    fn find_token(&self, input: TkSlice<'a>) -> bool {
        for ref tk in input.iter() {
            if *self == tk.id() { return true }
        }

        false
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


