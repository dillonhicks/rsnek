#![feature(associated_consts)]

use opcode::*;

#[derive(Debug)]
pub struct Instruction {
    op: OpCode,
    func: fn(i32,i32) -> i32
}


fn add(x: i32, y: i32) -> i32 {
    return x + y
}

pub const PYFUNC_ADD : Instruction = Instruction { op: OpCode::BinaryAdd, func: add};



