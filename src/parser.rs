use crate::ast::{Node, Program};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            cur_token: Token::Eof,
            peek_token: Some(Token::Eof),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone().unwrap();
        self.peek_token = self.l.next_token().ok();
    }

    pub fn parse_program(&mut self) -> Program {
        todo!();
    }
}
