mod binary_decision_diagram;
mod utility;
use binary_decision_diagram::*;

#[derive(Clone, Copy)]
enum Operation {
    And,
    Or,
    Implication,
    Equivalence,
}

impl Operation {
    fn calculate(&self, operands: [bool; 2]) -> bool {
        match self {
            Operation::And => operands.iter().all(|operand| *operand),
            Operation::Or => operands.iter().any(|operand| *operand),
            Operation::Implication => !(operands[0] && !operands[1]),
            Operation::Equivalence => operands[0] == operands[1],
        }
    }
}

fn apply<T>(
    operands: (BinaryDecisionDiagram<T>, BinaryDecisionDiagram<T>),
    operation: Operation,
) -> BinaryDecisionDiagram<T>
where
    T: Clone + Eq + Ord,
{
    let roots = [operands.0.get_root(), operands.1.get_root()];
    // Basic case
    if operands.0.get_root().is_leaf() && operands.1.get_root().is_leaf() {
        if let (Element::Binary(left), Element::Binary(right)) =
            (roots[0].get_element(), roots[1].get_element())
        {
            return BinaryDecisionDiagram::from(operation.calculate([left, right]));
        } else {
            panic!("Should be the basic case")
        }
    }

    // Case 1: both have smallest variable
    if roots[0].get_element() == roots[1].get_element() {
        let variable = unwrap!(
            roots[0].get_element(),
            Element::Variable(variable),
            variable
        );
        let (t1, t2, u1, u2) = unwrap!(
            (operands.0.split(), operands.1.split()),
            (Some((_, t1, t2)), Some((_, u1, u2))),
            (t1, t2, u1, u2)
        );
        return BinaryDecisionDiagram::new_from_subtrees(
            variable.clone(),
            (apply((t1, u1), operation), apply((t2, u2), operation)),
        );
    }

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_case() {
        let p = BinaryDecisionDiagram::<u32>::from(false);
        let q = BinaryDecisionDiagram::<u32>::from(false);
        println!("{:?}", apply((p, q), Operation::And));
    }

    #[test]
    fn case1() {
        let p = BinaryDecisionDiagram::<u32>::from(false);
        let q = BinaryDecisionDiagram::<u32>::from(false);
        println!("{:?}", apply((p, q), Operation::And));
    }
}
