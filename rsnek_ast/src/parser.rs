use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use tokenizer::Tokenizer;
use token::{Id, Tk};

use nom::IResult;

pub struct Parser {}

impl Parser {

    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_file(&self, filename: &str) {

        let mut contents: Vec<u8> = Vec::new();
        {
            let mut file = File::open(filename).unwrap();
            file.read_to_end(&mut contents).unwrap();
        }

        let bytes = contents.as_slice();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Tokenizer::tokenize(bytes);
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();
        match b {
            &IResult::Done(_, ref tokens) => {
                pprint(&tokens)
            },
            _ => {}
        }
    }
}


fn pprint(tokens: &Vec<Tk>) {
    for t in tokens {
        if t.id() == Id::Space {
            continue
        }
        println!("{:<?}: {:?}", t.id(), String::from_utf8_lossy(t.bytes()))
    }
}



