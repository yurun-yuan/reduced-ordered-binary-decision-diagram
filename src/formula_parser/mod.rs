//! A parser for logic formula.
//! The priority for operators are
//! 1. `!`
//! 2. `&`
//! 3. `|`
//! 4. `->`, `<->`
pub use crate::BinaryOperation;
pub use crate::Operation;
pub use crate::UnaryOperation;
mod grammar;

#[derive(Debug)]
pub enum ParserNode<T> {
    Unary(UnaryOperation, Box<ParserNode<T>>),
    Binary(BinaryOperation, (Box<ParserNode<T>>, Box<ParserNode<T>>)),
    Variable(T),
    Leaf(bool),
}

impl<T> ParserNode<T> {
    pub fn accept<U, F>(&self, f: F) -> U
    where
        F: Fn(&Self) -> U,
    {
        f(self)
    }
}

pub fn formula_parse<'input>(
    formula: &'input str,
) -> Result<
    ParserNode<String>,
    lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'input>, &'static str>,
> {
    grammar::FormulaParser::new().parse(formula)
}

#[test]
fn parse() {
    println!("{:?}", formula_parse("(a->b)&c").unwrap());
}
