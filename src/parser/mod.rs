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

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }

    pub fn parse(&mut self) -> Node {
        Node::BinaryExpression {
            op: OperatorKind::Aritmethic(Sign::Plus),
            lhs: Box::new(Node::Integer(2)),
            rhs: Box::new(Node::Integer(2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression() {
        let code = "45".to_string();
        let tokens = Lexer::new(code).tokenize();
        assert_eq!(
            Parser::new(tokens).parse(),
            Node::BinaryExpression {
                op: OperatorKind::Aritmethic(Sign::Plus),
                lhs: Box::new(Node::Integer(2)),
                rhs: Box::new(Node::Integer(2))
            }
        );
    }

}
