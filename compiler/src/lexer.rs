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
    For,
    Switch,
    DefaultCase,
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
    InDirective,
    InLineComment,
    InBlockComment,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    state: State,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
            state: State::Start,
        }
    }

    fn advance_char(&mut self) -> Option <char> {
        if self.position < self.input.len() {
            self.position += 1;
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(character) = self.peek_char() {
            if character.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }
    }
    
    //fn scan_token(&smut elf) -> Option<TokenType> {  
    //}

    fn keyword_oe_identifier(id: String) -> TokenType {
        match id.as_str() {
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "default" => TokenType::DefaultCase,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "for" => TokenType::For,
            _ => TokenType::Identifier(id),
        }
    }
    
    fn directive_from(name: &str) -> TokenType {
        match name {
            "include" => TokenType::Include,
            "define" => TokenType::Define,
            _ => panic!("Syntax Error: This directive is not supported in this specific version of the compiler.\n"),
        }
    }

}
