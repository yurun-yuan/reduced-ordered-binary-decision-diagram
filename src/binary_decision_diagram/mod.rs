pub mod child_index;
pub mod node_handler;

use crate::{unwrap, utility::*};
pub use child_index::*;
pub use node_handler::*;
use std::{ops::Deref, ptr};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Link<T>
where
    T: Clone,
{
    Node(*mut Node<T>),
    Leaf(bool),
}

struct Node<T>
where
    T: Clone,
{
    variable: T,
    links: [Link<T>; 2],
    parent: *mut Node<T>,
}

#[derive(Debug)]
pub struct BinaryDecisionDiagram<T>
where
    T: Clone,
{
    root: Link<T>,
}

impl<T> BinaryDecisionDiagram<T>
where
    T: Clone,
{
    pub fn new_single_variable(variable: T) -> BinaryDecisionDiagram<T> {
        let new_node = unsafe { allocate() };
        unsafe {
            *new_node = Node {
                variable,
                links: [Link::Leaf(false), Link::Leaf(true)],
                parent: ptr::null_mut(),
            };
        }
        BinaryDecisionDiagram {
            root: Link::Node(new_node),
        }
    }

    pub fn new_from_subtrees(
        root_value: T,
        subtrees: (BinaryDecisionDiagram<T>, BinaryDecisionDiagram<T>),
    ) -> BinaryDecisionDiagram<T> {
        let new_node = unsafe { allocate() };
        unsafe {
            *new_node = Node {
                variable: root_value,
                links: [subtrees.0.take_root(), subtrees.1.take_root()],
                parent: ptr::null_mut(),
            };
        }
        BinaryDecisionDiagram {
            root: Link::Node(new_node),
        }
    }

    pub fn new_leaf(value: bool) -> BinaryDecisionDiagram<T> {
        BinaryDecisionDiagram {
            root: Link::Leaf(value),
        }
    }

    pub fn split(self) -> Option<(T, BinaryDecisionDiagram<T>, BinaryDecisionDiagram<T>)> {
        if self.get_root().is_leaf() {
            return None;
        }
        let root = self.take_root();
        let node_ptr = unwrap!(root, Link::Node(node_ptr), node_ptr);
        let node;
        unsafe {
            node = ptr::read(node_ptr);
            deallocate(node_ptr);
        }

        let variable = node.variable;
        Some((
            variable.clone(),
            BinaryDecisionDiagram {
                root: node.links[0].clone(),
            },
            BinaryDecisionDiagram {
                root: node.links[1].clone(),
            },
        ))
    }

    fn take_root(mut self) -> Link<T> {
        let root = self.get_root().link;
        self.root = Link::Leaf(true); // To avoid the drop reclaiming the diagram
        root
    }

    pub fn get_root(&self) -> NodeHandler<T> {
        NodeHandler {
            link: self.root.clone(),
        }
    }
}

impl<T> From<bool> for BinaryDecisionDiagram<T>
where
    T: Clone,
{
    fn from(value: bool) -> Self {
        BinaryDecisionDiagram {
            root: Link::Leaf(value),
        }
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
