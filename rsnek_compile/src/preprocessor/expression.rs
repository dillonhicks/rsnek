use nom::Slice;

use ::token::{Id, Tk, NEWLINE, NEWLINE_BYTES};
use ::slice::{TkSlice};
use ::preprocessor::Preprocessor;


pub const TK_NEWLINE: Tk        = Tk { id: Id::Newline, bytes: NEWLINE_BYTES, tag: Tag::W(Ws::Newline)};
pub const TK_LINE_CONT: Tk      = Tk { id: Id::LineContinuation, bytes: NEWLINE_BYTES, tag: Tag::None};

const TK_EXPR_START: Tk = Tk {
    id: Id::ExprStart,
    bytes: NEWLINE_BYTES,
    tag: Tag::Note(ExpressionPreprocessor::NAME)
};

const TK_EXPR_END: Tk = Tk {
    id: Id::ExprEnd,
    bytes: NEWLINE_BYTES,
    tag: Tag::Note(ExpressionPreprocessor::NAME)
};

const EXPR_START       : &'static [Tk] = &[TK_EXPR_START];
const EXPR_END         : &'static [Tk] = &[TK_EXPR_END];


// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_mismatch {
    ($len:expr, $indent:expr) => {
        format!("Indent len={} is not a multiple of first indent len={}", $len, $indent);
    }
}

// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_unexpected_indent {
    ($len:expr, $indent:expr) => {
        format!("Unexpected indent len={}, expected indent len={}", $len, $indent);
    }
}


// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_overflow {
    ($max:expr) => {
        format!("Number of INDENT is more than the max allowed {}", $max);
    }
}


/// Preprocessor to splice ExprStart and ExprEnd tokens based on deltas
/// into the slice of tokens so we do not have to keep the state in the main parser
/// logic.
#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct ExpressionPreprocessor {};


impl ExpressionPreprocessor {
    pub const NAME: &'static str = "ExprScopePreprocessor";

    pub fn new() -> Self {
        ExpressionPreprocessor {}
    }
}


impl<'a> Preprocessor<'a> for ExpressionPreprocessor {
    type In = TkSlice<'a>;
    type Out = Box<[Tk<'a>]>;
    type Error = String;

    fn transform<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, String> {
        Ok(tokens.iter().map(Tk::clone).collect::<Vec<Tk<'b>>>().into_boxed_slice())
    }

}