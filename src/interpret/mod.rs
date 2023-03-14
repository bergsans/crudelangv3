use std::error::Error;

use crate::parser::*;
use crate::lexer::*;

pub fn interpret(code: String) -> Result<i32, Box<dyn Error>> {
    let tokens = Lexer::new(code).tokenize().unwrap();
    let mut ast = Parser::new(tokens).parse_expression();

    Ok(ast.eval())
}
