mod lexer;
mod parser;
mod eval;

fn main() {
        let code = "1 + (4 - 4)".to_string();
        let tokens = lexer::Lexer::new(code).tokenize().unwrap();
        parser::Parser::new(tokens).parse_expression();
}

