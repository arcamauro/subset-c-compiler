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

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<TokenType> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.scan_token() {
            tokens.push(token);
        }
        tokens
    }

    fn op(s: &str) -> TokenType {
        TokenType::Operator(s.to_string())
    }

    #[test]
    fn line_comment_produces_no_tokens() {
        assert_eq!(tokenize("// this is a comment"), vec![]);
    }

    #[test]
    fn block_comment_produces_no_tokens() {
        assert_eq!(tokenize("/* this is a comment */"), vec![]);
    }

    #[test]
    fn comment_is_skipped_between_real_tokens() {
        assert_eq!(
            tokenize("int a; // comment\n int b;"),
            vec![
                TokenType::Int,
                TokenType::Identifier("a".to_string()),
                TokenType::Semicolon,
                TokenType::Int,
                TokenType::Identifier("b".to_string()),
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn int_declaration_with_assignment() {
        assert_eq!(
            tokenize("int a = 10;"),
            vec![
                TokenType::Int,
                TokenType::Identifier("a".to_string()),
                op("="),
                TokenType::IntegerLiteral(10),
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn char_declaration_with_assignment() {
        assert_eq!(
            tokenize("char b = 'a';"),
            vec![
                TokenType::Char,
                TokenType::Identifier("b".to_string()),
                op("="),
                TokenType::CharLiteral('a'),
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn char_literal_with_escape_sequence() {
        assert_eq!(tokenize("'\\n'"), vec![TokenType::CharLiteral('\n')]);
    }

    #[test]
    fn for_loop_header() {
        assert_eq!(
            tokenize("for (int i = 0; i < 10; i++)"),
            vec![
                TokenType::For,
                TokenType::OpenParentheses,
                TokenType::Int,
                TokenType::Identifier("i".to_string()),
                op("="),
                TokenType::IntegerLiteral(0),
                TokenType::Semicolon,
                TokenType::Identifier("i".to_string()),
                op("<"),
                TokenType::IntegerLiteral(10),
                TokenType::Semicolon,
                TokenType::Identifier("i".to_string()),
                op("++"),
                TokenType::ClosedParentheses,
            ]
        );
    }

    #[test]
    fn while_loop_with_decrement() {
        assert_eq!(
            tokenize("while (i > 0) { i-- }"),
            vec![
                TokenType::While,
                TokenType::OpenParentheses,
                TokenType::Identifier("i".to_string()),
                op(">"),
                TokenType::IntegerLiteral(0),
                TokenType::ClosedParentheses,
                TokenType::OpenBracket,
                TokenType::Identifier("i".to_string()),
                op("--"),
                TokenType::ClosedBracket,
            ]
        );
    }

    #[test]
    fn keyword_return() {
        assert_eq!(tokenize("return"), vec![TokenType::Return]);
    }

    #[test]
    fn keyword_if() {
        assert_eq!(tokenize("if"), vec![TokenType::If]);
    }

    #[test]
    fn keyword_else() {
        assert_eq!(tokenize("else"), vec![TokenType::Else]);
    }

    #[test]
    fn keyword_do() {
        assert_eq!(
            tokenize("do { i++; } while (i < 10);"),
            vec![
                TokenType::Do,
                TokenType::OpenBracket,
                TokenType::Identifier("i".to_string()),
                op("++"),
                TokenType::Semicolon,
                TokenType::ClosedBracket,
                TokenType::While,
                TokenType::OpenParentheses,
                TokenType::Identifier("i".to_string()),
                op("<"),
                TokenType::IntegerLiteral(10),
                TokenType::ClosedParentheses,
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn keyword_switch() {
        assert_eq!(tokenize("switch"), vec![TokenType::Switch]);
    }

    #[test]
    fn keyword_default() {
        assert_eq!(tokenize("default"), vec![TokenType::DefaultCase]);
    }

    #[test]
    fn keyword_case() {
        assert_eq!(tokenize("case"), vec![TokenType::Case]);
    }

    #[test]
    fn keyword_break() {
        assert_eq!(tokenize("break"), vec![TokenType::Break]);
    }

    #[test]
    fn keyword_continue() {
        assert_eq!(tokenize("continue"), vec![TokenType::Continue]);
    }

    #[test]
    fn switch_case_default_block() {
        assert_eq!(
            tokenize("switch (x) { case 1: break; default: continue; }"),
            vec![
                TokenType::Switch,
                TokenType::OpenParentheses,
                TokenType::Identifier("x".to_string()),
                TokenType::ClosedParentheses,
                TokenType::OpenBracket,
                TokenType::Case,
                TokenType::IntegerLiteral(1),
                TokenType::Colon,
                TokenType::Break,
                TokenType::Semicolon,
                TokenType::DefaultCase,
                TokenType::Colon,
                TokenType::Continue,
                TokenType::Semicolon,
                TokenType::ClosedBracket,
            ]
        );
    }

    #[test]
    fn colon() {
        assert_eq!(tokenize(":"), vec![TokenType::Colon]);
    }

    #[test]
    fn directive_include() {
        assert_eq!(tokenize("#include"), vec![TokenType::Include]);
    }

    #[test]
    fn directive_define() {
        assert_eq!(tokenize("#define"), vec![TokenType::Define]);
    }

    #[test]
    fn string_literal() {
        assert_eq!(
            tokenize("\"hello\""),
            vec![TokenType::StringLiteral("hello".to_string())]
        );
    }

    #[test]
    fn string_literal_with_escape_sequence() {
        assert_eq!(
            tokenize("\"a\\nb\""),
            vec![TokenType::StringLiteral("a\nb".to_string())]
        );
    }

    #[test]
    fn identifier_alone() {
        assert_eq!(
            tokenize("my_var1"),
            vec![TokenType::Identifier("my_var1".to_string())]
        );
    }

    #[test]
    fn open_and_closed_parentheses() {
        assert_eq!(
            tokenize("()"),
            vec![TokenType::OpenParentheses, TokenType::ClosedParentheses]
        );
    }

    #[test]
    fn open_and_closed_bracket() {
        assert_eq!(
            tokenize("{}"),
            vec![TokenType::OpenBracket, TokenType::ClosedBracket]
        );
    }

    #[test]
    fn semicolon() {
        assert_eq!(tokenize(";"), vec![TokenType::Semicolon]);
    }

    #[test]
    fn operator_modulo() {
        assert_eq!(tokenize("%"), vec![op("%")]);
    }

    #[test]
    fn operator_multiplication() {
        assert_eq!(tokenize("*"), vec![op("*")]);
    }

    #[test]
    fn multiplication_does_not_break_block_comments() {
        assert_eq!(
            tokenize("a * b; /* comment */ c * d;"),
            vec![
                TokenType::Identifier("a".to_string()),
                op("*"),
                TokenType::Identifier("b".to_string()),
                TokenType::Semicolon,
                TokenType::Identifier("c".to_string()),
                op("*"),
                TokenType::Identifier("d".to_string()),
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    fn operator_equality() {
        assert_eq!(tokenize("=="), vec![op("==")]);
    }

    #[test]
    fn operator_inequality() {
        assert_eq!(tokenize("!="), vec![op("!=")]);
    }

    #[test]
    fn operator_logical_and() {
        assert_eq!(tokenize("&&"), vec![op("&&")]);
    }

    #[test]
    fn operator_division() {
        assert_eq!(tokenize("/"), vec![op("/")]);
    }

    #[test]
    fn operator_less_or_equal() {
        assert_eq!(tokenize("<="), vec![op("<=")]);
    }

    #[test]
    fn operator_greater_or_equal() {
        assert_eq!(tokenize(">="), vec![op(">=")]);
    }

    #[test]
    fn operator_plus() {
        assert_eq!(tokenize("+"), vec![op("+")]);
    }

    #[test]
    fn operator_minus() {
        assert_eq!(tokenize("-"), vec![op("-")]);
    }

    #[test]
    fn operator_plus_assign() {
        assert_eq!(tokenize("+="), vec![op("+=")]);
    }

    #[test]
    fn operator_minus_assign() {
        assert_eq!(tokenize("-="), vec![op("-=")]);
    }

    #[test]
    fn compound_assignment_in_context() {
        assert_eq!(
            tokenize("x += 1; y -= 2;"),
            vec![
                TokenType::Identifier("x".to_string()),
                op("+="),
                TokenType::IntegerLiteral(1),
                TokenType::Semicolon,
                TokenType::Identifier("y".to_string()),
                op("-="),
                TokenType::IntegerLiteral(2),
                TokenType::Semicolon,
            ]
        );
    }

    #[test]
    #[should_panic(expected = "unexpected character")]
    fn unexpected_character_panics() {
        tokenize("@");
    }

    #[test]
    #[should_panic(expected = "expected '=' after the '!'")]
    fn lone_bang_panics() {
        tokenize("!a");
    }

    #[test]
    #[should_panic(expected = "expected a second '&' after the first one")]
    fn lone_ampersand_panics() {
        tokenize("&a");
    }

    #[test]
    #[should_panic(expected = "unterminated string literal")]
    fn unterminated_string_literal_panics() {
        tokenize("\"hello");
    }

    #[test]
    #[should_panic(expected = "expected closing '")]
    fn unterminated_char_literal_panics() {
        tokenize("'a");
    }

    #[test]
    #[should_panic(expected = "unterminated char literal")]
    fn empty_char_literal_panics() {
        tokenize("'");
    }

    #[test]
    #[should_panic(expected = "expected closing '")]
    fn char_literal_missing_closing_quote_panics() {
        tokenize("'ab'");
    }

    #[test]
    #[should_panic(expected = "Unterminated multiline comment")]
    fn unterminated_block_comment_panics() {
        tokenize("/* this comment never ends");
    }

    #[test]
    #[should_panic(expected = "directive is not supported")]
    fn unsupported_directive_panics() {
        tokenize("#pragma");
    }
}
