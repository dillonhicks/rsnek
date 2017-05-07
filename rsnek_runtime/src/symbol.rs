use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefMut, RefCell, Cell};
use std::collections::{HashSet, HashMap, VecDeque};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};

use rsnek_compile::fmt;

use ::error::Error;
use ::graph::{DiGraph, Graph, Node};
use ::opcode::OpCode;
use ::scope::ScopeHint::{BaseScope, ModuleScope, FunctionScope};
use ::scope::{ScopeNode, ScopeHint, ManageScope};
use ::objects::native::{self, Instr, Native};


pub trait TrackSymbol {
    fn use_symbol(&self, symbol: &Symbol) -> Result<(), Error>;
    fn define_symbol(&self, def: &Definition) -> Result<(), Error>;
}


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
pub struct Symbol(pub String);

#[derive(Clone, Debug, Serialize)]
pub struct Definition(pub String, pub Native);


impl Ord for Definition {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}


impl Hash for Definition {
    fn hash<H: Hasher>(&self, state: &mut H) where H: Hasher{
        self.0.hash(state)
    }
}


impl Eq for Definition {}


impl PartialEq for Definition {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}


impl PartialOrd for Definition {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}


impl<'a> TryFrom<&'a Native> for Symbol {
    type Error = Error;

    fn try_from(n: &Native) -> Result<Symbol, Error> {
        match n {
            &Native::Str(ref string) => Ok(Symbol(string.clone())),
            _ => Err(Error::system(&
                format!("Name types can only be created from Native::String variants, not {:?}; file: {}, line: {}", n, file!(), line!())))
        }
    }
}


impl<T> Serialize for SymIndex<T> where T: Serialize + Hash + Eq + Ord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.collect_seq(self.0.iter())
    }
}


#[derive(Debug, Clone)]
struct SymIndex<T>(HashMap<ScopeNode, RefCell<HashSet<T>>>) where T: Hash + Eq + Ord;


impl<T> SymIndex<T> where T: Hash + Eq + Ord {
    fn new () -> Self {
        SymIndex(HashMap::new())
    }

    fn get_or_create(&mut self, scope: &ScopeNode) -> RefMut<HashSet<T>> {
        if !self.0.contains_key(&scope) {
            self.0.insert(*scope, RefCell::new(HashSet::new()));
        }

        self.0[scope].borrow_mut()
    }
}

#[derive(Debug, Clone, Serialize)]
struct SymTable<T>(RefCell<SymIndex<T>>) where T: Hash + Eq + Ord;


impl<T> SymTable<T> where T: Clone + Hash + Eq + Ord {
    fn new() -> Self {
        SymTable(RefCell::new(SymIndex::new()))
    }

    fn index(&self) -> RefMut<SymIndex<T>> {
        self.0.borrow_mut()
    }

    fn add(&self, scope: &ScopeNode, value: &T) -> Result<(), Error> {
        let mut index: RefMut<SymIndex<T>> = self.index();
        let mut row: RefMut<HashSet<T>> = index.get_or_create(scope);
        row.insert((*value).clone());
        Ok(())
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct SymbolMetadata {
    graph: DiGraph<ScopeNode>,
    curr_scope_id: Cell<usize>,
    definitions: SymTable<Definition>,
    usages: SymTable<Symbol>
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

    pub fn graph(&self) -> &Graph<Node=ScopeNode> {
        &self.graph
    }
}

impl TrackSymbol for SymbolMetadata {
    fn define_symbol(&self, symbol: &Definition) -> Result<(), Error> {
        let scope = self.current_scope();
        trace!("SymbolMetadata";
            "action" => "define_symbol",
            "scope" => format!("{:?}", scope),
            "symbol" => format!("{:?}", symbol));

        self.definitions.add(&scope, symbol)
    }

    fn use_symbol(&self, symbol: &Symbol) -> Result<(), Error> {
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

