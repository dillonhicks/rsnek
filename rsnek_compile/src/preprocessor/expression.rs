use std::convert::TryFrom;
use std::iter::Iterator;
use std::slice::Iter;
use std::collections::VecDeque;

use slog;
use slog_scope;

use itertools::Itertools;
use nom::Slice;

use ::token::{Id, Tk, Tag};
use ::slice::{TkSlice};
use ::preprocessor::Preprocessor;


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


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Precedent  {
}

const P_MAX: usize = 17;

impl Precedent {
    pub const Lambda: usize = P_MAX -  1;
    pub const If: usize = P_MAX - 2;
    pub const LogicalOr: usize = P_MAX - 3;
    pub const LogicalAnd: usize = P_MAX - 4;
    pub const LogicalNot: usize = P_MAX - 5;
    pub const In: usize = P_MAX - 6;

    pub const NotIn: usize = P_MAX - 6;
    pub const Is: usize = P_MAX - 6;
    pub const IsNot: usize = P_MAX - 6;
    pub const Less: usize = P_MAX - 6;
    pub const LessOrEqual: usize = P_MAX - 6;
    pub const Greater: usize = P_MAX - 6;
    pub const GreaterOrEqual: usize = P_MAX - 6;
    pub const NotEqual: usize = P_MAX - 6;
    pub const Equal: usize = P_MAX - 6;

    pub const BitwiseOr: usize = P_MAX - 7;
    pub const BitwiseXor: usize = P_MAX - 8;
    pub const BitwiseAnd: usize = P_MAX - 9;
    pub const LeftShift: usize = P_MAX - 10;
    pub const RightShift: usize = P_MAX - 10;
    pub const Add: usize = P_MAX - 11;
    pub const Subtract: usize = P_MAX - 11;
    pub const Multiply: usize = P_MAX - 12;
    pub const MatrixMultiply: usize = P_MAX - 12;
    pub const TrueDivision: usize = P_MAX - 12;
    pub const FloorDivision: usize = P_MAX - 12;
    pub const Modulus: usize = P_MAX - 12;
    pub const Positive: usize = P_MAX - 13;
    pub const Negate: usize = P_MAX - 13;
    pub const Invert: usize = P_MAX - 13;
    pub const Power: usize = P_MAX - 14;
    // Others

    pub const GetAttr: usize = P_MAX - 16;

    fn try_from(id: Id) -> Result<usize, ()>{
        let precedent = match id {
            Id::DoubleEqual => Precedent::Equal,
            Id::NotEqual => Precedent::NotEqual,
            Id::LessOrEqual => Precedent::LessOrEqual,
            Id::LeftShift => Precedent::LeftShift,
            Id::GreaterOrEqual => Precedent::GreaterOrEqual,
            Id::RightShift => Precedent::RightShift,
            Id::DoubleStar => Precedent::Power,
            Id::DoubleSlash => Precedent::FloorDivision,
            Id::Plus => Precedent::Add,
            Id::Minus => Precedent::Subtract,
            Id::Star => Precedent::Multiply,
            Id::Slash => Precedent::TrueDivision,
            Id::Pipe => Precedent::BitwiseOr,
            Id::Amp => Precedent::BitwiseAnd,
            Id::LeftAngle => Precedent::Less,
            Id::RightAngle => Precedent::Greater,
            Id::Percent => Precedent::Modulus,
            Id::Caret => Precedent::BitwiseXor,
            Id::Tilde => Precedent::Invert,
            Id::At => Precedent::MatrixMultiply,
            Id::Dot => Precedent::GetAttr,
            _ => return Err(())
        };

        Ok(precedent)
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
#[derive(Debug, Clone, Serialize)]
pub struct ExpressionPreprocessor {
    #[serde(skip_serializing)]
    log: slog::Logger
}


fn is_operator(id: Id) -> bool {
    match Precedent::try_from(id) {
        Ok(_) => true,
        Err(_) => false
    }
}

//lambda	Lambda expression
//if – else	Conditional expression
//or	Boolean OR
//and	Boolean AND
//not x	Boolean NOT
//in, not in, is, is not, <, <=, >, >=, !=, ==	Comparisons, including membership tests and identity tests
//|	Bitwise OR
//^	Bitwise XOR
//&	Bitwise AND
//<<, >>	Shifts
//+, -	Addition and subtraction
//*, @, /, //, %	Multiplication, matrix multiplication division, remainder [5]
//+x, -x, ~x	Positive, negative, bitwise NOT
//**	Exponentiation [6]
//await x	Await expression
//x[index], x[index:index], x(arguments...), x.attribute	Subscription, slicing, call, attribute reference
//(expressions...), [expressions...], {key: value...}, {expressions...}	Binding or tuple display, list display, dictionary display, set display
//const VALUE_MARKER = 1;
const TK_NONE: Tk = Tk::const_(Id::None, &[], Tag::None);
const NONE: &'static[Tk] = &[TK_NONE];

pub enum Error {
    UnexpectedToken,
}

use ast::{Expr, Stmt};

impl ExpressionPreprocessor {
    pub const NAME: &'static str = "ExprScopePreprocessor";

    pub fn new() -> Self {
        ExpressionPreprocessor {
            log: slog_scope::logger().new(slog_o!())
        }
    }

    fn sub_expr_tuple<'b>(&self, tokens: TkSlice<'b>) -> Result<Expr, Error> {
        let mut iter: &mut Iterator<Item=&Tk<'b>> = &mut tokens.iter();
        let mut stack: Vec<Tk<'b>> = Vec::new();
        let mut skip_until: usize = 0;

        let mut shifted_tokens: &mut Iterator<Item=&Tk<'b>> = &mut tokens.iter();
        shifted_tokens.next();
        let lookahead = shifted_tokens.chain(NONE); // Add end marker

        match iter.next() {
            Some(tk) if tk.id() == Id::LeftParen => stack.push(tk.clone()),
            _ => return Err(Error::UnexpectedToken)
        }

        for (idx, (tk, tk1)) in iter.zip(lookahead).enumerate() {
            if idx < skip_until {
                continue
            }

            let result = match tk.id() {
                Id::LeftParen => self.sub_expr_lparen_start(tokens.slice(idx..)),
                Id::RightParen |
                _ => {
                    Err(Error::UnexpectedToken)
                }

            };
        }

        Err(Error::UnexpectedToken)
    }

    fn sub_expr_lparen_start<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, Error> {
        Err(Error::UnexpectedToken)

    }
    //
//    tk_method!(start_expr, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
//        expression: alt_complete!(
//            call_m!(self.sub_expr_binop)                |
//            call_m!(self.sub_expr_call)                 |
//            call_m!(self.sub_expr_nameconstant)         |
//            call_m!(self.sub_expr_constant)             ) >>
//
//        (expression)
//    ));


}


impl<'a> Preprocessor<'a> for ExpressionPreprocessor {
    type In = TkSlice<'a>;
    type Out = Box<[Tk<'a>]>;
    type Error = Error;

    fn transform<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, Error> {
        let mut output: Vec<Tk<'b>> = Vec::with_capacity(tokens.len());
        let mut stack: VecDeque<Tk<'b>> = VecDeque::with_capacity(tokens.len() / 2);

        // - Create iterator over tokens
        // - Consume 1 token to effectively shift the tokens by 1
        // - Append a slice of NONE to the end to ensure tokens.len() == lookahead.len()
        //   so zip will not stop producing early.
        let mut shifted_tokens: &mut Iterator<Item=&Tk<'b>> = &mut tokens.iter();
        shifted_tokens.next();
        let lookahead = shifted_tokens.chain(NONE); // Add end marker

       tokens.iter()
           .zip(lookahead)
           .enumerate()
           .map(|(idx, (tk, tk1))| {

            match tk.id() {
                Id::LeftParen => {
                    match tk1.id() {
                        _ => {}
                    }
                },
                _ => {}
            }
        });

        Err(Error::UnexpectedToken)
    }

}