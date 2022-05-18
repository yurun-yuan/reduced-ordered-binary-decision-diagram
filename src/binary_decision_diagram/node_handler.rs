use std::{collections::HashMap, fmt::Display};

use crate::unwrap;

use super::{binary_index::*, BinaryDecisionDiagram, NodePtrMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeHandler<T>(pub(super) super::Link<T>)
where
    T: Clone;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Element<T> {
    Variable(T),
    Binary(bool),
}

impl<T> NodeHandler<T>
where
    T: Clone,
{
    pub fn get_child(&self, child_index: BinaryIndex) -> Option<NodeHandler<T>> {
        match &self.0 {
            super::Link::Node(node) => unsafe {
                Some(match child_index {
                    BinaryIndex::Left => NodeHandler((*(*node)).links.0.clone()),
                    BinaryIndex::Right => NodeHandler((*(*node)).links.1.clone()),
                })
            },
            super::Link::Leaf(_) => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        return matches!(self, Self(super::Link::Leaf(_)));
    }

    pub(super) fn get_parents<'a>(
        &self,
        diagram: &'a BinaryDecisionDiagram<T>,
    ) -> &'a std::collections::HashSet<NodePtrMut<T>> {
        match self.0 {
            super::Link::Node(node) => &unsafe { &mut *node }.parents,
            super::Link::Leaf(value) => match value {
                true => &diagram.leaf_parents.1,
                false => &diagram.leaf_parents.0,
            },
        }
    }

    pub fn get_element(&self) -> Element<&T> {
        match &self.0 {
            super::Link::Node(node) => Element::Variable(unsafe { &(*(*node)).variable }),
            super::Link::Leaf(value) => Element::Binary(*value),
        }
    }
}

pub struct FormulaRoot<T>(NodeHandler<usize>, HashMap<usize, T>);

impl<T> FormulaRoot<T> {
    pub fn new(
        node_handler: NodeHandler<usize>,
        inverse_table: HashMap<usize, T>,
    ) -> FormulaRoot<T> {
        FormulaRoot(node_handler, inverse_table)
    }
}

// For Display
// returns (index, flag)
// `flag` is true iff the index is generated by this function call
impl<T> FormulaRoot<T>
where
    T: Display,
{
    fn get_index(
        node_handler: &NodeHandler<usize>,
        inverse_table: &HashMap<usize, T>,
        f: &mut std::fmt::Formatter<'_>,
        index: &mut u32,
        visit_record: &mut HashMap<NodePtrMut<usize>, u32>,
    ) -> (u32, bool) {
        match node_handler.0 {
            super::Link::Node(node) => {
                if visit_record.contains_key(&node) {
                    (visit_record[&node], false)
                } else {
                    let var_value =
                        unwrap!(node_handler.get_element(), Element::Variable(var), var);
                    let var = inverse_table.get(var_value).unwrap().to_string();
                    writeln!(f, "{index} [label=\"{var}\"]").unwrap();
                    let node_index = *index;
                    visit_record.insert(node, node_index);
                    *index += 1;
                    (node_index, true)
                }
            }
            super::Link::Leaf(value) => (value as u32, false),
        }
    }

    pub fn generic_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "digraph{{")?;
        match self.0.get_element() {
            Element::Variable(_) => {
                writeln!(f, r#"0 [label="false"]"#)?;
                writeln!(f, r#"1 [label="true"]"#)?;
                let mut index = 2;
                Self::fmt_reclusive(&self.0, &self.1, f, &mut index, &mut HashMap::new())?;
            }
            Element::Binary(value) => match value {
                true => writeln!(f, r#"1 [label="true"]"#)?,
                false => writeln!(f, r#"0 [label="false"]"#)?,
            },
        }

        writeln!(f, "}}")?;
        Ok(())
    }

    fn fmt_reclusive(
        node_handler: &NodeHandler<usize>,
        inverse_table: &HashMap<usize, T>,
        f: &mut std::fmt::Formatter<'_>,
        index: &mut u32,
        visit_record: &mut HashMap<NodePtrMut<usize>, u32>,
    ) -> std::fmt::Result {
        match node_handler.get_element() {
            Element::Variable(_) => {
                let (parent_index, _) =
                    Self::get_index(node_handler, inverse_table, f, index, visit_record);
                let ((left_index, left_recursive_flag), (right_index, right_recursive_flag)) = (
                    Self::get_index(
                        &node_handler.get_child(BinaryIndex::Left).unwrap(),
                        inverse_table,
                        f,
                        index,
                        visit_record,
                    ),
                    Self::get_index(
                        &node_handler.get_child(BinaryIndex::Right).unwrap(),
                        inverse_table,
                        f,
                        index,
                        visit_record,
                    ),
                );
                writeln!(f, "{parent_index} -> {left_index} [label=\"0\"]")?;
                writeln!(f, "{parent_index} -> {right_index} [label=\"1\"]")?;
                if left_recursive_flag {
                    Self::fmt_reclusive(
                        &node_handler.get_child(BinaryIndex::Left).unwrap(),
                        inverse_table,
                        f,
                        index,
                        visit_record,
                    )?;
                }
                if right_recursive_flag {
                    Self::fmt_reclusive(
                        &node_handler.get_child(BinaryIndex::Right).unwrap(),
                        inverse_table,
                        f,
                        index,
                        visit_record,
                    )?;
                }
                Ok(())
            }
            Element::Binary(_) => Ok(()),
        }
    }
}

impl<T> Display for FormulaRoot<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.generic_fmt(f)
    }
}
