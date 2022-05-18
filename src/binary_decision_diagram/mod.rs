pub mod binary_index;
pub mod node_handler;
use std::collections::HashSet;

use crate::{unwrap, utility::*};
pub use binary_index::*;
pub use node_handler::*;
use std::{ops::Deref, ptr};

type NodePtrMut<T> = *mut Node<T>;

#[derive(Clone, Copy, Hash, Debug, Eq)]
enum Link<T>
where
    T: Clone,
{
    Node(NodePtrMut<T>),
    Leaf(bool),
}

impl<T> PartialEq for Link<T>
where
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Node(l0), Self::Node(r0)) => l0 == r0,
            (Self::Leaf(l0), Self::Leaf(r0)) => l0 == r0,
            _ => false,
        }
    }
}

struct Node<T>
where
    T: Clone,
{
    variable: T,
    links: (Link<T>, Link<T>),
    parents: HashSet<NodePtrMut<T>>,
}

#[derive(Debug, Default)]
pub struct BinaryDecisionDiagram<T>
where
    T: Clone,
{
    roots: HashSet<NodePtrMut<T>>, // Maybe just for destruction use
    leaf_parents: (HashSet<NodePtrMut<T>>, HashSet<NodePtrMut<T>>),
}

// For basic operations
impl<T> BinaryDecisionDiagram<T>
where
    T: Clone,
{
    fn add_parent_for_node(&mut self, node: NodePtrMut<T>, parent: *const Node<T>) {
        if self.roots.contains(&node) {
            self.roots.remove(&node);
        }
        unsafe { &mut *node }
            .parents
            .insert(parent as NodePtrMut<T>);
    }

    fn add_node(
        &mut self,
        variable: T,
        children: (NodeHandler<T>, NodeHandler<T>),
    ) -> NodeHandler<T> {
        let new_node: NodePtrMut<T> = unsafe { allocate() };
        unsafe {
            std::ptr::write(
                new_node,
                Node {
                    variable,
                    links: (children.0 .0.clone(), children.1 .0.clone()),
                    parents: HashSet::default(),
                },
            );
        }
        self.roots.insert(new_node);
        if let NodeHandler(Link::Node(left)) = children.0 {
            self.add_parent_for_node(left, new_node);
        }
        if let NodeHandler(Link::Node(right)) = children.1 {
            self.add_parent_for_node(right, new_node);
        }
        NodeHandler(Link::Node(new_node))
    }
}

// For API implementation
impl<T> BinaryDecisionDiagram<T>
where
    T: Clone,
{
    pub fn add_node_if_necessary(
        &mut self,
        variable: T,
        children: (NodeHandler<T>, NodeHandler<T>),
    ) -> NodeHandler<T>
    where
        T: Eq,
    {
        if children.0 == children.1 {
            return children.0;
        }
        let common_parent_sets =
            HashSet::intersection(children.0.get_parents(self), children.1.get_parents(self));
        for node in common_parent_sets {
            let node = unsafe { &mut **node };
            if node.variable == variable {
                return NodeHandler(Link::Node(node));
            }
        }
        return self.add_node(variable, children);
    }

    pub fn get_leaf(value: bool) -> NodeHandler<T> {
        NodeHandler(Link::Leaf(value))
    }

    pub fn get_leaves() -> (NodeHandler<T>, NodeHandler<T>) {
        (Self::get_leaf(false), Self::get_leaf(true))
    }

    pub fn add_variable(&mut self, variable: T) -> NodeHandler<T>
    where
        T: Eq,
    {
        let leaves = Self::get_leaves();
        self.add_node_if_necessary(variable, leaves)
    }
}
