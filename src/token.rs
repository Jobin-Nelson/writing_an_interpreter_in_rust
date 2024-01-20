#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    Function,
    Return,
    Let,
    If,
    Else,
    True,
    False,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenError {
    Illegal(u8),
}

impl Token {
    pub fn literal(&self) -> &str {
        match &self {
            Token::Ident(s) => s.as_str(),
            Token::Int(i) => i.as_str(),
            Token::Bang => "!",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::Eq => "==",
            Token::Neq => "!=",
            Token::Assign => "=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Lparen => "(",
            Token::Rparen => ")",
            Token::Lbrace => "{",
            Token::Rbrace => "}",
            Token::Op(o) => match o {
                Operation::Plus => "+",
                Operation::Minus => "-",
                Operation::Asterisk => "*",
                Operation::Slash => "/",
            },
            Token::Kw(k) => match k {
                Keyword::Function => "function",
                Keyword::Return => "return",
                Keyword::Let => "let",
                Keyword::If => "if",
                Keyword::Else => "else",
                Keyword::True => "true",
                Keyword::False => "false",
            },
            Token::Eof => "EOF",
        }
    }
}
