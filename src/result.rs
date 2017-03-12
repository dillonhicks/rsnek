use std::result;
use std::fmt::{Debug,Formatter};
use std;

pub type Result<T> = result::Result<T, InterpreterError>;

pub struct InterpreterError {
    pub message: &'static str
}

impl Debug for InterpreterError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "InterpreterError: {}", self.message)
    }
}