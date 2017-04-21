//! See Grammar.txt for the python reference grammar.
use std::fmt;
use std::fmt::{Display, Debug, Formatter};

use std::str;
use std::str::FromStr;
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};

use lexer::Lexer;
use token::{Id, Tk, OwnedTk, pprint_tokens};
use slice::{TkSlice};

use nom::{IResult, digit, multispace};


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Ast {
    Module(Module),
    Statement(Stmt),
    Expression(Expr),
}

impl Default for Ast {
    fn default() -> Self {
        Ast::Module(Module::Body(Vec::new()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Module {
    Body(Vec<Stmt>)
}

/// Type alias for a boxed expression. The Expr enum needs this heap indirection to break
/// a recursive type definition that would otherwise result in a struct of infinite size.
///
pub type DynExpr = Box<Expr>;


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Stmt {
    FunctionDef { name: OwnedTk, arguments: Vec<OwnedTk>, body: Box<Stmt> },
    Assign { target: Expr, value: Expr},
    AugAssign { target: Expr, op: Op, value: Expr},
    Expr(Expr),
    Block(Vec<Stmt>), // TODO: Do blocks all share the same scope?
    Return(Option<Expr>),
    Newline,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Expr {
    BinOp { op: Op, left: DynExpr, right: DynExpr },
    NameConstant(Vec<OwnedTk>),
    Constant(OwnedTk)
}



#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Op(pub OwnedTk);