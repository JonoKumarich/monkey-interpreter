use std::collections::HashMap;


#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Self {
        Token {
            literal: String::from_utf8(vec![ch]).unwrap(),
            kind: token_type
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT, // 1343456
    
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERIX,
    SLASH,

    GT,
    LT,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

// impl TokenType {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             TokenType::EOF => "EOF",
//             TokenType::ILLEGAL => "ILLEGAL",
//             TokenType::IDENT => "IDENT",
//             TokenType::INT => "INT",
//             TokenType::ASSIGN => "=",
//             TokenType::PLUS => "+",
//             TokenType::COMMA => ",",
//             TokenType::SEMICOLON => ";",
//             TokenType::LPAREN => "(",
//             TokenType::RPAREN => ")",
//             TokenType::LBRACE => "{",
//             TokenType::RBRACE => "}",
//             TokenType::LET => "LET",
//             TokenType::FUNCTION => "FUNCTION",
//         }
//     }
// }

pub fn lookup_ident(ident: &String) -> TokenType {
    let mut keywords = HashMap::new();

    keywords.insert("fn".to_owned(), TokenType::FUNCTION);
    keywords.insert("let".to_owned(), TokenType::LET);
    keywords.insert("if".to_owned(), TokenType::IF);
    keywords.insert("else".to_owned(), TokenType::ELSE);
    keywords.insert("true".to_owned(), TokenType::TRUE);
    keywords.insert("false".to_owned(), TokenType::FALSE);
    keywords.insert("return".to_owned(), TokenType::RETURN);

    *keywords.get(ident).unwrap_or(&TokenType::IDENT)

}

