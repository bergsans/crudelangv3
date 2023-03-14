use std::io::prelude::*;

use crate::interpret::*;

mod lexer;
mod parser;
mod eval;
mod interpret;

fn get_line() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_s) => {}
        Err(_e) => {}
    };
    input.trim().to_string()
}


fn main() {
    println!("==============");
    println!("CRUDELANG v. 3");
    println!("==============");
    loop {
        match interpret(get_line()) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
