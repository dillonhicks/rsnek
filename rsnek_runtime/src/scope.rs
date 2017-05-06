use ::graph::Node;


pub trait ManageScope {
    fn current_scope(&self) -> Box<ScopeNode>;
    fn enter_scope(&self, ScopeHint);
    fn exit_scope<T>(&self, T) -> T;
}

pub type ScopeNode = Descriptor<Scope>;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Serialize)]
pub enum ScopeHint {
    BaseScope,
    ModuleScope,
    FunctionScope
}


#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Serialize)]
pub struct Scope {
    id: usize,
    parent_id: usize
}

impl Node for Scope {
    fn id(&self) -> usize {
        self.id
    }

    fn parent_id(&self) -> usize {
        self.parent_id
    }
}


#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Serialize)]
pub enum Descriptor<T> {
    Base(T),
    Module(T),
    Function(T)
}


impl Descriptor<Scope> {
    pub fn new(parent_id: usize, id: usize, hint: ScopeHint) -> Self {
        let node = Scope {
            id: id,
            parent_id: parent_id
        };

        match hint {
            ScopeHint::BaseScope => Descriptor::Base(node),
            ScopeHint::ModuleScope => Descriptor::Module(node),
            ScopeHint::FunctionScope  => Descriptor::Function(node)
        }
    }

    fn node(&self) -> &Scope {
        match self {
            &Descriptor::Base(ref node)      |
            &Descriptor::Module(ref node)    |
            &Descriptor::Function(ref node)  => node
        }
    }
}


impl Node for Descriptor<Scope> {

    fn id(&self) -> usize {
        self.node().id()
    }

    fn parent_id(&self) -> usize {
        self.node().parent_id()
    }
}

