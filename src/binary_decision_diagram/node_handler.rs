use super::child_index::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct NodeHandler<T>
where
    T: Clone,
{
    pub(super) link: super::Link<T>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Element<T> {
    Variable(T),
    Binary(bool),
}

impl<T> NodeHandler<T>
where
    T: Clone,
{
    pub fn get_child(&self, child_index: ChildIndex) -> Option<NodeHandler<T>> {
        match &self.link {
            super::Link::Node(node) => unsafe {
                Some(NodeHandler {
                    link: (*(*node)).links[usize::from(child_index)].clone(),
                })
            },
            super::Link::Leaf(_) => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        return matches!(self.link, super::Link::Leaf(_));
    }

    pub fn get_element(&self) -> Element<&T> {
        match &self.link {
            super::Link::Node(node) => Element::Variable(unsafe { &(*(*node)).variable }),
            super::Link::Leaf(value) => Element::Binary(*value),
        }
    }
}
