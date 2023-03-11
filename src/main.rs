fn main() {
    let code = "45".to_string();
    let mut lexer = Lexer::new(code);
    let _tokens = lexer.tokenize();
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Integer
}

#[derive(Debug, PartialEq)]
struct Token {
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
struct Lexer {
    source: Vec<char>,
    position: usize
}

const EOF: char = '#';

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
            match self.current_char().is_digit(10) {
            true => {
                buff.push(self.current_char());
                self.position += 1;
            }
            _ => break
            }
        }
        buff
    }

    pub fn current_char(&self) -> char {
        *self.source.get(self.position).unwrap_or(&EOF)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.position < self.source.len() {
            match self.current_char() {
                _ if self.current_char().is_digit(10) => {
                    tokens.push(Token::new(TokenKind::Integer,self.parse_number()));
                },
                _ => {
                    //panic!("Invalid character!");
                    break;
                }
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_number() {
        let code = "45".to_string();
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        assert_eq!(tokens.get(0).unwrap(), &Token { kind: TokenKind::Integer, literal: "45".to_string() });
    }

}
