#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Int,
    Char,
    OpenParentheses,
    ClosedParentheses,
    Semicolon,
    Return,
    If,
    Else,
    While,
    Do,
    Switch,
    Default,
    Comment,
    OpenBracket,
    ClosedBracket,
    Include,
    Define,
    Operator(String),
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Start,
    InIdentifier,
    InNumber,
    InString,
    InComment,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    state: State,
}

impl Lexer {
    fn advance_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let character = self.input[self.position];
            self.position += 1;
            Some(character)
        } else {
            None
        }
    }
    fn peek_char(&self) -> Option<char> {
        if self.position < self.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    fn read_token(&mut self) -> Option<TokenType>{
    
    }

    fn read_number(&mut self, character: char) -> TokenType {

    }

    fn read_id_or_kw(&mut self, character: char) -> TokenType {

    }

}
