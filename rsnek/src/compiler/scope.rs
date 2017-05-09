//! Accounting for compile type discovered scopes
use ::compiler::graph::Node;


pub trait ManageScope {
    fn current_scope(&self) -> Box<ScopeNode>;
    fn enter_scope(&self, ScopeHint);
    fn exit_scope<T>(&self, T) -> T;
}

pub type ScopeNode = Descriptor<Scope>;

/// A way to reference a `Descriptor<T>` variant without knowing the inner
/// data of the `Descriptor<T>`.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Serialize)]
pub enum ScopeHint {
    BaseScope,
    ModuleScope,
    FunctionScope
}

/// Define the scope by its a unique id of all scopes within a module
/// and the id of its parent. It is an implementation detail that the
/// scope ids are numbered by incrementing the number of total scopes
/// discovered in a DFS search of scopes.
///
/// # Examples
/// ```python
/// # this is a module and it has an implicit definition
/// # Scope {id: 1, parent_id: 0}
///
/// def some_func():
///     # This scope will be defined by
///     # Scope {id: 2, parent_id: 1}
///
///     def nested():
///         # It follows that the Scope for this function will be
///         # Scope {id: 3, parent_id: 2}
///         pass
///     pass
///
/// def back_to_module_level():
///     # This Scope will be defined as
///     # Scope {id: 4, parent_id: 1}
///     pass
/// ```
///
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


/// Generic Descriptor enum
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
