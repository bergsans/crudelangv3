use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;

pub fn interpret(code: String) -> i32 {
    eval(parse(tokenize(code).unwrap()))
}
