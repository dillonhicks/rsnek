//! Simple directed graph implementation with storing nodes as adjacency lists
//! in order to trace scopes as compilation happens.
use std::cell::RefCell;

use serde::Serialize;


pub trait Node {
    fn id(&self) -> usize;
    fn parent_id(&self) -> usize;
}


pub trait Graph {
    type Node: Node + ?Sized + Serialize;

    fn count(&self) -> usize;
    fn add_node(&self, Self::Node);
    fn get_node(&self, usize) -> Box<Self::Node>;
}


/// Adjacency List Style Directed Graph
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
        let mut adjacency_list: Vec<Box<T>> = Vec::new();
        adjacency_list.push(Box::new(root));

        DiGraph {
            nodes: RefCell::new(adjacency_list),
        }
    }

}