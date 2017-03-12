use std;

pub trait Exception: Sized + std::fmt::Debug + std::fmt::Display{
    fn error_type(&self) -> ErrorType;
    fn message(&self) -> &'static str;
}

#[derive(Debug,Clone,Copy)]
pub struct Error(pub ErrorType, pub &'static str);

#[deprecated]
#[derive(Debug,Clone,Copy)]
pub struct InterpreterError {
    pub message: &'static str
}

#[derive(Debug,Clone,Copy)]
pub enum ErrorType {
    Runtime,
    Type,
    Overflow
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[deprecated]
impl Exception for InterpreterError {
    fn error_type(&self) -> ErrorType {
        ErrorType::Runtime
    }

    fn message(&self) -> &'static str {
        self.message
    }
}

impl Exception for Error {
    fn error_type(&self) -> ErrorType {
        self.0.clone()
    }

    fn message(&self) -> &'static str {
        self.1
    }
}