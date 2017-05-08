//! The `Result` and `Error` types returned by almost everything in rsnek
//!
use std;
use std::result;

use ::api::RtObject;
use ::resources::strings;

pub type RtResult<T> = result::Result<T, Error>;
pub type ObjectResult = RtResult<RtObject>;


pub trait Exception: Sized + std::fmt::Debug + std::fmt::Display {
    fn error_type(&self) -> ErrorType;
    fn message(&self) -> String;
}


#[derive(Debug, Clone, Serialize)]
pub struct Error(pub ErrorType, pub String);


#[derive(Debug, Clone, Serialize)]
pub enum ErrorType {
    Runtime,
    Type,
    Overflow,
    NotImplemented,
    Attribute,
    Value,
    Key,
    ModuleNotFound,
    StopIteration,
    Name,
    System,
    Recursion,
    Assertion,
    Syntax,
    Index
}


impl Error {

    pub fn runtime(message: &str) -> Error {
        Error(ErrorType::Runtime, message.to_string())
    }

    pub fn typerr(message: &str) -> Error {
        Error(ErrorType::Type, message.to_string())
    }

    pub fn overflow(message: &str) -> Error {
        Error(ErrorType::Overflow, message.to_string())
    }

    pub fn not_implemented() -> Error {
        Error(ErrorType::NotImplemented, "Not Implemented".to_string())
    }

    pub fn attribute(message: &str) -> Error {
        Error(ErrorType::Attribute, message.to_string())
    }

    pub fn value(message: &str) -> Self {
        Error(ErrorType::Value, message.to_string())
    }

    pub fn key(message: &str) -> Error {
        Error(ErrorType::Key, message.to_string())
    }

    pub fn module_not_found(name: &str) -> Error {
        Error(ErrorType::ModuleNotFound, format!("No module named '{}'", name))
    }

    pub fn stop_iteration() -> Error {
        return Error(ErrorType::StopIteration, "".to_string())
    }

    pub fn name(name: &str) -> Error {
        Error(ErrorType::Name, format!("name '{}' is not defined", name))
    }

    pub fn system(message: &str) -> Error {
        Error(ErrorType::System, format!("{}, version: {}", message, strings::VERSION))
    }

    pub fn system_not_implemented(name: &str, extra: &str) -> Error {
        Error(ErrorType::System, format!(
            "feature '{}' not implemented for type; {}, version: {}",
            name, extra, strings::VERSION))
    }

    pub fn recursion() -> Error {
        Error(ErrorType::Recursion, "Maximum recursion depth exceeded".to_string())
    }

    pub fn assertion(message: &str) -> Error {
        Error(ErrorType::Assertion, message.to_string())
    }

    pub fn syntax(message: &str) -> Error {
        Error(ErrorType::Syntax, message.to_string())
    }

    pub fn index(message: &str) -> Error {
        Error(ErrorType::Index, message.to_string())
    }


    pub fn log(&self) {
        error!("{:?}Error", self.error_type(); "message" => self.message());
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}Error: {}", self.error_type(), self.message())
    }
}


impl Exception for Error {
    fn error_type(&self) -> ErrorType {
        self.0.clone()
    }

    fn message(&self) -> String {
        self.1.to_string()
    }

}
