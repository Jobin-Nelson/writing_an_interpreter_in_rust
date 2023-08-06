use crate::token::{Keyword, Operation, Token};

pub trait Node {
    fn get_token(&self) -> &Token;
    fn token_literal(&self) -> &str {
        match self.get_token() {
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

pub struct Statement;
pub struct Expression;

// Root node
pub struct Program<T> {
    statements: Vec<T>,
}

impl<T: Node> Node for Program<T> {
    fn get_token(&self) -> &Token {
        self.statements[0].get_token()
    }
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            return "";
        }
        self.statements[0].token_literal()
    }
}

// Statement nodes
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn get_token(&self) -> &Token {
        &self.token
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn get_token(&self) -> &Token {
        &self.token
    }
}
