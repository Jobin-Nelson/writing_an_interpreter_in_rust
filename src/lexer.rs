use crate::{token, token::Token};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            '\0' => Token::new(token::EOF, self.ch),
            '=' => Token::new(token::ASSIGN, self.ch),
            '+' => Token::new(token::PLUS, self.ch),
            '-' => Token::new(token::MINUS, self.ch),
            ',' => Token::new(token::COMMA, self.ch),
            ';' => Token::new(token::SEMICOLON, self.ch),
            '(' => Token::new(token::LPAREN, self.ch),
            ')' => Token::new(token::RPAREN, self.ch),
            '{' => Token::new(token::LBRACE, self.ch),
            '}' => Token::new(token::RBRACE, self.ch),
            'a'..='z' => {
                todo!();
            }
            '0'..='9' => {
                todo!();
            }
            _ => Token::new(token::ILLEGAL, self.ch),
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }
}
