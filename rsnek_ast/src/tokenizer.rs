use std;
use std::cell::{RefMut, RefCell};
use std::ops::Deref;
use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;

use num::FromPrimitive;
use itertools::{Itertools};


use token::{Token, Id, NewToken};

//#define is_potential_identifier_start(c) (\
//(c >= 'a' && c <= 'z')\
//|| (c >= 'A' && c <= 'Z')\
//|| c == '_'\
//|| (c >= 128))
//
//#define is_potential_identifier_char(c) (\
//(c >= 'a' && c <= 'z')\
//|| (c >= 'A' && c <= 'Z')\
//|| (c >= '0' && c <= '9')\
//|| c == '_'\
//|| (c >= 128))

fn is_potential_identifier_start(ch: u8) -> bool {
    match ch as char {
        'A' ... 'Z' |
        'a' ... 'z' |
        '_' => true,
        _ => ch >= 128
    }
}

fn is_potential_identifier_char(ch: u8) -> bool {
    match ch as char {
        '0' ... '9' => true,
        _ => is_potential_identifier_start(ch)
    }
}

/*
    /* Input state; buf <= cur <= inp <= end */
    /* NB an entire line is held in the buffer */
    char *buf;          /* Input buffer, or NULL; malloc'ed if fp != NULL */
    char *cur;          /* Next character in buffer */
    char *inp;          /* End of data in buffer */
    char *end;          /* End of input buffer if buf != NULL */
    char *start;        /* Start of current token if not NULL */
    int done;           /* E_OK normally, E_EOF at EOF, otherwise error code */
    /* NB If done != E_OK, cur must be == inp!!! */
    FILE *fp;           /* Rest of input; NULL if tokenizing a string */
    int tabsize;        /* Tab spacing */
    int indent;         /* Current indentation index */
    int indstack[MAXINDENT];            /* Stack of indents */
    int atbol;          /* Nonzero if at begin of new line */
    int pendin;         /* Pending indents (if > 0) or dedents (if < 0) */
    const char *prompt, *nextprompt;          /* For interactive prompting */
    int lineno;         /* Current line number */
    int level;          /* () [] {} Parentheses nesting level */
            /* Used to allow free continuations inside them */
    /* Stuff for checking on different tab sizes */
#ifndef PGEN
    /* pgen doesn't have access to Python codecs, it cannot decode the input
       filename. The bytes filename might be kept, but it is only used by
       indenterror() and it is not really needed: pgen only compiles one file
       (Grammar/Grammar). */
    PyObject *filename;
#endif
    int altwarning;     /* Issue warning if alternate tabs don't match */
    int alterror;       /* Issue error if alternate tabs don't match */
    int alttabsize;     /* Alternate tab spacing */
    int altindstack[MAXINDENT];         /* Stack of alternate indents */
    /* Stuff for PEP 0263 */
    enum decoding_state decoding_state;
    int decoding_erred;         /* whether erred in decoding  */
    int read_coding_spec;       /* whether 'coding:...' has been read  */
    char *encoding;         /* Source encoding. */
    int cont_line;          /* whether we are in a continuation line. */
    const char* line_start;     /* pointer to start of current line */
#ifndef PGEN
    PyObject *decoding_readline; /* open(...).readline */
    PyObject *decoding_buffer;
#endif
    const char* enc;        /* Encoding for the current str. */
    const char* str;
    const char* input; /* Tokenizer's newline translated copy of the string. */

    /* async/await related fields; can be removed in 3.7 when async and await
       become normal keywords. */
    int async_def;        /* =1 if tokens are inside an 'async def' body. */
    int async_def_indent; /* Indentation level of the outermost 'async def'. */
    int async_def_nl;     /* =1 if the outermost 'async def' had at least one
                             NEWLINE token after it. */
*/
pub type TokenResult = std::result::Result<Token, Error>;



enum Error{
    IO(std::io::Error),
    Parse(),
    TOO_DEEP(),
    DEDENT(),
}


pub type UInt = Box<usize>;
pub type Int = Box<isize>;

const MAXINDENT: usize = 100;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Position {
    Beginning,
    Offset,
    End
}



pub type ByteStream = PutBackN<Bytes<File>>;


fn bytestream(filename: &'static str) -> ByteStream {
        let mut file = File::open(filename).unwrap();
        ByteStream  {
            top: Vec::new(),
            iter: file.bytes()
        }
}


pub struct Tokenizer {
    filename: String,
    stream: ByteStream,
    line: UInt,
    column: UInt,
    indent: UInt,
    line_pos: Box<Position>,
    paren_level: UInt,
    indent_stack: [usize; MAXINDENT],
    altindent_stack: [usize; MAXINDENT],
    pending_dents: Int,
}

impl Tokenizer {
    pub fn from_file(filename: &'static str) -> Self {
        Tokenizer {
            filename: filename.to_string(),
            stream: bytestream(filename),
            line: UInt::new(0),
            column: UInt::new(0),
            indent: UInt::new(0),
            line_pos: Box::new(Position::Beginning),
            paren_level: UInt::new(0),
            indent_stack: [0; MAXINDENT],
            altindent_stack: [0; MAXINDENT],
            pending_dents: Int::new(0),
        }
    }

    fn process_next_byte(&self, byte: u8) -> Option<TokenResult> {
        if is_potential_identifier_char(byte) {
            Some(Ok(Token::new(Id::NAME, vec![byte])))
        } else {
            Some(Ok(self.match_one_byte(byte)))
        }
    }

    fn match_one_byte(&self, ch: u8) -> Token {
        match ch as char {
            '(' => Token::new(Id::LPAR, ch),
            ')' => Token::new(Id::RPAR, ch),
            '[' => Token::new(Id::LSQB, ch),
            ']' => Token::new(Id::RSQB, ch),
            ':' => Token::new(Id::COLON, ch),
            ',' => Token::new(Id::COMMA, ch),
            ';' => Token::new(Id::SEMI, ch),
            '+' => Token::new(Id::PLUS, ch),
            '-' => Token::new(Id::MINUS, ch),
            '*' => Token::new(Id::STAR, ch),
            '/' => Token::new(Id::SLASH, ch),
            '|' => Token::new(Id::VBAR, ch),
            '&' => Token::new(Id::AMPER, ch),
            '<' => Token::new(Id::LESS, ch),
            '>' => Token::new(Id::GREATER, ch),
            '=' => Token::new(Id::EQUAL, ch),
            '.' => Token::new(Id::DOT, ch),
            '%' => Token::new(Id::PERCENT, ch),
            '{' => Token::new(Id::LBRACE, ch),
            '}' => Token::new(Id::RBRACE, ch),
            '^' => Token::new(Id::CIRCUMFLEX, ch),
            '~' => Token::new(Id::TILDE, ch),
            '@' => Token::new(Id::AT, ch),
            _ => Token::new(Id::OP, ch)
        }
    }

    fn match_two_bytes(&self, ch1: u8, ch2: u8) -> Token {
        let arr = [ch1, ch2];
        let bytes = &arr[..];
        let chars = std::str::from_utf8(bytes).unwrap_or("");
        
        match chars {
            "==" => Token::new(Id::EQEQUAL, bytes),
            "!=" => Token::new(Id::NOTEQUAL, bytes),
            "<>" => Token::new(Id::NOTEQUAL, bytes),
            "<=" => Token::new(Id::LESSEQUAL, bytes),
            "<<" => Token::new(Id::LEFTSHIFT, bytes),
            ">=" => Token::new(Id::GREATEREQUAL, bytes),
            ">>" => Token::new(Id::LESSEQUAL, bytes),
            "+=" => Token::new(Id::PLUSEQUAL, bytes),
            "-=" => Token::new(Id::MINEQUAL, bytes),
            "->" => Token::new(Id::RARROW, bytes),
            "**" => Token::new(Id::DOUBLESTAR, bytes),
            "*=" => Token::new(Id::STAREQUAL, bytes),
            "//" => Token::new(Id::DOUBLESLASH, bytes),
            "/=" => Token::new(Id::SLASHEQUAL, bytes),
            "|=" => Token::new(Id::VBAREQUAL, bytes),
            "%=" => Token::new(Id::PERCENTEQUAL, bytes),
            "&=" => Token::new(Id::AMPEREQUAL, bytes),
            "^=" => Token::new(Id::CIRCUMFLEXEQUAL, bytes),
            "@=" => Token::new(Id::ATEQUAL, bytes),
            _ => Token::new(Id::OP, bytes)
        }
    }

    fn match_three_bytes(&self, ch1: u8, ch2: u8, ch3: u8) -> Token {
        let arr = [ch1, ch2, ch3];
        let bytes = &arr[..];
        let chars = std::str::from_utf8(&bytes).unwrap_or("");

        match chars {
            "<<=" => Token::new(Id::LEFTSHIFTEQUAL, bytes),
            ">>=" => Token::new(Id::RIGHTSHIFTEQUAL, bytes),
            "**=" => Token::new(Id::DOUBLESTAREQUAL, bytes),
            "//=" => Token::new(Id::DOUBLESLASHEQUAL, bytes),
            "..." => Token::new(Id::ELLIPSIS, bytes),
            _ => Token::new(Id::OP, bytes)
        }
    }

    fn check_indent(&mut self) -> Result<(), Error> {
        Ok(())

//        if (tok->alterror) {
//            tok->done = E_TABSPACE;
//            tok->cur = tok->inp;
//        return 1;
//        }
//        if (tok->altwarning) {
//        #ifdef PGEN
//        PySys_WriteStderr("inconsistent use of tabs and spaces "
//        "in indentation\n");
//        #else
//        PySys_FormatStderr("%U: inconsistent use of tabs and spaces "
//        "in indentation\n", tok->filename);
//        #endif
//        tok->altwarning = 0;
//        }
//        return 0;
//        }
    }

    fn get_token(&mut self) -> Option<TokenResult> {
        let mut blankline = false;
        let mut ch: u8;

        'next_line:  loop {
            let mut column = 0;
            let mut alt_column = 0;
            println!("WP0");
            if *(self.line_pos.deref()) == Position::Beginning {
                'calculate_indent: loop {

                    *self.line_pos = Position::Offset;

                    ch = match self.stream.next() {
                        Some(Ok(ch)) => ch,
                        Some(Err(err)) => return Some(Err(Error::IO(err))),
                        None => return None,
                    };

                    match ch as char {
                        ' ' => {
                            column += 1;
                            alt_column += 1;
                        },
                        '\t' => panic!("You used a tab like a baddie"),
                        _ => break 'calculate_indent
                    }
                }

                self.stream.put_back(Ok(ch));

                match ch as char {
                    '#' | '\n' => blankline = true,
                    _ => {},
                }

                if !blankline && *self.paren_level == 0 {
                    if column == self.indent_stack[*self.indent] {
                        if alt_column != self.altindent_stack[*self.indent] {
                            match self.check_indent() {
                                Ok(_) => {},
                                Err(err) => return Some(Err(err))
                            };
                        }
                    } else if column > self.indent_stack[*self.indent] {
                        if (*self.indent + 1) > MAXINDENT {
                            return Some(Err(Error::TOO_DEEP()))
                        }
                        if (alt_column <= self.altindent_stack[*self.indent]) {
                            match self.check_indent() {
                                Ok(_) => {},
                                Err(err) => return Some(Err(err))
                            };
                        }

                        *self.pending_dents += 1;
                        *self.indent += 1;

                        self.indent_stack[*self.indent] = column;
                        self.altindent_stack[*self.indent] = alt_column;
                    } else { // tokenizer.c L#1438
                        /* col < tok->indstack[tok->indent] */

                        /* Dedent -- any number, must be consistent */
                        while *self.indent > 0 && column < self.indent_stack[*self.indent] {
                            *self.pending_dents -= 1;
                            *self.indent -= 1;
                        }
                        if column != self.indent_stack[*self.indent] {
                            return Some(Err(Error::DEDENT()))
                        }
                        if alt_column != self.altindent_stack[*self.indent] {
                            match self.check_indent() {
                                Ok(_) => {},
                                Err(err) => return Some(Err(err))
                            };
                        }
                    }

                }

            }

            // tokenizer.c L#1457
            let tk_bytes:Vec<u8> =  vec![];

            if *self.pending_dents != 0 {
                if *self.pending_dents < 0 {
                    *self.pending_dents += 1;
                    return Some(Ok(Token::new(Id::DEDENT, ())));
                } else {
                    *self.pending_dents -= 1;
                    return Some(Ok(Token::new(Id::INDENT, ())));
                }
            }

//            if (tok->async_def
//                && !blankline
//                && tok->level == 0
//                /* There was a NEWLINE after ASYNC DEF,
//                   so we're past the signature. */
//                && tok->async_def_nl
//                /* Current indentation level is less than where
//                   the async function was defined */
//                && tok->async_def_indent >= tok->indent)
//            {
//            tok->async_def = 0;
//            tok->async_def_indent = 0;
//            tok->async_def_nl = 0;
//            }

            // tokenizer.c L#1487

            /* Skip spaces */

            println!("WP1");
            let mut wscount = 0;
            'skip_whitespace: loop {
                let ch = match self.stream.next() {
                    Some(Ok(ch)) => ch,
                    Some(Err(err)) => return Some(Err(Error::IO(err))),
                    None => return None
                };

                match ch as char {
                    ' ' | '\t' |'\x14' => {wscount += 1; continue 'skip_whitespace},
                    _ => {self.stream.put_back(Ok(ch)); break 'skip_whitespace},
                }
            }

            println!("WP2: {:?}", wscount);
            // This is just code to break out of the inf loop
            if let Ok(ch) = self.stream.next().unwrap() {
                return Some(Ok(self.match_one_byte(ch)))
            }

            return Some(Err(Error::Parse()))
        }
    }
}


//
//impl Iterator for Tokenizer {
//    type Item = TokenResult;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let mut buf: RefMut<Bytes<File>> =  self.buffer.borrow_mut();
//
//        match buf.next() {
//            // For file iterators None represents EOF and the iteration returns
//            // Option<Result<u8, IOError>> which introduces double nested matching
//            // to get to the next byte.
//            Some(result) => {
//                match result {
//                    Ok(byte) => self.process_next_byte(byte),
//                    Err(err) => Some(Err(err))
//                }
//            },
//            Option::None => Option::None
//        }
//    }
//}


//impl Itertools for Tokenizer {}
//impl Iterator for ByteStream {
//    type Item = Result<u8, std::io::Error>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if self.buffer.is_empty() {
//            self.buffer.push(self.source.next())
//        }
//
//        self.buffer.pop().unwrap_or(None)
//    }
//}

/// An iterator adaptor that allows putting multiple
/// items in front of the iterator.
///
/// Iterator element type is `I::Item`.
#[derive(Debug, Clone)]
pub struct PutBackN<I: Iterator> {
    top: Vec<I::Item>,
    iter: I,
}

/// Create an iterator where you can put back multiple values to the front
/// of the iteration.
///
/// Iterator element type is `I::Item`.
pub fn put_back_n<I>(iterable: I) -> PutBackN<I::IntoIter>
    where I: IntoIterator
{
    PutBackN {
        top: Vec::new(),
        iter: iterable.into_iter(),
    }
}

impl<I: Iterator> PutBackN<I> {
    /// Puts x in front of the iterator.
    /// The values are yielded in order of the most recently put back
    /// values first.
    ///
    /// rust
    /// use itertools::put_back_n;
    ///
    /// let mut it = put_back_n(1..5);
    /// it.next();
    /// it.put_back(1);
    /// it.put_back(0);
    ///
    /// assert!(itertools::equal(it, 0..5));
    ///
    #[inline]
    pub fn put_back(&mut self, x: I::Item) {
        self.top.push(x);
    }
}

impl<I: Iterator> Iterator for PutBackN<I> {
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.top.is_empty() {
            self.iter.next()
        } else {
            self.top.pop()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::add_scalar(self.iter.size_hint(), self.top.len())
    }
}



mod size_hint {
    use std::usize;

    /// **SizeHint** is the return type of **Iterator::size_hint()**.
    pub type SizeHint = (usize, Option<usize>);

    /// Add **SizeHint** correctly.
    #[inline]
    pub fn add(a: SizeHint, b: SizeHint) -> SizeHint {
        let min = a.0.checked_add(b.0).unwrap_or(usize::MAX);
        let max = match (a.1, b.1) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };

        (min, max)
    }

    /// Add **x** correctly to a **SizeHint**.
    #[inline]
    pub fn add_scalar(sh: SizeHint, x: usize) -> SizeHint {
        let (mut low, mut hi) = sh;
        low = low.saturating_add(x);
        hi = hi.and_then(|elt| elt.checked_add(x));
        (low, hi)
    }
}



#[cfg(test)]
#[allow(non_snake_case)]
mod _Tokenizer {
    use std::rc::Rc;
    use super::*;

    #[test]
    fn tokens() {
        let mut tokenizer = Tokenizer::from_file("../tests/python/e0002_add_x_plus_y.py");
        let mut tokens: Vec<u8> = Vec::new();
        while let Some(Ok(token)) = tokenizer.get_token() {
            println!("tz: {:?}", token);
            let mut cp = token.data.clone();
            tokens.append(&mut cp);
        }

        // Should be the content of the file with all ws besides newlines removed
        println!("{:?}", String::from_utf8(tokens));
    }

}


#[cfg(test)]
#[allow(non_snake_case)]
mod _ByteStream {
    use std::rc::Rc;
    use super::*;

    #[test]
    fn iter() {
        let mut tokenizer = bytestream("../tests/python/e0002_add_x_plus_y.py");
        let mut tokens: Vec<Result<u8, std::io::Error>> = Vec::new();
        let mut count = 0;

        for token in tokenizer {
            println!("{:?}", token);
            tokens.push(token);
        }

    }
//        while !tokens.is_empty() {
//            let token = tokens.pop().unwrap();
//            tokenizer.put_back(token);
//        }
//
//        for token in tokenizer {
//            println!("{:?}", token);
//            tokens.push(token);
//            count += 1;
//        }
//
//        assert_eq!(count, tokens.len())

}