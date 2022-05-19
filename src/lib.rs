//! This project aims to generate a Reduced Ordered Binary Decision Diagram from a text-based PL formula.

mod binary_decision_diagram;
mod utility;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};
mod formula_parser;
use binary_decision_diagram::*;
use formula_parser::ParserNode;

#[derive(Clone, Copy)]
pub enum Operation {
    Binary(BinaryOperation),
    Unary(UnaryOperation),
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperation {
    And,
    Or,
    Implication,
    Equivalence,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperation {
    Not,
}

pub type LexerError<'a> =
    lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'static str>;
pub use binary_decision_diagram::node_handler::FormulaRoot;

pub fn construct_robdd<'a>(
    input: &'a str,
) -> Result<(BinaryDecisionDiagram<usize>, FormulaRoot<String>), LexerError> {
    let mut diagram = BinaryDecisionDiagram::default();
    let mut inverse_table = vec![];
    let root = construct_robdd_from_parser_tree(
        &rename_variable(
            &formula_parser::formula_parse(input)?,
            &mut HashMap::default(),
            &mut inverse_table,
        ),
        &mut diagram,
    );
    Ok((
        diagram,
        FormulaRoot::new(root, inverse_table.into_iter().enumerate().collect()),
    ))
}

fn construct_robdd_from_parser_tree(
    input: &ParserNode<usize>,
    diagram: &mut BinaryDecisionDiagram<usize>,
) -> NodeHandler<usize> {
    match input {
        ParserNode::Unary(op, operand) => match op {
            UnaryOperation::Not => {
                let operand = construct_robdd_from_parser_tree(operand, diagram);
                apply_unary(diagram, operand, *op)
            }
        },
        ParserNode::Binary(op, (left, right)) => {
            let left = construct_robdd_from_parser_tree(left, diagram);
            let right = construct_robdd_from_parser_tree(right, diagram);
            apply_binary(diagram, (left, right), *op)
        }
        ParserNode::Variable(var) => diagram.add_variable(*var),
        ParserNode::Leaf(value) => BinaryDecisionDiagram::get_leaf(*value),
    }
}

#[test]
fn construct_test() {
    println!(
        "{}",
        construct_robdd("(!x1 | x2) & (x1 | !x3) & (!x1 | !x2 | x3)")
            .unwrap()
            .1
    );
}

fn rename_variable<From>(
    input: &ParserNode<From>,
    symbol_table: &mut HashMap<From, usize>,
    inverse_table: &mut Vec<From>,
) -> ParserNode<usize>
where
    From: Eq + Hash + Clone,
{
    match input {
        ParserNode::Unary(op, operand) => ParserNode::Unary(
            *op,
            Box::new(rename_variable(operand, symbol_table, inverse_table)),
        ),
        ParserNode::Binary(op, (left, right)) => ParserNode::Binary(
            *op,
            (
                Box::new(rename_variable(left, symbol_table, inverse_table)),
                Box::new(rename_variable(right, symbol_table, inverse_table)),
            ),
        ),
        ParserNode::Variable(var) => {
            if !symbol_table.contains_key(var) {
                symbol_table.insert(var.clone(), inverse_table.len());
                inverse_table.push(var.clone());
            }
            ParserNode::Variable(symbol_table[var])
        }
        ParserNode::Leaf(value) => ParserNode::Leaf(*value),
    }
}

fn apply_binary<T>(
    diagram: &mut BinaryDecisionDiagram<T>,
    operands: (NodeHandler<T>, NodeHandler<T>),
    operation: BinaryOperation,
) -> NodeHandler<T>
where
    T: Clone + Eq + Ord + Copy,
{
    let elements = (operands.0.get_element(), operands.1.get_element());

    // Basic case
    if let Element::Binary(value) = elements.0 {
        match value {
            true => match operation {
                BinaryOperation::And => return operands.1,
                BinaryOperation::Or => return BinaryDecisionDiagram::get_leaf(true),
                BinaryOperation::Implication => return operands.1,
                BinaryOperation::Equivalence => return operands.1,
            },
            false => match operation {
                BinaryOperation::And => return BinaryDecisionDiagram::get_leaf(false),
                BinaryOperation::Or => return operands.1,
                BinaryOperation::Implication => return BinaryDecisionDiagram::get_leaf(true),
                BinaryOperation::Equivalence => {
                    return apply_unary(diagram, operands.1, UnaryOperation::Not)
                }
            },
        }
    }
    if let Element::Binary(value) = elements.1 {
        match value {
            true => match operation {
                BinaryOperation::And => return operands.0,
                BinaryOperation::Or => return BinaryDecisionDiagram::get_leaf(true),
                BinaryOperation::Implication => return BinaryDecisionDiagram::get_leaf(true),
                BinaryOperation::Equivalence => return operands.0,
            },
            false => match operation {
                BinaryOperation::And => return BinaryDecisionDiagram::get_leaf(false),
                BinaryOperation::Or => return operands.0,
                BinaryOperation::Implication => {
                    return apply_unary(diagram, operands.0, UnaryOperation::Not)
                }
                BinaryOperation::Equivalence => {
                    return apply_unary(diagram, operands.0, UnaryOperation::Not)
                }
            },
        }
    }

    // Case 1: both have smallest variable
    if Ord::cmp(&elements.0, &elements.1) == Ordering::Equal {
        let (t1, t2, u1, u2) = (
            operands.0.get_child(BinaryIndex::Left).unwrap(),
            operands.0.get_child(BinaryIndex::Right).unwrap(),
            operands.1.get_child(BinaryIndex::Left).unwrap(),
            operands.1.get_child(BinaryIndex::Right).unwrap(),
        );
        let new_children = (
            apply_binary(diagram, (t1, u1), operation),
            apply_binary(diagram, (t2, u2), operation),
        );
        return diagram
            .add_node_if_necessary(unwrap!(elements.0, Element::Variable(v), *v), new_children);
    }

    // Case 2: the smallest variable only appears on one side
    let (smaller, larger) = match Ord::cmp(&elements.0, &elements.1) {
        Ordering::Less => (operands.0, operands.1),
        Ordering::Greater => (operands.1, operands.0),
        _ => panic!(),
    };

    let (t1, t2, u) = (
        smaller.get_child(BinaryIndex::Left).unwrap(),
        smaller.get_child(BinaryIndex::Right).unwrap(),
        larger,
    );
    let new_children = (
        apply_binary(diagram, (t1, u), operation),
        apply_binary(diagram, (t2, u), operation),
    );

    return diagram.add_node_if_necessary(
        unwrap!(smaller.get_element(), Element::Variable(v), *v),
        new_children,
    );
}
fn apply_unary<T>(
    diagram: &mut BinaryDecisionDiagram<T>,
    operand: NodeHandler<T>,
    operation: UnaryOperation,
) -> NodeHandler<T>
where
    T: Clone + Eq + Ord,
{
    match operation {
        UnaryOperation::Not => match operand.get_element() {
            Element::Variable(var) => {
                let children = (
                    operand.get_child(BinaryIndex::Left).unwrap(),
                    operand.get_child(BinaryIndex::Right).unwrap(),
                );
                let not_children = (
                    apply_unary(diagram, children.0, operation),
                    apply_unary(diagram, children.1, operation),
                );
                diagram.add_node_if_necessary(var.clone(), not_children)
            }
            Element::Binary(value) => match value {
                true => BinaryDecisionDiagram::<T>::get_leaf(false),
                false => BinaryDecisionDiagram::<T>::get_leaf(true),
            },
        },
    }
}
