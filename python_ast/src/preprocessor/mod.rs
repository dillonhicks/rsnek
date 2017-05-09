//! Useful `TkSlice` transformations before parsing with `Parser` which would otherwise
//! complicate tokenizing or parsing.
//!
mod traits;
mod blockscope;

pub use self::traits::Preprocessor;
pub use self::blockscope::BlockScopePreprocessor;
