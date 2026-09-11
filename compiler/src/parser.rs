use crate::lexer::TokenType;
use crate::ast::{Expr, Function, Item, Param, Program, Stmt, SwitchCase, Type};

#[derive(Debug, PartialEq, Clone)]
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

    fn parse_primary(&mut self) -> Expr {
        self.peek().cloned().map(|token| match token {
            TokenType::IntegerLiteral(value) => {
                self.advance();
                Expr::IntegerLit(value)
            }
            TokenType::CharLiteral(value) => {
                self.advance();
                Expr::CharLit(value)
            }
            TokenType::StringLiteral(value) => {
                self.advance();
                Expr::StringLit(value)
            }
            TokenType::Identifier(name) => {
                self.advance();
                if let Some(TokenType::OpenParentheses) = self.peek() {
                    self.advance();
                    let args = self.parse_call_args();
                    match self.advance() {
                        Some(TokenType::ClosedParentheses) => Expr::FunCall { callee: name, args },
                        other => panic!(
                            "Syntax error: expected ')' after function call arguments, found {:?}.\n",
                            other
                        ),
                    }
                } else {
                    Expr::Identifier(name)
                }
            }
            TokenType::OpenParentheses => {
                self.advance();
                let expr = self.parse_expression();
                match self.advance() {
                    Some(TokenType::ClosedParentheses) => expr,
                    other => panic!("Syntax error: expected ')' but found {:?}.\n", other),
                }
            }
            other => panic!("Syntax error: unexpected token {:?}, expected an expression.\n", other),
        })
        .unwrap_or_else(|| panic!("Syntax error: unexpected end of input, expected an expression.\n"))
    }

    fn parse_call_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if let Some(TokenType::ClosedParentheses) = self.peek() {
            return args;
        }

        args.push(self.parse_expression());

        while let Some(TokenType::Comma) = self.peek() {
            self.advance();
            args.push(self.parse_expression());
        }

        args
    }

    fn parse_expression(&mut self) -> Expr {
        let expr = self.parse_logical_and();

        match self.peek().cloned() {
            Some(TokenType::Operator(op)) if op == "=" || op == "+=" || op == "-=" => {
                self.advance();
                let value = self.parse_expression();
                Expr::Assignment {
                    target: Box::new(expr),
                    op,
                    value: Box::new(value),
                }
            }
            _ => expr,
        }
    }

    fn parse_logical_and(&mut self) -> Expr {
        let mut expr = self.parse_equality();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "&&" {
                break;
            }
            self.advance();
            let right = self.parse_equality();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_relational();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "=" && op != "!=" {
                break;
            }
            self.advance();
            let right = self.parse_relational();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_relational(&mut self) -> Expr {
        let mut expr = self.parse_additive();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "<" && op != "<=" && op != ">" && op != ">=" {
                break;
            }
            self.advance();
            let right = self.parse_additive();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_additive(&mut self) -> Expr {
        let mut expr = self.parse_multiplicative();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "+" && op != "-" {
                break;
            }
            self.advance();
            let right = self.parse_multiplicative();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "*" && op != "/" && op != "%" {
                break;
            }
            self.advance();
            let right = self.parse_unary();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek().cloned() {
            Some(TokenType::Operator(op)) if op == "-" || op == "!" || op == "++" || op == "--" => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while let Some(TokenType::Operator(op)) = self.peek().cloned() {
            if op != "++" && op != "--" {
                break;
            }
            self.advance();
            expr = Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            };
        }

        expr
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::OpenBracket) => self.parse_block(),
            Some(TokenType::Int) | Some(TokenType::Char) => self.parse_var_declaration(),
            Some(TokenType::Return) => self.parse_return_stmt(),
            Some(TokenType::If) => self.parse_if_stmt(),
            Some(TokenType::While) => self.parse_while_stmt(),
            Some(TokenType::Do) => self.parse_do_while_stmt(),
            Some(TokenType::For) => self.parse_for_stmt(),
            Some(TokenType::Switch) => self.parse_switch_stmt(),
            Some(TokenType::Break) => self.parse_break_stmt(),
            Some(TokenType::Continue) => self.parse_continue_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_break_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::Break) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'break', found {:?}.\n", other),
        }

        match self.advance() {
            Some(TokenType::Semicolon) => {}
            other => panic!(
                "Syntax error: expected ';' after 'break', found {:?}.\n",
                other
            ),
        }

        Stmt::Break
    }

    fn parse_continue_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::Continue) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'continue', found {:?}.\n", other),
        }

        match self.advance() {
            Some(TokenType::Semicolon) => {}
            other => panic!(
                "Syntax error: expected ';' after 'continue', found {:?}.\n",
                other
            ),
        }

        Stmt::Continue
    }

    fn parse_type(&mut self) -> Type {
        match self.peek().cloned() {
            Some(TokenType::Int) => {
                self.advance();
                Type::Int
            }
            Some(TokenType::Char) => {
                self.advance();
                Type::Char
            }
            other => panic!("Syntax error: expected a type, found {:?}.\n", other),
        }
    }

    fn parse_var_declaration(&mut self) -> Stmt {
        let var_type = self.parse_type();

        let name = match self.peek().cloned() {
            Some(TokenType::Identifier(name)) => {
                self.advance();
                name
            }
            other => panic!("Syntax error: expected an identifier, found {:?}.\n", other),
        };

        let value = match self.peek().cloned() {
            Some(TokenType::Operator(op)) if op == "=" => {
                self.advance();
                Some(self.parse_expression())
            }
            _ => None,
        };

        match self.advance() {
            Some(TokenType::Semicolon) => {}
            other => panic!(
                "Syntax error: expected ';' after variable declaration, found {:?}.\n",
                other
            ),
        }

        Stmt::VarDeclaration {
            varType: var_type,
            name,
            value,
        }
    }

    fn parse_return_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::Return) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'return', found {:?}.\n", other),
        }

        let value = match self.peek().cloned() {
            Some(TokenType::Semicolon) => None,
            _ => Some(self.parse_expression()),
        };

        match self.advance() {
            Some(TokenType::Semicolon) => {}
            other => panic!(
                "Syntax error: expected ';' after return statement, found {:?}.\n",
                other
            ),
        }

        Stmt::ReturnStmt(value)
    }

    fn parse_block(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::OpenBracket) => {
                self.advance();
                let mut statements = Vec::new();
                while let Some(token) = self.peek() {
                    if *token == TokenType::ClosedBracket {
                        break;
                    }
                    statements.push(self.parse_statement());
                }
                match self.advance() {
                    Some(TokenType::ClosedBracket) => {}
                    other => panic!("Syntax error: expected '}}' to close block, found {:?}.\n", other),
                }
                Stmt::Block(statements)
            }
            _ => self.parse_statement(),
        }
    }

    fn parse_if_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::If) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'if', found {:?}.\n", other),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!("Syntax error: expected '(' after 'if', found {:?}.\n", other),
        }

        let condition = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after if condition, found {:?}.\n",
                other
            ),
        }

        let b_then = Box::new(self.parse_block());

        let b_else = match self.peek().cloned() {
            Some(TokenType::Else) => {
                self.advance();
                Some(Box::new(self.parse_block()))
            }
            _ => None,
        };

        Stmt::If {
            condition,
            b_then,
            b_else,
        }
    }

    fn parse_while_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::While) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'while', found {:?}.\n", other),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!("Syntax error: expected '(' after 'while', found {:?}.\n", other),
        }

        let condition = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after while condition, found {:?}.\n",
                other
            ),
        }

        let body = Box::new(self.parse_block());

        Stmt::While { condition, body }
    }

    fn parse_do_while_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::Do) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'do', found {:?}.\n", other),
        }

        let body = Box::new(self.parse_block());

        match self.peek().cloned() {
            Some(TokenType::While) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected 'while' after 'do' body, found {:?}.\n",
                other
            ),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!("Syntax error: expected '(' after 'while', found {:?}.\n", other),
        }

        let condition = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after do-while condition, found {:?}.\n",
                other
            ),
        }

        match self.peek().cloned() {
            Some(TokenType::Semicolon) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ';' after do-while statement, found {:?}.\n",
                other
            ),
        }

        Stmt::DoWhile { body, condition }
    }

    fn parse_for_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::For) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'for', found {:?}.\n", other),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!("Syntax error: expected '(' after 'for', found {:?}.\n", other),
        }

        let init = match self.peek().cloned() {
            Some(TokenType::Semicolon) => {
                self.advance();
                None
            }
            _ => Some(Box::new(self.parse_statement())),
        };

        let condition = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::Semicolon) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ';' after for condition, found {:?}.\n",
                other
            ),
        }

        let update = match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => None,
            _ => Some(self.parse_expression()),
        };

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after for update, found {:?}.\n",
                other
            ),
        }

        let body = Box::new(self.parse_block());

        Stmt::For {
            init,
            condition,
            update,
            body,
        }
    }

    fn parse_case_body(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                TokenType::Case | TokenType::DefaultCase | TokenType::ClosedBracket => break,
                _ => statements.push(self.parse_statement()),
            }
        }
        statements
    }

    fn parse_switch_stmt(&mut self) -> Stmt {
        match self.peek().cloned() {
            Some(TokenType::Switch) => {
                self.advance();
            }
            other => panic!("Syntax error: expected 'switch', found {:?}.\n", other),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!("Syntax error: expected '(' after 'switch', found {:?}.\n", other),
        }

        let cond = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after switch condition, found {:?}.\n",
                other
            ),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenBracket) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected '{{' to start switch body, found {:?}.\n",
                other
            ),
        }

        let mut cases = Vec::new();
        while let Some(token) = self.peek().cloned() {
            match token {
                TokenType::ClosedBracket => break,
                TokenType::Case => {
                    self.advance();
                    let value = self.parse_expression();
                    match self.peek().cloned() {
                        Some(TokenType::Colon) => {
                            self.advance();
                        }
                        other => panic!(
                            "Syntax error: expected ':' after case value, found {:?}.\n",
                            other
                        ),
                    }
                    let body = self.parse_case_body();
                    cases.push(SwitchCase {
                        value: Some(value),
                        body,
                    });
                }
                TokenType::DefaultCase => {
                    self.advance();
                    match self.peek().cloned() {
                        Some(TokenType::Colon) => {
                            self.advance();
                        }
                        other => panic!(
                            "Syntax error: expected ':' after 'default', found {:?}.\n",
                            other
                        ),
                    }
                    let body = self.parse_case_body();
                    cases.push(SwitchCase { value: None, body });
                }
                other => panic!(
                    "Syntax error: expected 'case', 'default', or '}}' in switch body, found {:?}.\n",
                    other
                ),
            }
        }

        match self.advance() {
            Some(TokenType::ClosedBracket) => {}
            other => panic!(
                "Syntax error: expected '}}' to close switch body, found {:?}.\n",
                other
            ),
        }

        Stmt::Switch { cond, cases }
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expression();

        match self.peek().cloned() {
            Some(TokenType::Semicolon) => {
                self.advance();
            }
            other => panic!("Syntax error: expected ';' after expression, found {:?}.\n", other),
        }

        Stmt::Expr(expr)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();

        while self.peek().is_some() {
            items.push(self.parse_item());
        }

        Program { items }
    }

    fn parse_item(&mut self) -> Item {
        Item::Function(self.parse_function())
    }

    fn parse_function(&mut self) -> Function {
        let ret_type = self.parse_type();

        let name = match self.peek().cloned() {
            Some(TokenType::Identifier(name)) => {
                self.advance();
                name
            }
            other => panic!("Syntax error: expected a function name, found {:?}.\n", other),
        };

        match self.peek().cloned() {
            Some(TokenType::OpenParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected '(' after function name, found {:?}.\n",
                other
            ),
        }

        let params = self.parse_params();

        match self.peek().cloned() {
            Some(TokenType::ClosedParentheses) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected ')' after function parameters, found {:?}.\n",
                other
            ),
        }

        match self.peek().cloned() {
            Some(TokenType::OpenBracket) => {
                self.advance();
            }
            other => panic!(
                "Syntax error: expected '{{' to start function body, found {:?}.\n",
                other
            ),
        }

        let mut body = Vec::new();
        while let Some(token) = self.peek() {
            if *token == TokenType::ClosedBracket {
                break;
            }
            body.push(self.parse_statement());
        }

        match self.advance() {
            Some(TokenType::ClosedBracket) => {}
            other => panic!(
                "Syntax error: expected '}}' to close function body, found {:?}.\n",
                other
            ),
        }

        Function {
            ret_type,
            name,
            params,
            body,
        }
    }

    fn parse_param(&mut self) -> Param {
        let param_type = self.parse_type();

        let name = match self.peek().cloned() {
            Some(TokenType::Identifier(name)) => {
                self.advance();
                name
            }
            other => panic!("Syntax error: expected a parameter name, found {:?}.\n", other),
        };

        Param { param_type, name }
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();

        if let Some(TokenType::ClosedParentheses) = self.peek() {
            return params;
        }

        params.push(self.parse_param());

        while let Some(TokenType::Comma) = self.peek() {
            self.advance();
            params.push(self.parse_param());
        }

        params
    }
}
