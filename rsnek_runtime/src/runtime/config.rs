use ::system::{Argv, ThreadModel};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum Mode {
    Interactive,
    Command(String),
    Module(String),
    File
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Config<'a> {
    pub mode: Mode,
    pub arguments: Argv<'a>,
    pub debug_support: bool,
    pub thread_model: ThreadModel,
    pub logging: Logging
}



#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Logging;
