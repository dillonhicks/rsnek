//! See Grammar.txt for the python reference grammar.
use ::token::OwnedTk;


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
pub type BoxedExpr = Box<Expr>;


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
    Lambda {arguments: Vec<Expr>, body: Box<Expr>},
    Conditional {condition: Box<Expr>, consequent: Box<Expr>, alternative: Box<Expr>},
    BinOp { op: Op, left: BoxedExpr, right: BoxedExpr },
    Call { func: OwnedTk, args: Vec<Expr>,  keywords: ()},
    Attribute { value: Box<Expr>, attr: OwnedTk },
    List { elems: Vec<Expr> },
    NameConstant(OwnedTk),
    Constant(OwnedTk),
    None
}

impl Default for Expr {
    fn default() -> Self {
        Expr::None
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Op(pub OwnedTk);