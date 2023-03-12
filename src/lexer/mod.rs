mod predicates;

#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    Aritmethic(Sign)
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Integer,
    String,
    Operator(OperatorKind),
    Identifier
}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    message: String,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    literal: String
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {
            kind,
            literal
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            source: contents.chars().collect(),
            position: 0
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
                _ => break
            }
        }
        buff
    }

    pub fn parse_operator(&mut self) -> OperatorKind {
        match self.current_char()  {
            Some(c) if c == '+' => {
                self.position += 1;
                OperatorKind::Aritmethic(Sign::Plus)
            }
            _ => panic!("Expected operator")
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
                _ => break
            }
        }
        buff
    }

    pub fn parse_string(&mut self) -> String {
        let mut buff: String = String::new();
        self.position += 1;
        loop {
            match self.current_char() {
            Some(c) if c == '"' => {
                self.position += 1;
                break;
            }
            _ => {
                buff.push(self.current_char().unwrap());
                self.position += 1;
            }
            }
        }
        buff
    }

    pub fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, SyntaxError> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.position < self.source.len() {
            match self.current_char() {
                Some(c) if c.is_digit(10) =>
                    tokens.push(Token::new(TokenKind::Integer, self.parse_number())),
                Some(maybe_operator) if predicates::is_operator(maybe_operator) =>
                    tokens.push(Token::new(TokenKind::Operator(self.parse_operator()), maybe_operator.to_string())),
                Some(c) if c == ' ' => self.position += 1,
                Some(c) if c.is_alphabetic() =>
                    tokens.push(Token::new(TokenKind::Identifier, self.parse_identifier())),
                Some(c) if c == '"' => tokens.push(Token::new(TokenKind::String,self.parse_string())),
                _ => return Err(SyntaxError { message: "Invalid character".to_string() })
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_number() {
        let code = "45".to_string();
        let tokens = Lexer::new(code).tokenize();
        match tokens {
            Ok(t) => assert_eq!(t.get(0).unwrap(), &Token { kind: TokenKind::Integer, literal: "45".to_string() }),
            Err(_e) => ()
        };
    }

    #[test]
    fn tokenize_string() {
        let code = "\"text\"".to_string();
        let tokens = Lexer::new(code).tokenize();
        match tokens {
            Ok(t) => assert_eq!(t.get(0).unwrap(), &Token { kind: TokenKind::String, literal: "text".to_string() }),
            Err(_e) => ()
        };

    }

    #[test]
    fn tokenize_plus() {
        let code = "+".to_string();
        let tokens = Lexer::new(code).tokenize();
        match tokens {
            Ok(t) => assert_eq!(t.get(0).unwrap(), &Token { kind: TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)), literal: "+".to_string() }),
            Err(_e) => ()
        };
    }

    #[test]
    fn tokenize_syntax_error() {
        let code = "&".to_string();
        let tokens = Lexer::new(code).tokenize();
        match tokens {
            Ok(_t) => (),
            Err(e) => assert_eq!(e, SyntaxError { message: "Invalid character".to_string() })
        };
    }

    #[test]
    fn tokenize_identifier() {
        let code = "someIdentifier".to_string();
        let tokens = Lexer::new(code).tokenize();
        match tokens {
            Ok(t) => assert_eq!(t.get(0).unwrap(), &Token { kind: TokenKind::Identifier, literal: "someIdentifier".to_string() }),
            Err(_e) => ()
        };
    }

    #[test]
    fn tokenize_expression() {
        let code = "1 + 1".to_string();
        let tokens = Lexer::new(code).tokenize();

        match &tokens {
            Ok(t) => assert_eq!(t.get(0).unwrap(), &Token { kind: TokenKind::Integer, literal: "1".to_string() }),
            Err(_e) => ()
        };
        match &tokens {
            Ok(t) => assert_eq!(t.get(1).unwrap(), &Token { kind: TokenKind::Operator(OperatorKind::Aritmethic(Sign::Plus)), literal: "+".to_string() }),
            Err(_e) => ()
        };
        match &tokens {
            Ok(t) => assert_eq!(t.get(2).unwrap(), &Token { kind: TokenKind::Integer, literal: "1".to_string() }),
            Err(_e) => ()
        };
    }
}
