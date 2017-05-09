//! Simple directed graph implementation with storing nodes as adjacency lists
//! in order to trace scopes as compilation happens.
use std::cell::RefCell;

use serde::Serialize;

/// Trait to define a node with link back to its parent
pub trait Node {
    fn id(&self) -> usize;
    fn parent_id(&self) -> usize;
}

/// Minimal graph trait that only requires access to the `count()` of all
/// nodes. `add_node()` to the graph when it already defines it's id and the
/// id of its parent, and to `get_node()` using it's Id.
///
pub trait Graph {
    type Node: Node + ?Sized + Serialize;

    /// The number of nodes in the `Graph`.
    fn count(&self) -> usize;

    /// Insert a node into the graph when it already defines its id and the id of its
    /// parent.
    fn add_node(&self, Self::Node);

    /// Get a node by id.
    ///
    fn get_node(&self, usize) -> Box<Self::Node>;
}


/// Adjacency List Style Directed Graph
///
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Serialize)]
pub struct DiGraph<T: Node + ?Sized + Serialize> {
    nodes: RefCell<Vec<Box<T>>>
}


impl<T> Graph for DiGraph<T> where T: Node + Clone + Serialize{
    type Node = T;

    fn count(&self) -> usize {
        self.nodes.borrow().len()
    }

    fn add_node(&self, node: Self::Node) {
        self.nodes.borrow_mut().insert(node.id(), Box::new(node))
    }

    fn get_node(&self, id: usize) -> Box<Self::Node> {
        let node: &Box<Self::Node> = &self.nodes.borrow()[id];
        node.clone()
    }

}


impl<T> DiGraph<T> where T: Node + Clone + Serialize {

    pub fn new(root: T) -> Self {
        let mut nodes: Vec<Box<T>> = Vec::new();
        nodes.push(Box::new(root));

        DiGraph {
            nodes: RefCell::new(nodes),
        }
    }

}