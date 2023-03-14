use std::num::ParseIntError;

use crate::lexer::*;
use crate::eval::*;

#[derive(Debug, PartialEq)]
pub struct Ast {
    program: Node
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

fn to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0
        }
    }

    pub fn peek_kind(&self) -> Option<&TokenKind> {
        match &self.tokens.get(self.position + 1) {
            Some(t) => Some(t.get_kind()),
            _ => None
        }
    }

    pub fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    pub fn parse_expression(&mut self) -> Node {
        let current_token = self.tokens.get(self.position).unwrap();
        match self.peek_kind() {
            Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus))) =>  {
                self.position += 2;
                Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(to_number(current_token.get_literal()).unwrap())),
                    op: OperatorKind::Aritmethic(Sign::Plus),
                    rhs: Box::new(self.parse_expression())
                }
            }
            Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Minus))) =>  {
                self.position += 2;
                Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(to_number(current_token.get_literal()).unwrap())),
                    op: OperatorKind::Aritmethic(Sign::Minus),
                    rhs: Box::new(self.parse_expression())
                }
            }
            _ => Node::Integer(to_number(current_token.get_literal()).unwrap())
        }
    }

    pub fn parse(&mut self) -> Result<Ast, SyntaxError> {
        let current_token = self.tokens.get(self.position).unwrap();
        let n: Node = match current_token.get_kind() {
            TokenKind::Integer =>  self.parse_expression(),
            _ => return Err(SyntaxError { message: "Parse error".to_string() })
        };

        Ok(Ast {
            program: n
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression() {
        let code = "123 + 123".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression(),
            Node::BinaryExpression {
                lhs: Box::new(Node::Integer(123)),
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: Box::new(Node::Integer(123))
            }
        );
    }

    #[test]
    fn parse_expression_nested() {
        let code = "1 + 2 + 3".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression(),
            Node::BinaryExpression {
                lhs: Box::new(Node::Integer(1)),
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: Box::new(Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(2)),
                    op: OperatorKind::Aritmethic(Sign::Plus),
                    rhs: Box::new(Node::Integer(3)),
                })
            }
        );
    }
}
