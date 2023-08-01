use crate::token::{Keyword, Operation, Token, TokenError};

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

    pub fn peek_char(&self) -> Option<&u8> {
        self.input.get(self.read_position)
    }

    pub fn next_token(&mut self) -> Result<Token, TokenError> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'\0' => Token::Eof,
            b'>' => Token::Gt,
            b'<' => Token::Lt,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'=' => {
                if let Some(&b'=') = self.peek_char() {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if let Some(&b'=') = self.peek_char() {
                    self.read_char();
                    Token::Neq
                } else {
                    Token::Bang
                }
            }
            b'+' => Token::Op(Operation::Plus),
            b'-' => Token::Op(Operation::Minus),
            b'*' => Token::Op(Operation::Asterisk),
            b'/' => Token::Op(Operation::Slash),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return Ok(match ident.as_str() {
                    "let" => Token::Kw(Keyword::Let),
                    "fn" => Token::Kw(Keyword::Function),
                    "return" => Token::Kw(Keyword::Return),
                    "if" => Token::Kw(Keyword::If),
                    "else" => Token::Kw(Keyword::Else),
                    "true" => Token::Kw(Keyword::True),
                    "false" => Token::Kw(Keyword::False),
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Ok(Token::Int(self.read_numbers())),
            _ => return Err(TokenError::Illegal),
        };
        self.read_char();
        Ok(tok)
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

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = TokenIterator;

    fn into_iter(self) -> Self::IntoIter {
        TokenIterator {
            lexer: self,
            is_eof: false,
        }
    }
}

pub struct TokenIterator {
    lexer: Lexer,
    is_eof: bool,
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }
        let next_token = self.lexer.next_token().ok();
        if let Some(Token::Eof) = next_token {
            self.is_eof = true;
        }
        next_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let expected_tokens: Vec<Token> = vec![
            Token::Kw(Keyword::Let),
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Kw(Keyword::Let),
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Kw(Keyword::Let),
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Kw(Keyword::Function),
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Op(Operation::Plus),
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Kw(Keyword::Let),
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Op(Operation::Minus),
            Token::Op(Operation::Slash),
            Token::Op(Operation::Asterisk),
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Gt,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Kw(Keyword::If),
            Token::Lparen,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Kw(Keyword::Return),
            Token::Kw(Keyword::True),
            Token::Semicolon,
            Token::Rbrace,
            Token::Kw(Keyword::Else),
            Token::Lbrace,
            Token::Kw(Keyword::Return),
            Token::Kw(Keyword::False),
            Token::Semicolon,
            Token::Rbrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::Neq,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.to_string());

        // assert!(l.into_iter().eq(expected_tokens.into_iter()));
        // l.into_iter().enumerate().for_each(|(i, tok)| {
        //     println!("recieved {:?}, expected: {:?}", tok, expected_tokens.get(i));
        //     assert_eq!(&tok, expected_tokens.get(i).unwrap());
        // });
        for exp_tok in expected_tokens {
            assert_eq!(l.next_token().unwrap(), exp_tok);
        }
    }
}
