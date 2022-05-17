mod binary_decision_diagram;
mod utility;
use std::cmp::Ordering;

use binary_decision_diagram::*;

#[derive(Clone, Copy)]
enum BinaryOperation {
    And,
    Or,
    Implication,
    Equivalence,
}

enum UnaryOperation {
    Not,
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
        return diagram.add_node_if_necessary(
            unwrap!(elements.0, Element::Variable(v), v.clone()),
            new_children,
        );
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
        unwrap!(smaller.get_element(), Element::Variable(v), v.clone()),
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
                diagram.add_node_if_necessary(var.clone(), (children.1, children.0))
            }
            Element::Binary(value) => match value {
                true => BinaryDecisionDiagram::<T>::get_leaf(false),
                false => BinaryDecisionDiagram::<T>::get_leaf(true),
            },
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_case() {
        let mut diagram = BinaryDecisionDiagram::default();
        let first = BinaryDecisionDiagram::<u32>::get_leaf(false);
        let second = BinaryDecisionDiagram::<u32>::get_leaf(true);
        println!(
            "{:?}",
            apply_binary(&mut diagram, (first, second), BinaryOperation::Implication)
        );
    }
    #[test]
    fn slide_example_test() {
        let mut diagram = BinaryDecisionDiagram::<u32>::default();
        let p = diagram.add_variable(1);
        let q = diagram.add_variable(2);
        let r = diagram.add_variable(3);
        let r_or_q = apply_binary(&mut diagram, (r, p), BinaryOperation::Or);
        let q_eq_r_or_q = apply_binary(&mut diagram, (q, r_or_q), BinaryOperation::Equivalence);
        let p_impl_r = apply_binary(&mut diagram, (p, r), BinaryOperation::Implication);
        println!(
            "{}",
            apply_binary(&mut diagram, (p_impl_r, q_eq_r_or_q), BinaryOperation::And)
        )
    }
}
