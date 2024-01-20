use std::rc::Rc;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::{Token, TokenError};

pub struct ParseTokenError(u8);

impl From<TokenError> for ParseTokenError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::Illegal(n) => ParseTokenError(n),
        }
    }
}

pub struct Parser {
    l: Lexer,
    cur_token: Rc<Token>,
    peek_token: Rc<Token>,
}

impl Parser {
    pub fn new(l: Lexer) -> Result<Self, ParseTokenError> {
        let mut p = Self {
            l,
            cur_token: Rc::new(Token::Eof),
            peek_token: Rc::new(Token::Eof),
        };
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }

    pub fn next_token(&mut self) -> Result<(), ParseTokenError> {
        self.cur_token = Rc::clone(&self.peek_token);
        self.peek_token = Rc::new(self.l.next_token()?);
        Ok(())
    }

    pub fn parse_program(&mut self) -> Program {
        todo!();
    }
}
