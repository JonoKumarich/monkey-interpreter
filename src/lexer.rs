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
        let token_type = match self.ch {
            b'=' => TokenType::ASSIGN,
            b';' => TokenType::SEMICOLON,
            b',' => TokenType::COMMA,
            b'+' => TokenType::PLUS,
            b'(' => TokenType::LPAREN,
            b'{' => TokenType::LBRACE,
            b')' => TokenType::RPAREN,
            b'}' => TokenType::RBRACE,
            0 => TokenType::EOF,
            _ => panic!("{}", self.ch)
            // _ => if self.ch.is_ascii_alphabetic()
        };
        
        let token = Token::new(token_type, self.ch);

        self.read_char();
        token
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";
        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ":"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ":"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for (token_type, text) in tests {
            let token = lexer.next_token();

            assert_eq!(token_type, token.kind);
            assert_eq!(text, token.literal);
        }
    }
}

