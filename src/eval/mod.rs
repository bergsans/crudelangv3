use crate::parser::*;
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
            } => left.eval() + right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Minus),
                rhs: right
            } => left.eval() - right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Mult),
                rhs: right
            } => left.eval() * right.eval(),
            Node::BinaryExpression {
                lhs: left,
                op: OperatorKind::Aritmethic(Sign::Div),
                rhs: right
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
                }.eval(), 2 + 2 - 3
            );
    }

    #[test]
    fn eval_expression_add() {
        let code = "1 + 1".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 2);
    }

    #[test]
    fn eval_expression_sub() {
        let code = "1 - 1".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 0);
    }

    #[test]
    fn eval_expression_mult() {
        let code = "5 * 5".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 25);
    }

    #[test]
    fn eval_expression_div() {
        let code = "10 / 5".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 2);
    }

    #[test]
    fn eval_group_p1() {
        let code = "1 + (2 + 3)".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 6);
    }

    #[test]
    fn eval_group_p2() {
        let code = "1 + (2 + (3 + 4))".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        let mut ns = Parser::new(tokens).parse_expression();
        assert_eq!(ns.eval(), 10);
    }

    #[test]
    fn eval_group_p3() {
        let code1 = "5 * 5 + 10".to_string();
        let tokens1 = Lexer::new(code1).tokenize().unwrap();
        let mut ns1 = Parser::new(tokens1).parse_expression();
        assert_eq!(ns1.eval(), 35);

        let code2 = "5 * (5 + 10)".to_string();
        let tokens2 = Lexer::new(code2).tokenize().unwrap();
        let mut ns2 = Parser::new(tokens2).parse_expression();
        assert_eq!(ns2.eval(), 75);
    }
}
