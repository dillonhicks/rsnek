/// Exit codes for the main interpreter loops
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum ExitCode {
    Ok = 0,
    GenericError = 1,
    SyntaxError = 2,
    NotImplemented = 3,
}
