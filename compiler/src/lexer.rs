#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Int,
    Char,
    OpenParentheses,
    ClosedParentheses,
    Semicolon,
    Colon,
    Return,
    If,
    Else,
    While,
    Do,
    For,
    Switch,
    Case,
    DefaultCase,
    Break,
    Continue,
    OpenBracket,
    ClosedBracket,
    Include,
    Define,
    Operator(String),
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),
    CharLiteral(char),
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Start,
    InIdentifier,
    InNumber,
    InString,
    InChar,
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
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
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
    
   pub fn scan_token(&mut self) -> Option<TokenType> {
        let mut buffer = String::new();
        loop {
            match self.state {
                State::Start => {
                    self.skip_whitespaces();
                    let character = self.peek_char()?;
                    buffer.clear();
                    match character {
                        ';' => {
                            self.advance_char();
                            return Some(TokenType::Semicolon);
                        },
                        ':' => {
                            self.advance_char();
                            return Some(TokenType::Colon);
                        },
                        '(' => {
                            self.advance_char();
                            return Some(TokenType::OpenParentheses)
                        },
                        ')' => {
                            self.advance_char();
                            return Some(TokenType::ClosedParentheses);
                        },
                        '{' => {
                            self.advance_char();
                            return Some(TokenType::OpenBracket);
                        },
                        '}' => {
                            self.advance_char();
                            return Some(TokenType::ClosedBracket);
                        },
                        '#' => {
                            self.advance_char();
                            self.state = State::InDirective;
                        }
                        '"' => {
                            self.advance_char();
                            self.state = State::InString;
                        }
                        '\'' => {
                            self.advance_char();
                            self.state = State::InChar;
                        }
                        '%' => {
                            self.advance_char();
                            return Some(TokenType::Operator('%'.to_string()));
                        }
                        '*' => {
                            self.advance_char();
                            return Some(TokenType::Operator('*'.to_string()));
                        }
                        '+' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('+') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("++".to_string()));
                                }
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("+=".to_string()));
                                }
                                _ => return Some(TokenType::Operator('+'.to_string())),
                            }
                        }
                        '-' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('-') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("--".to_string()));
                                }
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("-=".to_string()));
                                }
                                _ => return Some(TokenType::Operator('-'.to_string())),
                            }
                        }
                        '<' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("<=".to_string()));
                                }
                                _ => return Some(TokenType::Operator('<'.to_string())),
                            }
                        }
                        '>' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator(">=".to_string()));
                                }
                                _ => return Some(TokenType::Operator('>'.to_string())),
                            }
                        }
                        '=' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("==".to_string()));
                                }
                                _ => return Some(TokenType::Operator('='.to_string())),
                            }
                        }
                        '!' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('=') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("!=".to_string()));
                                }
                                _ => panic!("Syntax error: expected '=' after the '!'.\n"),
                            }
                        }

                        '&' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('&') => {
                                    self.advance_char();
                                    return Some(TokenType::Operator("&&".to_string()));
                                }
                                _ => panic!("Syntax error: expected a second '&' after the first one.\n"),
                            }
                        }
                        '/' => {
                            self.advance_char();
                            match self.peek_char() {
                                Some('/') => {
                                    self.advance_char();
                                    self.state = State::InLineComment;
                                }
                                Some('*') => {
                                    self.advance_char();
                                    self.state = State::InBlockComment;
                                }
                                _ => return Some(TokenType::Operator('/'.to_string())),
                            }
                        }
                        '0'..='9' => {
                            buffer.push(self.advance_char().unwrap());
                            self.state = State::InNumber;
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            buffer.push(self.advance_char().unwrap());
                            self.state = State::InIdentifier;
                        }
                        other => panic!("Syntax error: unexpected character '{}'.\n", other),
                    }
                },

                State::InIdentifier => match self.peek_char() {
                    Some(character) if character.is_alphanumeric() || character == '_' => {
                        buffer.push(character);
                        self.advance_char();
                    }
                    _ => {
                        self.state = State::Start;
                        return Some(Self::keyword_or_identifier(buffer));
                    }
                },
                State::InDirective => match self.peek_char() {
                    Some(character) if character.is_alphabetic() => {
                        buffer.push(character);
                        self.advance_char();
                    }
                    _ => {
                        self.state = State::Start;
                        return Some(Self::directive_from(&buffer));
                    }
                },

                State::InString => match self.advance_char() {
                    Some('"') => {
                        self.state = State::Start;
                        return Some(TokenType::StringLiteral(buffer));
                    }
                    Some('\\') => {
                        if let Some(esc) = self.advance_char() {
                            match esc {
                                'n' => buffer.push('\n'),
                                't' => buffer.push('\t'),
                                '"' => buffer.push('"'),
                                '\\' => buffer.push('\\'),
                                other => buffer.push(other),
                            }
                        }
                    }
                    Some(character) => buffer.push(character),
                    None => panic!("Syntax error: unterminated string literal.\n"),
                },

                State::InChar => {
                    let character = match self.advance_char() {
                        Some('\\') => match self.advance_char() {
                            Some('n') => '\n',
                            Some('t') => '\t',
                            Some('\'') => '\'',
                            Some('\\') => '\\',
                            Some(other) => other,
                            None => panic!("Syntax error: unterminated char literal.\n"),
                        },
                        Some(character) => character,
                        None => panic!("Syntax error: unterminated char literal.\n"),
                    };
                    match self.advance_char() {
                        Some('\'') => {
                            self.state = State::Start;
                            return Some(TokenType::CharLiteral(character));
                        }
                        _ => panic!("Syntax error: expected closing ' after char literal.\n"),
                    }
                }

                State::InNumber => match self.peek_char() {
                   Some(character) if character.is_ascii_digit() => {
                       buffer.push(character);
                       self.advance_char();
                   }
                   _ => {
                       self.state = State::Start;
                       return Some(TokenType::IntegerLiteral(buffer.parse().unwrap_or(0)));
                   }
                },

                State::InBlockComment => match self.advance_char() {
                    Some('*') if self.peek_char() == Some('/') => {
                        self.advance_char();
                        self.state = State::Start;
                    }
                    Some(_) => {}
                    None => panic!("Syntax Error: Unterminated multiline comment. \nAdd */ at the end to terminate it!\n"),
                },

                State::InLineComment => match self.advance_char() {
                    Some('\n') | None => self.state = State::Start,
                    Some(_) => {},
                },
            }
        }

    }

    fn keyword_or_identifier(id: String) -> TokenType {
        match id.as_str() {
            "int" => TokenType::Int,
            "char" => TokenType::Char,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "default" => TokenType::DefaultCase,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
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
