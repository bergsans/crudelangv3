mod predicates;

#[derive(Debug, PartialEq, Clone)]
pub enum Sign {
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorKind {
    Aritmethic(Sign),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Integer,
    // String,
    Operator(OperatorKind),
    RightParens,
    LeftParens,
    Identifier,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }

    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn get_literal(&self) -> &String {
        &self.literal
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            position: 0,
        }
    }

    pub fn parse_number(&mut self) -> String {
        let mut buff: String = String::new();
        loop {
            match self.current_char() {
                Some(c) if c.is_digit(10) => {
                    buff.push(self.current_char().unwrap());
                    self.position += 1;
                }
                _ => break,
            }
        }
        buff
    }

    pub fn parse_parens(&mut self) -> TokenKind {
        match self.current_char() {
            Some(c) if c == '(' => {
                self.position += 1;
                TokenKind::LeftParens
            }
            Some(c) if c == ')' => {
                self.position += 1;
                TokenKind::RightParens
            }
            _ => panic!("Expected operator"),
        }
    }

    pub fn parse_operator(&mut self) -> OperatorKind {
        match self.current_char() {
            Some(c) if c == '+' => {
                self.position += 1;
                OperatorKind::Aritmethic(Sign::Plus)
            }
            Some(c) if c == '*' => {
                self.position += 1;
                OperatorKind::Aritmethic(Sign::Mult)
            }
            Some(c) if c == '/' => {
                self.position += 1;
                OperatorKind::Aritmethic(Sign::Div)
            }
            Some(c) if c == '-' => {
                self.position += 1;
                OperatorKind::Aritmethic(Sign::Minus)
            }
            _ => panic!("Expected operator"),
        }
    }

    pub fn parse_identifier(&mut self) -> String {
        let mut buff: String = String::new();
        loop {
            match self.current_char() {
                Some(c) if c.is_alphabetic() => {
                    buff.push(self.current_char().unwrap());
                    self.position += 1;
                }
                _ => break,
            }
        }
        buff
    }

    //pub fn parse_string(&mut self) -> String {
    //    let mut buff: String = String::new();
    //    self.position += 1;
    //    loop {
    //        match self.current_char() {
    //            Some(c) if c == '"' => {
    //                self.position += 1;
    //                break;
    //            }
    //            _ => {
    //                buff.push(self.current_char().unwrap());
    //                self.position += 1;
    //            }
    //        }
    //    }
    //    buff
    //}

    pub fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, SyntaxError> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.position < self.source.len() {
            match self.current_char() {
                Some(c) if c.is_digit(10) => {
                    tokens.push(Token::new(TokenKind::Integer, self.parse_number()))
                }
                Some(maybe_operator) if predicates::is_operator(maybe_operator) => {
                    tokens.push(Token::new(
                        TokenKind::Operator(self.parse_operator()),
                        maybe_operator.to_string(),
                    ))
                }
                Some(maybe_parens) if predicates::is_parens(maybe_parens) => {
                    tokens.push(Token::new(self.parse_parens(), maybe_parens.to_string()))
                }
                Some(maybe_whitespace) if predicates::is_whitespace(maybe_whitespace) => {
                    self.position += 1
                }
                Some(c) if c.is_alphabetic() => {
                    tokens.push(Token::new(TokenKind::Identifier, self.parse_identifier()))
                }
                //Some(maybe_doublequote) if predicates::is_doublequote(maybe_doublequote) => {
                //    tokens.push(Token::new(TokenKind::String, self.parse_string()))
                //}
                _ => {
                    return Err(SyntaxError {
                        message: "Invalid character".to_string(),
                    })
                }
            }
        }
        Ok(tokens)
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, SyntaxError> {
    Lexer::new(input).tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_number() {
        let tokens = tokenize("45".to_string()).unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::Integer,
                literal: "45".to_string()
            }
        );
    }

    //#[test]
    //fn tokenize_string() {
    //    let tokens = tokenize("\"text\"".to_string()).unwrap();
    //    assert_eq!(
    //        tokens.get(0).unwrap(),
    //        &Token {
    //            kind: TokenKind::String,
    //            literal: "text".to_string()
    //        }
    //    );
    //}

    #[test]
    fn tokenize_plus() {
        let tokens = Lexer::new("+".to_string()).tokenize().unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)),
                literal: "+".to_string()
            }
        );
    }

    #[test]
    fn tokenize_minus() {
        let tokens = Lexer::new("-".to_string()).tokenize().unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::Operator(OperatorKind::Aritmethic(Sign::Minus)),
                literal: "-".to_string()
            }
        );
    }

    #[test]
    fn tokenize_syntax_error() {
        let tokens = Lexer::new("&".to_string()).tokenize();
        match tokens {
            Ok(_t) => (),
            Err(e) => assert_eq!(
                e,
                SyntaxError {
                    message: "Invalid character".to_string()
                }
            ),
        };
    }

    #[test]
    fn tokenize_identifier() {
        let tokens = Lexer::new("someIdentifier".to_string()).tokenize().unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::Identifier,
                literal: "someIdentifier".to_string()
            }
        );
    }

    #[test]
    fn tokenize_expression() {
        let tokens = Lexer::new("1 + 1".to_string()).tokenize().unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::Integer,
                literal: "1".to_string()
            }
        );
        assert_eq!(
            tokens.get(1).unwrap(),
            &Token {
                kind: TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)),
                literal: "+".to_string()
            }
        );
        assert_eq!(
            tokens.get(2).unwrap(),
            &Token {
                kind: TokenKind::Integer,
                literal: "1".to_string()
            }
        );
    }

    #[test]
    fn tokenize_parens() {
        let tokens = Lexer::new("()".to_string()).tokenize().unwrap();
        assert_eq!(
            tokens.get(0).unwrap(),
            &Token {
                kind: TokenKind::LeftParens,
                literal: "(".to_string()
            }
        );
        assert_eq!(
            tokens.get(1).unwrap(),
            &Token {
                kind: TokenKind::RightParens,
                literal: ")".to_string()
            }
        );
    }
}
