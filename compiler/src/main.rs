// in main.rs
mod lexer;
use lexer::{Lexer, TokenType};

fn main() {
    let mut lexer = Lexer::new("int a = 10; // comment ");
    while let Some(token) = lexer.scan_token() {
        println!("{:?}", token);
    }
}
