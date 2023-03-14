use std::error::Error;

use crate::parser::*;
use crate::eval::*;
use crate::lexer::*;

pub fn interpret(code: String) -> Result<String, Box<dyn Error>> {
    let tokens = Lexer::new(code).tokenize().unwrap();
    let mut ast = Parser::new(tokens).parse().unwrap();

    Ok(eval(ast))
}
