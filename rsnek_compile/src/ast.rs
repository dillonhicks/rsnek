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
pub enum FnType {
    Async,
    Sync,
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Stmt {
    FunctionDef { fntype: FnType, name: OwnedTk, arguments: Vec<Expr>, body: Box<Stmt> },
    Block(Vec<Stmt>),
    ClassDef {name: OwnedTk, bases: Vec<Expr>, body: Box<Stmt> },
    Return(Option<Expr>),
    Delete(Vec<Expr>),
    Assign { target: Expr, value: Expr},
    AugAssign { target: Expr, op: Op, value: Expr},
//    | AnnAssign(expr target, expr annotation, expr? value, int simple)
//
//    -- use 'orelse' because else is a keyword in target languages
//    | For(expr target, expr iter, stmt* body, stmt* orelse)
//    | AsyncFor(expr target, expr iter, stmt* body, stmt* orelse)
//    | While(expr test, stmt* body, stmt* orelse)
//    | If(expr test, stmt* body, stmt* orelse)
//    | With(withitem* items, stmt* body)
//    | AsyncWith(withitem* items, stmt* body)
//
//    | Raise(expr? exc, expr? cause)
//    | Try(stmt* body, excepthandler* handlers, stmt* orelse, stmt* finalbody)
//    | Assert(expr test, expr? msg)

    Import, //(alias* names)
    ImportFrom, // (identifier? module, alias* names, int? level))
    Global(Vec<OwnedTk>),
    Nonlocal(Vec<OwnedTk>),
    Expr(Expr),
    Pass,
    Break,
    Continue,
    Newline,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Expr {
    BinOp { op: Op, left: DynExpr, right: DynExpr },
    Call { func: OwnedTk, args: Vec<Expr>,  keywords: ()},
    NameConstant(Vec<OwnedTk>),
    Constant(OwnedTk)
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Op(pub OwnedTk);