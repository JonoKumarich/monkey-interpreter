#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: u8,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Self {
        Token {
            literal: ch,
            kind: token_type
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT, // 1343456
    
    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::EOF => "EOF",
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "=",
            TokenType::PLUS => "+",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::LPAREN => "(",
            TokenType::RPAREN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::LET => "LET",
            TokenType::FUNCTION => "FUNCTION",
        }
    }
}


        
