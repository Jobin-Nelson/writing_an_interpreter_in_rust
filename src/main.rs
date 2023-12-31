use monkey_interpreter::repl;

// fn main() {
//     run_repl();
// }

use std::io;

fn main() {
    let user = std::env::var("USER").unwrap();
    println!("Hello {}! This is Monkey programming language", user);
    println!("Feel free to type in commands");

    repl::start(io::stdin().lock(), io::stdout().lock()).unwrap();
}
