use std::io::prelude::*;

use crate::interpret::*;

mod eval;
mod interpret;
mod lexer;
mod parser;

fn get_line() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("==============");
    println!("CRUDELANG v. 3");
    println!("==============");
    loop {
        println!("{}", interpret(get_line()));
    }
}
