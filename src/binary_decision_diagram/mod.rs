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
    fn remove_parent_for_node(&mut self, node_ptr: NodePtrMut<T>, parent: *const Node<T>) {
        let node = unsafe { &mut *node_ptr };
        node.parents.remove(&(parent as NodePtrMut<T>));
        if node.parents.is_empty() {
            self.roots.insert(node_ptr);
        }
    }

    pub fn add_node(
        &mut self,
        variable: T,
        children: (NodeHandler<T>, NodeHandler<T>),
    ) -> NodeHandler<T> {
        let new_node: NodePtrMut<T> = unsafe { allocate() };
        *unsafe { &mut *new_node } = Node {
            variable,
            links: (children.0 .0.clone(), children.1 .0.clone()),
            parents: Default::default(),
        };
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
    pub fn alter_link(
        &mut self,
        parent_ptr: NodeHandler<T>,
        child_index: BinaryIndex,
        child_ptr: NodeHandler<T>,
    ) {
        let parent = match parent_ptr.0 {
            Link::Node(node) => unsafe { &mut *node },
            Link::Leaf(_) => panic!(),
        };
        let link: &mut Link<T> = match child_index {
            BinaryIndex::Left => &mut parent.links.0,
            BinaryIndex::Right => &mut parent.links.1,
        };

        let previous_link = link.clone();
        *link = child_ptr.0.clone();

        // remove previous relation
        match previous_link {
            Link::Node(node) => self.remove_parent_for_node(node, parent),
            Link::Leaf(value) => {
                match value {
                    true => self.leaf_parents.1.remove(&(parent as NodePtrMut<T>)),
                    false => self.leaf_parents.0.remove(&(parent as NodePtrMut<T>)),
                };
            }
        };

        // add new relation
        match child_ptr.0 {
            Link::Node(node) => self.add_parent_for_node(node, parent),
            Link::Leaf(value) => {
                match value {
                    true => self.leaf_parents.1.insert(parent as NodePtrMut<T>),
                    false => self.leaf_parents.0.insert(parent as NodePtrMut<T>),
                };
            }
        };
    }

    pub fn add_node_if_necessary(
        &mut self,
        variable: T,
        children: (NodeHandler<T>, NodeHandler<T>),
    ) -> NodeHandler<T>
    where
        T: Eq,
    {
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

    pub fn remove_node(&mut self, node: NodeHandler<T>) {
        if let NodeHandler(Link::Node(node)) = node {
            let node = unsafe { &mut *node };
            if !node.parents.is_empty() {
                panic!()
            }
            match node.links.0 {
                Link::Node(left) => self.remove_parent_for_node(unsafe { &mut *left }, node),
                Link::Leaf(value) => {
                    match value {
                        true => self.leaf_parents.1.remove(&(node as *mut Node<T>)),
                        false => self.leaf_parents.0.remove(&(node as *mut Node<T>)),
                    };
                }
            }
            match node.links.1 {
                Link::Node(right) => self.remove_parent_for_node(unsafe { &mut *right }, node),
                Link::Leaf(value) => {
                    match value {
                        true => self.leaf_parents.1.remove(&(node as *mut Node<T>)),
                        false => self.leaf_parents.0.remove(&(node as *mut Node<T>)),
                    };
                }
            }
            self.roots.remove(&(node as *mut Node<T>));
        } else {
            panic!()
        }
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

// impl<T> Drop for BinaryDecisionDiagram<T>
// where
//     T: Clone,
// {
//     fn drop(&mut self) {
//         todo!()
//     }
// }
