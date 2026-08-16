#[derive(Debug, PartialEq, Clone)]
enum Token {
    Int,
    Char,
    OpenParentheses,
    ClosedParentheses,
    Semicolon,
    Return,
    If,
    While,
    Do,
    Switch,
    Default,
    Comment,
    OpenBracket,
    ClosedBracket,
    Operator(String),
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),
}

