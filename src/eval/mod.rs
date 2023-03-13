use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    Integer(i32),
    BinaryExpression {
        op: OperatorKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Node {
    pub fn eval(&mut self) -> i32 {
        match self {
            Node::Integer(n) => *n,
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: right
            } => {
                left.eval() + right.eval()
            }
        }
    }
}
