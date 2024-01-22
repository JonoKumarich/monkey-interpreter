use std::io;
use std::io::Write;
use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &'static str = ">> ";


pub fn start() {
    println!("Welcome to Monkey!");


    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");


        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_token();
            println!("{}, {}", token.kind, token.literal);
            if token.kind == TokenType::EOF {
                break
            }
        }

    }


}
