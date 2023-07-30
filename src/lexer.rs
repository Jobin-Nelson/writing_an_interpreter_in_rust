use crate::token::Token;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: b'\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            b'\0' => Token::EOF,
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'a'..=b'z' | b'A'..=b'Z' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "let" => Token::LET,
                    "fn" => Token::FUNCTION,
                    _ => Token::IDENT(ident),
                }
            }
            b'0'..=b'9' => Token::INT(self.read_numbers()),
            _ => Token::ILLEGAL,
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    pub fn read_numbers(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }
}
