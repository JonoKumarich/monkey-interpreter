use crate::token::{TokenType, Token};


pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8 // Input file must be utf-8 encoded
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self { 
            input, 
            position: 0, 
            read_position: 0, 
            ch: 0 
        };

        lexer.read_char();
        lexer
    }

    fn read_char(self: &mut Self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(self: &mut Self) -> Token {
        let token = match self.ch {
            b'=' => Token::new(TokenType::ASSIGN, self.ch),
            b';' => Token::new(TokenType::SEMICOLON, self.ch),
            b',' => Token::new(TokenType::COMMA, self.ch),
            b'+' => Token::new(TokenType::PLUS, self.ch),
            b'(' => Token::new(TokenType::LPAREN, self.ch),
            b'{' => Token::new(TokenType::LBRACE, self.ch),
            b')' => Token::new(TokenType::RPAREN, self.ch),
            b'}' => Token::new(TokenType::RBRACE, self.ch),
            0 => Token::new(TokenType::EOF, 0),
            _ => panic!("Token not found {}", self.ch)
        };

        self.read_char();
        token
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = &"=+(){},;";

        let tests = vec![
            (TokenType::ASSIGN, b'='), 
            (TokenType::PLUS, b'+'),  
            (TokenType::LPAREN, b'('),  
            (TokenType::RPAREN, b')'),  
            (TokenType::LBRACE, b'{'),  
            (TokenType::RBRACE, b'}'),  
            (TokenType::COMMA, b','),  
            (TokenType::SEMICOLON, b';'),  
            (TokenType::EOF, 0),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (token_type, bytes) in tests {
            let token = lexer.next_token();

            assert_eq!(token.kind, token_type);
            assert_eq!(token.literal, bytes);
        }
    }
}

