use std::io::{self, Write};

use crate::lexer::Lexer;

const PROMPT: &[u8] = b">> ";

pub fn start(input: io::Stdin, output: io::Stdout) -> Result<(), io::Error> {
    let mut locked_output = output.lock();
    locked_output.write_all(PROMPT)?;
    locked_output.flush()?;

    for line in input.lines() {
        let lex = Lexer::new(line.unwrap());
        for tok in lex {
            writeln!(locked_output, "{:?}", tok)?;
        }
        locked_output.write_all(PROMPT)?;
        locked_output.flush()?;
    }
    Ok(())
}
