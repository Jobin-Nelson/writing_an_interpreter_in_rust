use std::io;

use monkey_interpreter::repl;

fn main() {
    let user = std::env::var("USER").unwrap();
    println!("Hello {}! This is Monkey programming language", user);
    println!("Feel free to type in commands");

    repl::start(io::stdin(), io::stdout()).unwrap();
}
