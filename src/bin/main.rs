extern crate rattlesnake;

use rattlesnake::opcode::OpCode;
use rattlesnake::instruction::*;

use std::io::prelude::*;
use std::fs::File;

static FILEPATH: &'static str = "tests/python/e0002_add_x_plus_y.pyc";


fn main() {

    let mut f = File::open(FILEPATH).unwrap();

    for byte in f.bytes() {
        let b = byte.unwrap();
        println!("{} = {:?}", b, OpCode::from(b));
    }

    println!("{:?}", PYFUNC_ADD);
}
