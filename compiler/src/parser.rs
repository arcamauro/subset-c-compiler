use crate::lexer::TokenType;
pub struct Parser {
    tokens: Vec<TokenType>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    fn advance(&mut self) -> Option<&TokenType> {
        if self.position < self.tokens.len() {
            let tok = &self.tokens[self.position];
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&TokenType> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }
}
