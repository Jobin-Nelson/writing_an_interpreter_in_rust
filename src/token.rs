#[derive(PartialEq, Debug)]
pub enum Token {
    IDENT(String),
    INT(String),
    ILLEGAL,
    EOF,
    ASSIGN,
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
