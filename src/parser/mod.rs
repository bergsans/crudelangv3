use std::num::ParseIntError;

use crate::eval::*;
use crate::lexer::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

fn to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn peek_kind(&self) -> Option<&TokenKind> {
        match &self.tokens.get(self.position + 1) {
            Some(t) => Some(t.get_kind()),
            _ => None,
        }
    }

    pub fn parse_group(&mut self) -> Node {
        self.position += 1;
        let mut temp_counter = 0;
        loop {
            let current_token = self.tokens.get(self.position + temp_counter).unwrap();
            match current_token.get_kind() {
                TokenKind::RightParens => {
                    temp_counter += 1;
                    break;
                },
                _ => temp_counter += 1
            }
        }
        let slice = self.tokens[self.position ..self.position + temp_counter].to_vec();
        let expression = Parser::new(slice).parse_expression();
        self.position += temp_counter;
        expression
    }

    pub fn parse_integer(&mut self) -> Node {
        let current_token = self.tokens.get(self.position).unwrap();
        self.position += 1;
        Node::Integer(to_number(current_token.get_literal()).unwrap())
    }

    pub fn parse_atomic(&mut self) -> Node {
        let current_token = self.tokens.get(self.position).unwrap();
        match current_token.get_kind() {
            TokenKind::Integer => self.parse_integer(),
            TokenKind::LeftParens => self.parse_group(),
            _ => panic!("Pannnnik"),
        }
    }

    pub fn parse_operator(&mut self) -> OperatorKind {
        let current_token = self.tokens.get(self.position).unwrap();
        self.position += 1;
        match current_token.get_kind() {
            TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)) => {
                OperatorKind::Aritmethic(Sign::Plus)
            }
            TokenKind::Operator(OperatorKind::Aritmethic(Sign::Mult)) => {
                OperatorKind::Aritmethic(Sign::Mult)
            }
            TokenKind::Operator(OperatorKind::Aritmethic(Sign::Div)) => {
                OperatorKind::Aritmethic(Sign::Div)
            }
            TokenKind::Operator(OperatorKind::Aritmethic(Sign::Minus)) => {
                OperatorKind::Aritmethic(Sign::Minus)
            }
            _ => panic!("expected operator!"),
        }
    }

    pub fn parse_binary_expression(&mut self) -> Node {
        Node::BinaryExpression {
            lhs: Box::new(self.parse_atomic()),
            op: self.parse_operator(),
            rhs: Box::new(self.parse_expression()),
        }
    }

    pub fn parse_expression(&mut self) -> Node {
        match self.peek_kind() {
            Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Mult)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Div)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Minus))) => {
                self.parse_binary_expression()
            }
            _ => self.parse_atomic(),
        }
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Result<Ast, SyntaxError> {
        let current_token = self.tokens.get(self.position).unwrap();
        let n: Node = match current_token.get_kind() {
            TokenKind::Integer => self.parse_expression(),
            _ => {
                return Err(SyntaxError {
                    message: "Parse error".to_string(),
                })
            }
        };

        // refactor me
        Ok(Ast { program: vec![n] })
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

    #[test]
    fn parse_grouped_expression_nested() {
        let code = "1 + (4 - 5)".to_string();
        let tokens = Lexer::new(code).tokenize().unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression(),
            Node::BinaryExpression {
                lhs: Box::new(Node::Integer(1)),
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: Box::new(Node::BinaryExpression {
                    lhs: Box::new(Node::Integer(4)),
                    op: OperatorKind::Aritmethic(Sign::Minus),
                    rhs: Box::new(Node::Integer(5)),
                })
            }
        );
    }
}
