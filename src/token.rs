#[derive(PartialEq, Debug)]
pub enum Token {
    Ident(String),
    Int(String),
    Eof,
    Assign,
    Plus,
    Minus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

#[derive(PartialEq, Debug)]
pub enum TokenError {
    Illegal,
}
