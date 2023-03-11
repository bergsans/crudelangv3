mod lexer;

fn main() {
    let code = "123 + 321".to_string();
    let tokens = lexer::Lexer::new(code).tokenize();
    println!("{:?}", tokens);
}

