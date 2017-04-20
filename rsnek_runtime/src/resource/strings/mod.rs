pub const PROGRAM: &'static str = include_str!("program.in");
pub const AUTHORS: &'static str = include_str!("authors.in");
pub const LICENSE: &'static str = include_str!("license.in");
pub const ABOUT:   &'static str = include_str!("about.in");
pub const BANNER:  &'static str = include_str!("banner.in");
pub const BANNER2: &'static str = include_str!("banner2.in");
pub const VERSION: &'static str = include_str!("version.in");
pub const BUILD:   &'static str = include_str!("build.cfg.in");
pub const PROMPT:  &'static str = include_str!("prompt.in");


// Error Strings and format strings
// if a constant ends in a _F{N} then it is a format string with N positional arguments
pub const ERROR_NATIVE_INT_OVERFLOW: &'static str = "Python int too large to convert to rust usize";
pub const ERROR_NEG_BIT_SHIFT: &'static str = "negative shift count";
