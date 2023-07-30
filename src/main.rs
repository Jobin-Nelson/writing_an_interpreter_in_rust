mod lexer;
mod token;

fn main() {
    let ch = ' ' as u8;
    println!("{}", ch);
}

#[cfg(test)]
mod lexer_test;
