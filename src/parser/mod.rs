use std::num::ParseIntError;

use crate::eval::*;
use crate::lexer::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

fn get_group(tokens: &Vec<Token>, position: usize) -> Result<(usize, Vec<Token>), SyntaxError> {
    let mut temp_counter = 0;
    loop {
        let current_token = match tokens.get(position + temp_counter) {
            Some(t) => t,
            None => return Err(SyntaxError { message: "Expected RightPares".to_string() })
        };
        match current_token.get_kind() {
            TokenKind::RightParens => {
                temp_counter += 1;
                break;
            }
            _ => temp_counter += 1,
        }
    }
    let slice = tokens[position..position + temp_counter].to_vec();
    Ok((temp_counter, slice))
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

    pub fn parse_group(&mut self) -> Result<Node, SyntaxError> {
        self.position += 1;
        let (temp_counter, slice) = get_group(&self.tokens, self.position).unwrap();
        let expression = Parser::new(slice).parse_expression();
        match expression {
            Ok(expr) => {
                self.position += temp_counter;
                Ok(expr)
            },
            Err(_) => Err(SyntaxError { message: "Error parsing group".to_string() })
        }
    }

    pub fn parse_integer(&mut self) -> Result<Node, SyntaxError> {
        let current_token = self.tokens.get(self.position).unwrap();
        self.position += 1;
        let int = to_number(current_token.get_literal());
        match int {
            Ok(n) => Ok(Node::Integer(n)),
            Err(e) => Err(SyntaxError { message: e.to_string() })
        }
    }

    pub fn parse_atomic(&mut self) -> Result<Node, SyntaxError> {
        let current_token = self.tokens.get(self.position).unwrap();
        match current_token.get_kind() {
            TokenKind::Integer => Ok(self.parse_integer().unwrap()),
            TokenKind::LeftParens => Ok(self.parse_group().unwrap()),
            _ => Err(SyntaxError { message: "Pannnnik".to_string() }),
        }
    }

    pub fn parse_operator(&mut self) -> Result<OperatorKind, SyntaxError> {
        let current_token = self.tokens.get(self.position).unwrap();
        self.position += 1;
        let operator = match current_token.get_kind() {
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
            _ => return Err(SyntaxError { message: "Expected operator".to_string() }),
        };
        Ok(operator)
    }

    pub fn parse_binary_expression(&mut self) -> Result<Node, SyntaxError> {
        let expr = Node::BinaryExpression {
            lhs: Box::new(self.parse_atomic().unwrap()),
            op: self.parse_operator().unwrap(),
            rhs: Box::new(self.parse_expression().unwrap()),
        };
        Ok(expr)
    }

    pub fn parse_expression(&mut self) -> Result<Node, SyntaxError> {
        match self.peek_kind() {
            Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Mult)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Div)))
            | Some(TokenKind::Operator(OperatorKind::Aritmethic(Sign::Minus))) => {
                Ok(self.parse_binary_expression().unwrap())
            }
            _ => Ok(self.parse_atomic().unwrap())
        }
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Ast {
        let current_token = self.tokens.get(self.position).unwrap();
        let n: Node = match current_token.get_kind() {
            TokenKind::Integer => self.parse_expression().unwrap(),
            _ => panic!("Parse error")
        };

        // refactor me
        Ast { program: n }
    }
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    Parser::new(tokens).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expression() {
        let tokens = tokenize("123 + 123".to_string()).unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression().unwrap(),
            Node::BinaryExpression {
                lhs: Box::new(Node::Integer(123)),
                op: OperatorKind::Aritmethic(Sign::Plus),
                rhs: Box::new(Node::Integer(123))
            }
        );
    }

    #[test]
    fn parse_expression_nested() {
        let tokens = tokenize("1 + 2 + 3".to_string()).unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression().unwrap(),
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
        let tokens = tokenize("1 + (4 - 5)".to_string()).unwrap();
        assert_eq!(
            Parser::new(tokens).parse_expression().unwrap(),
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
