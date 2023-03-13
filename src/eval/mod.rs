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
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Minus),
                rhs: right
            } => {
                left.eval() - right.eval()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_expression_nested() {
        assert_eq!(
                Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(1)),
                    op: OperatorKind::Aritmethic(Sign::Plus),
                    rhs: Box::new(Node::BinaryExpression {
                        lhs: Box::new(Node::Integer(2)),
                        op: OperatorKind::Aritmethic(Sign::Minus),
                        rhs: Box::new(Node::Integer(3)),
                    })
                }.eval(), 1 + 2 - 3
            );
    }
}
