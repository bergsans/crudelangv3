mod lexer;

fn main() {
    let code = "1 + 1".to_string();
    let mut lexer = lexer::Lexer::new(code);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}

