use crate::lexer::*;

#[allow(unused_imports)]
use crate::parser::*;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub program: Node,
}

pub fn eval(mut ast: Ast) -> i32 {
    ast.program.eval()
}

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
                rhs: right,
            } => left.eval() + right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Minus),
                rhs: right,
            } => left.eval() - right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Mult),
                rhs: right,
            } => left.eval() * right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Div),
                rhs: right,
            } => left.eval() / right.eval(),
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
                lhs: Box::new(Node::Integer(2)),
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: Box::new(Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(2)),
                    op: OperatorKind::Aritmethic(Sign::Minus),
                    rhs: Box::new(Node::Integer(3)),
                })
            }
            .eval(),
            2 + 2 - 3
        );
    }

    #[test]
    fn eval_expression_add() {
        let ast = parse(tokenize("1 + 1".to_string()).unwrap());
        assert_eq!(eval(ast), 2);
    }

    #[test]
    fn eval_expression_sub() {
        let ast = parse(tokenize("1 - 1".to_string()).unwrap());
        assert_eq!(eval(ast), 0);
    }

    #[test]
    fn eval_expression_mult() {
        let ast = parse(tokenize("5 * 5".to_string()).unwrap());
        assert_eq!(eval(ast), 25);
    }

    #[test]
    fn eval_expression_div() {
        let ast = parse(tokenize("10 / 5".to_string()).unwrap());
        assert_eq!(eval(ast), 2);
    }

    #[test]
    fn eval_group_p1() {
        let ast = parse(tokenize("1 + (2 + 3)".to_string()).unwrap());
        assert_eq!(eval(ast), 6);
    }

    #[test]
    fn eval_group_p2() {
        let ast = parse(tokenize("1 + (2 + (3 + 4))".to_string()).unwrap());
        assert_eq!(eval(ast), 10);
    }

    #[test]
    fn eval_eval() {
        let ast = parse(tokenize("1 + 1".to_string()).unwrap());
        assert_eq!(eval(ast), 2);
    }

    #[test]
    fn eval_group_p3() {
        let ast1 = parse(tokenize("5 * 5 + 10".to_string()).unwrap());
        assert_eq!(eval(ast1), 35);

        let ast2 = parse(tokenize("5 * (5 + 10)".to_string()).unwrap());
        assert_eq!(eval(ast2), 75);
    }
}
