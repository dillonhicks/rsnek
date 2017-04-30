use std;

pub trait Exception: Sized + std::fmt::Debug + std::fmt::Display {
    fn error_type(&self) -> ErrorType;
    fn message(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Error(pub ErrorType, pub String);


#[derive(Debug, Clone)]
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
    System
}


impl Error {
    pub fn runtime(message: &str) -> Error {
        return Error(ErrorType::Runtime, message.to_string());
    }

    pub fn typerr(message: &str) -> Error {
        return Error(ErrorType::Type, message.to_string());
    }

    pub fn overflow(message: &str) -> Error {
        return Error(ErrorType::Overflow, message.to_string());
    }

    pub fn not_implemented() -> Error {
        return Error(ErrorType::NotImplemented, "Not Implemented".to_string());
    }

    pub fn attribute(message: &str) -> Error {
        return Error(ErrorType::Attribute, message.to_string());
    }

    pub fn value(message: &str) -> Self {
        return Error(ErrorType::Value, message.to_string());
    }

    pub fn key(message: &str) -> Error {
        return Error(ErrorType::Key, message.to_string());
    }

    pub fn module_not_found(name: &'static str) -> Error {
        return Error(ErrorType::ModuleNotFound, format!("could not find {:?}", name));
    }

    pub fn stop_iteration() -> Error {
        return Error(ErrorType::StopIteration, "".to_string());
    }

    pub fn name(name: &str) -> Error {
         Error(ErrorType::Name, format!("name '{}' is not defined", name))
    }

    pub fn system(message: &str) -> Error {Error(ErrorType::System, message.to_string())}
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
