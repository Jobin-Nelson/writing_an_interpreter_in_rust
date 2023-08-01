#[derive(PartialEq, Debug)]
pub enum Token {
    Ident(String),
    Int(String),
    Bang,
    Lt,
    Gt,
    Eq,
    Neq,
    Assign,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Op(Operation),
    Kw(Keyword),
    Eof,
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    Function,
    Return,
    Let,
    If,
    Else,
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub enum TokenError {
    Illegal,
}
