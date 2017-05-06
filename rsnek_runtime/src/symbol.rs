use std::convert::TryFrom;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefMut, RefCell, Cell};
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};

use rsnek_compile::{
    Ast, Module, Stmt, Expr, Op, Lexer,
    LexResult, Parser, ParserResult,
    OwnedTk, Id};

use rsnek_compile::fmt;
use ::error::Error;
use ::opcode::OpCode;
use ::typedef::native::{self, Instr, Native};


use ::graph::{DiGraph, Graph, Node};
use ::scope::{ScopeNode, ScopeHint, ManageScope};
use ::scope::ScopeHint::{BaseScope, ModuleScope, FunctionScope};


pub trait TrackSymbol {
    fn use_symbol(&self, symbol: &Native) -> Result<(), Error>;
    fn define_symbol(&self, symbol: &Native) -> Result<(), Error>;
}


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
struct Name(pub String);


impl<'a> TryFrom<&'a Native> for Name {
    type Error = Error;

    fn try_from(n: &Native) -> Result<Name, Error> {
        match n {
            &Native::Str(ref string) => Ok(Name(string.clone())),
            _ => Err(Error::system(&
                format!("Name types can only be created from Native::String variants, not {:?}; file: {}, line: {}", n, file!(), line!())))
        }
    }
}

type NameSet = HashSet<Name>;


impl Serialize for SymIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.collect_seq(self.0.iter())
    }
}


#[derive(Debug, Clone)]
struct SymIndex(HashMap<ScopeNode, RefCell<NameSet>>);

impl SymIndex {
    fn new () -> Self {
        SymIndex(HashMap::new())
    }

    fn get_or_create(&mut self, scope: &ScopeNode) -> RefMut<NameSet> {
        if !self.0.contains_key(&scope) {
            self.0.insert(*scope, RefCell::new(HashSet::new()));
        }

        self.0[scope].borrow_mut()
    }
}

#[derive(Debug, Clone, Serialize)]
struct SymTable(RefCell<SymIndex>);


impl SymTable {
    fn new() -> Self {
        SymTable(RefCell::new(SymIndex::new()))
    }
    fn index(&self) -> RefMut<SymIndex> {
        self.0.borrow_mut()
    }

    fn add(&self, scope: &ScopeNode, name: &Native) -> Result<(), Error> {
        let mut index: RefMut<SymIndex> = self.index();
        let mut row: RefMut<NameSet> = index.get_or_create(scope);
        row.insert(Name::try_from(name)?);
        Ok(())
    }
}



#[derive(Debug, Clone, Serialize)]
pub struct SymbolMetadata {
    graph: DiGraph<ScopeNode>,
    curr_scope_id: Cell<usize>,
    definitions: SymTable,
    usages: SymTable
}



impl SymbolMetadata {
    pub fn new() -> Self {
        SymbolMetadata {
            graph: DiGraph::new(ScopeNode::new(0, 0, BaseScope)),
            curr_scope_id: Cell::new(0),
            definitions: SymTable::new(),
            usages: SymTable::new()
        }
    }

}

impl TrackSymbol for SymbolMetadata {
    fn define_symbol(&self, symbol: &Native) -> Result<(), Error> {
        let scope = self.current_scope();
        trace!("SymbolMetadata";
        "action" => "define_symbol",
        "scope" => format!("{:?}", scope),
        "symbol" => format!("{:?}", symbol));

        self.definitions.add(&scope, symbol)
    }

    fn use_symbol(&self, symbol: &Native) -> Result<(), Error> {
        let scope = self.current_scope();
        trace!("SymbolMetadata";
        "action" => "add_usage",
        "scope" => format!("{:?}", scope),
        "symbol" => format!("{:?}", symbol));

        self.usages.add(&scope, symbol)
    }
}


impl ManageScope for SymbolMetadata {

    fn current_scope(&self) -> Box<ScopeNode> {
        self.graph.get_node(self.curr_scope_id.get())
    }

    fn enter_scope(&self, hint: ScopeHint) {
        let parent = self.current_scope();

        let new_scope = ScopeNode::new(parent.id(), self.graph.count(), hint);

        trace!("SymbolMetadata";
        "action" => "enter_scope",
        "scope_hint" => format!("{:?}", hint),
        "prev_scope" => format!("{:?}", parent),
        "next_scope" => format!("{:?}", new_scope));

        self.curr_scope_id.set(new_scope.id());
        self.graph.add_node(new_scope)
    }

    fn exit_scope<T>(&self, result: T) -> T {
        let prev_scope = self.current_scope();
        self.curr_scope_id.set(prev_scope.parent_id());
        let next_scope = self.current_scope();

        trace!("SymbolMetadata";
        "action" => "exit_scope",
        "prev_scope" => format!("{:?}", prev_scope),
        "next_scope" => format!("{:?}", next_scope));
        result
    }
}

