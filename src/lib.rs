use std::io;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

pub fn run_repl() {
    let user = std::env::var("USER").unwrap();
    println!("Hello {}! This is Monkey programming language", user);
    println!("Feel free to type in commands");

    repl::start(io::stdin(), io::stdout()).unwrap();
}
