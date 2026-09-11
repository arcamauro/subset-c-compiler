use compiler::ast::{Expr, Function, Item, Param, Program, Stmt, SwitchCase, Type};
use compiler::lexer::Lexer;
use compiler::parser::Parser;

fn parse(source: &str) -> Program {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    Parser::new(tokens).parse_program()
}

fn parse_body(stmt_source: &str) -> Vec<Stmt> {
    let src = format!("int main() {{ {} }}", stmt_source);
    match parse(&src).items.into_iter().next() {
        Some(Item::Function(f)) => f.body,
        None => panic!("expected a function item"),
    }
}

fn ident(s: &str) -> Expr {
    Expr::Identifier(s.to_string())
}

fn int_lit(v: i32) -> Expr {
    Expr::IntegerLit(v)
}

fn call(name: &str, args: Vec<Expr>) -> Expr {
    Expr::FunCall {
        callee: name.to_string(),
        args,
    }
}

fn bin(left: Expr, op: &str, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: op.to_string(),
        right: Box::new(right),
    }
}

fn unary(op: &str, expr: Expr) -> Expr {
    Expr::UnaryOp {
        op: op.to_string(),
        expr: Box::new(expr),
    }
}

fn assign(target: Expr, op: &str, value: Expr) -> Expr {
    Expr::Assignment {
        target: Box::new(target),
        op: op.to_string(),
        value: Box::new(value),
    }
}

fn expr_stmt(e: Expr) -> Stmt {
    Stmt::Expr(e)
}

#[test]
fn empty_source_produces_empty_program() {
    assert_eq!(parse(""), Program { items: vec![] });
}

#[test]
fn empty_function() {
    assert_eq!(
        parse("int main() {}"),
        Program {
            items: vec![Item::Function(Function {
                ret_type: Type::Int,
                name: "main".to_string(),
                params: vec![],
                body: vec![],
            })],
        }
    );
}

#[test]
fn function_with_single_param() {
    assert_eq!(
        parse("int id(int a) { return a; }"),
        Program {
            items: vec![Item::Function(Function {
                ret_type: Type::Int,
                name: "id".to_string(),
                params: vec![Param {
                    param_type: Type::Int,
                    name: "a".to_string(),
                }],
                body: vec![Stmt::ReturnStmt(Some(ident("a")))],
            })],
        }
    );
}

#[test]
fn function_with_multiple_params() {
    assert_eq!(
        parse("int add(int a, char b) { return a; }"),
        Program {
            items: vec![Item::Function(Function {
                ret_type: Type::Int,
                name: "add".to_string(),
                params: vec![
                    Param {
                        param_type: Type::Int,
                        name: "a".to_string(),
                    },
                    Param {
                        param_type: Type::Char,
                        name: "b".to_string(),
                    },
                ],
                body: vec![Stmt::ReturnStmt(Some(ident("a")))],
            })],
        }
    );
}

#[test]
fn multiple_functions_in_program() {
    let program = parse(
        r#"
        int add(int a, int b) { return a; }
        int main() { return add(1, 2); }
        "#,
    );

    assert_eq!(program.items.len(), 2);
    match &program.items[1] {
        Item::Function(f) => {
            assert_eq!(f.name, "main");
            assert_eq!(
                f.body,
                vec![Stmt::ReturnStmt(Some(call("add", vec![int_lit(1), int_lit(2)])))]
            );
        }
    }
}

#[test]
fn int_declaration_without_initializer() {
    assert_eq!(
        parse_body("int a;"),
        vec![Stmt::VarDeclaration {
            varType: Type::Int,
            name: "a".to_string(),
            value: None,
        }]
    );
}

#[test]
fn int_declaration_with_initializer() {
    assert_eq!(
        parse_body("int a = 10;"),
        vec![Stmt::VarDeclaration {
            varType: Type::Int,
            name: "a".to_string(),
            value: Some(int_lit(10)),
        }]
    );
}

#[test]
fn char_declaration_with_initializer() {
    assert_eq!(
        parse_body("char c = 'x';"),
        vec![Stmt::VarDeclaration {
            varType: Type::Char,
            name: "c".to_string(),
            value: Some(Expr::CharLit('x')),
        }]
    );
}

#[test]
fn return_without_value() {
    assert_eq!(parse_body("return;"), vec![Stmt::ReturnStmt(None)]);
}

#[test]
fn return_with_value() {
    assert_eq!(
        parse_body("return 0;"),
        vec![Stmt::ReturnStmt(Some(int_lit(0)))]
    );
}

#[test]
fn identifier_expression_statement() {
    assert_eq!(parse_body("a;"), vec![expr_stmt(ident("a"))]);
}

#[test]
fn assignment_expression_statement() {
    assert_eq!(
        parse_body("a = 5;"),
        vec![expr_stmt(assign(ident("a"), "=", int_lit(5)))]
    );
}

#[test]
fn function_call_no_args() {
    assert_eq!(parse_body("foo();"), vec![expr_stmt(call("foo", vec![]))]);
}

#[test]
fn function_call_with_args() {
    assert_eq!(
        parse_body("foo(1, x, 2);"),
        vec![expr_stmt(call("foo", vec![int_lit(1), ident("x"), int_lit(2)]))]
    );
}

#[test]
fn nested_function_calls() {
    assert_eq!(
        parse_body("foo(bar(1, 2), 3);"),
        vec![expr_stmt(call(
            "foo",
            vec![call("bar", vec![int_lit(1), int_lit(2)]), int_lit(3)]
        ))]
    );
}

#[test]
fn function_calls_in_binary_expression() {
    assert_eq!(
        parse_body("a = foo(1) + bar(2);"),
        vec![expr_stmt(assign(
            ident("a"),
            "=",
            bin(call("foo", vec![int_lit(1)]), "+", call("bar", vec![int_lit(2)]))
        ))]
    );
}

#[test]
fn compound_assignment_plus() {
    assert_eq!(
        parse_body("a += 1;"),
        vec![expr_stmt(assign(ident("a"), "+=", int_lit(1)))]
    );
}

#[test]
fn compound_assignment_minus() {
    assert_eq!(
        parse_body("a -= 1;"),
        vec![expr_stmt(assign(ident("a"), "-=", int_lit(1)))]
    );
}

#[test]
fn assignment_is_right_associative() {
    assert_eq!(
        parse_body("a = b = 5;"),
        vec![expr_stmt(assign(
            ident("a"),
            "=",
            assign(ident("b"), "=", int_lit(5))
        ))]
    );
}

#[test]
fn string_literal_as_call_argument() {
    assert_eq!(
        parse_body(r#"foo("hi");"#),
        vec![expr_stmt(call("foo", vec![Expr::StringLit("hi".to_string())]))]
    );
}

#[test]
fn char_literal_in_assignment() {
    assert_eq!(
        parse_body("a = 'x';"),
        vec![expr_stmt(assign(ident("a"), "=", Expr::CharLit('x')))]
    );
}

#[test]
fn multiplicative_before_additive() {
    assert_eq!(
        parse_body("return 1 + 2 * 3;"),
        vec![Stmt::ReturnStmt(Some(bin(
            int_lit(1),
            "+",
            bin(int_lit(2), "*", int_lit(3))
        )))]
    );
}

#[test]
fn additive_is_left_associative() {
    assert_eq!(
        parse_body("return 1 - 2 - 3;"),
        vec![Stmt::ReturnStmt(Some(bin(
            bin(int_lit(1), "-", int_lit(2)),
            "-",
            int_lit(3)
        )))]
    );
}

#[test]
fn parentheses_override_precedence() {
    assert_eq!(
        parse_body("return (1 + 2) * 3;"),
        vec![Stmt::ReturnStmt(Some(bin(
            bin(int_lit(1), "+", int_lit(2)),
            "*",
            int_lit(3)
        )))]
    );
}

#[test]
fn relational_binds_tighter_than_equality() {
    assert_eq!(
        parse_body("return a == b < c;"),
        vec![Stmt::ReturnStmt(Some(bin(
            ident("a"),
            "==",
            bin(ident("b"), "<", ident("c"))
        )))]
    );
}

#[test]
fn equality_binds_tighter_than_logical_and() {
    assert_eq!(
        parse_body("return a && b == c;"),
        vec![Stmt::ReturnStmt(Some(bin(
            ident("a"),
            "&&",
            bin(ident("b"), "==", ident("c"))
        )))]
    );
}

#[test]
fn logical_and_is_left_associative() {
    assert_eq!(
        parse_body("return a && b && c;"),
        vec![Stmt::ReturnStmt(Some(bin(
            bin(ident("a"), "&&", ident("b")),
            "&&",
            ident("c")
        )))]
    );
}

#[test]
fn unary_minus_in_additive_expression() {
    assert_eq!(
        parse_body("return a + -b;"),
        vec![Stmt::ReturnStmt(Some(bin(
            ident("a"),
            "+",
            unary("-", ident("b"))
        )))]
    );
}

#[test]
fn prefix_increment() {
    assert_eq!(
        parse_body("++a;"),
        vec![expr_stmt(unary("++", ident("a")))]
    );
}

#[test]
fn postfix_increment() {
    assert_eq!(
        parse_body("a++;"),
        vec![expr_stmt(unary("++", ident("a")))]
    );
}

#[test]
fn postfix_decrement() {
    assert_eq!(
        parse_body("a--;"),
        vec![expr_stmt(unary("--", ident("a")))]
    );
}

#[test]
fn prefix_binds_around_postfix_operand() {
    assert_eq!(
        parse_body("return -a++;"),
        vec![Stmt::ReturnStmt(Some(unary("-", unary("++", ident("a")))))]
    );
}

#[test]
fn if_without_else() {
    assert_eq!(
        parse_body("if (a) { b = 1; }"),
        vec![Stmt::If {
            condition: ident("a"),
            b_then: Box::new(Stmt::Block(vec![expr_stmt(assign(ident("b"), "=", int_lit(1)))])),
            b_else: None,
        }]
    );
}

#[test]
fn if_with_else() {
    assert_eq!(
        parse_body("if (a) { b = 1; } else { b = 2; }"),
        vec![Stmt::If {
            condition: ident("a"),
            b_then: Box::new(Stmt::Block(vec![expr_stmt(assign(ident("b"), "=", int_lit(1)))])),
            b_else: Some(Box::new(Stmt::Block(vec![expr_stmt(assign(
                ident("b"),
                "=",
                int_lit(2)
            ))]))),
        }]
    );
}

#[test]
fn if_with_single_statement_body_has_no_block_wrapper() {
    assert_eq!(
        parse_body("if (a) b = 1;"),
        vec![Stmt::If {
            condition: ident("a"),
            b_then: Box::new(expr_stmt(assign(ident("b"), "=", int_lit(1)))),
            b_else: None,
        }]
    );
}

#[test]
fn if_else_with_single_statement_bodies() {
    assert_eq!(
        parse_body("if (a) b = 1; else b = 2;"),
        vec![Stmt::If {
            condition: ident("a"),
            b_then: Box::new(expr_stmt(assign(ident("b"), "=", int_lit(1)))),
            b_else: Some(Box::new(expr_stmt(assign(ident("b"), "=", int_lit(2))))),
        }]
    );
}

#[test]
fn while_loop_with_block_body() {
    assert_eq!(
        parse_body("while (a < 10) { a++; }"),
        vec![Stmt::While {
            condition: bin(ident("a"), "<", int_lit(10)),
            body: Box::new(Stmt::Block(vec![expr_stmt(unary("++", ident("a")))])),
        }]
    );
}

#[test]
fn while_loop_without_braces() {
    assert_eq!(
        parse_body("while (a) a--;"),
        vec![Stmt::While {
            condition: ident("a"),
            body: Box::new(expr_stmt(unary("--", ident("a")))),
        }]
    );
}

#[test]
fn do_while_loop() {
    assert_eq!(
        parse_body("do { a++; } while (a < 10);"),
        vec![Stmt::DoWhile {
            body: Box::new(Stmt::Block(vec![expr_stmt(unary("++", ident("a")))])),
            condition: bin(ident("a"), "<", int_lit(10)),
        }]
    );
}

#[test]
fn for_loop_full_header() {
    assert_eq!(
        parse_body("for (int i = 0; i < 10; i++) { a++; }"),
        vec![Stmt::For {
            init: Some(Box::new(Stmt::VarDeclaration {
                varType: Type::Int,
                name: "i".to_string(),
                value: Some(int_lit(0)),
            })),
            condition: bin(ident("i"), "<", int_lit(10)),
            update: Some(unary("++", ident("i"))),
            body: Box::new(Stmt::Block(vec![expr_stmt(unary("++", ident("a")))])),
        }]
    );
}

#[test]
fn for_loop_with_empty_init() {
    assert_eq!(
        parse_body("for (; i < 10; i++) {}"),
        vec![Stmt::For {
            init: None,
            condition: bin(ident("i"), "<", int_lit(10)),
            update: Some(unary("++", ident("i"))),
            body: Box::new(Stmt::Block(vec![])),
        }]
    );
}

#[test]
fn for_loop_with_empty_update() {
    assert_eq!(
        parse_body("for (int i = 0; i < 10;) {}"),
        vec![Stmt::For {
            init: Some(Box::new(Stmt::VarDeclaration {
                varType: Type::Int,
                name: "i".to_string(),
                value: Some(int_lit(0)),
            })),
            condition: bin(ident("i"), "<", int_lit(10)),
            update: None,
            body: Box::new(Stmt::Block(vec![])),
        }]
    );
}

#[test]
fn switch_with_case_and_default() {
    assert_eq!(
        parse_body("switch (x) { case 1: a = 1; case 2: a = 2; default: a = 0; }"),
        vec![Stmt::Switch {
            cond: ident("x"),
            cases: vec![
                SwitchCase {
                    value: Some(int_lit(1)),
                    body: vec![expr_stmt(assign(ident("a"), "=", int_lit(1)))],
                },
                SwitchCase {
                    value: Some(int_lit(2)),
                    body: vec![expr_stmt(assign(ident("a"), "=", int_lit(2)))],
                },
                SwitchCase {
                    value: None,
                    body: vec![expr_stmt(assign(ident("a"), "=", int_lit(0)))],
                },
            ],
        }]
    );
}

#[test]
fn switch_case_falls_through_multiple_statements() {
    assert_eq!(
        parse_body("switch (x) { case 1: a = 1; b = 2; case 2: c = 3; }"),
        vec![Stmt::Switch {
            cond: ident("x"),
            cases: vec![
                SwitchCase {
                    value: Some(int_lit(1)),
                    body: vec![
                        expr_stmt(assign(ident("a"), "=", int_lit(1))),
                        expr_stmt(assign(ident("b"), "=", int_lit(2))),
                    ],
                },
                SwitchCase {
                    value: Some(int_lit(2)),
                    body: vec![expr_stmt(assign(ident("c"), "=", int_lit(3)))],
                },
            ],
        }]
    );
}

#[test]
fn switch_with_only_default() {
    assert_eq!(
        parse_body("switch (x) { default: a = 1; }"),
        vec![Stmt::Switch {
            cond: ident("x"),
            cases: vec![SwitchCase {
                value: None,
                body: vec![expr_stmt(assign(ident("a"), "=", int_lit(1)))],
            }],
        }]
    );
}

#[test]
fn switch_with_empty_body() {
    assert_eq!(
        parse_body("switch (x) {}"),
        vec![Stmt::Switch {
            cond: ident("x"),
            cases: vec![],
        }]
    );
}

#[test]
fn switch_case_with_block_and_break() {
    assert_eq!(
        parse_body("switch (x) { case 1: { a = 1; } break; }"),
        vec![Stmt::Switch {
            cond: ident("x"),
            cases: vec![SwitchCase {
                value: Some(int_lit(1)),
                body: vec![
                    Stmt::Block(vec![expr_stmt(assign(ident("a"), "=", int_lit(1)))]),
                    Stmt::Break,
                ],
            }],
        }]
    );
}

#[test]
fn break_statement_alone() {
    assert_eq!(parse_body("break;"), vec![Stmt::Break]);
}

#[test]
fn continue_statement_alone() {
    assert_eq!(parse_body("continue;"), vec![Stmt::Continue]);
}

#[test]
fn break_inside_while_loop() {
    assert_eq!(
        parse_body("while (a) { break; }"),
        vec![Stmt::While {
            condition: ident("a"),
            body: Box::new(Stmt::Block(vec![Stmt::Break])),
        }]
    );
}

#[test]
fn continue_inside_for_loop() {
    assert_eq!(
        parse_body("for (int i = 0; i < 10; i++) { continue; }"),
        vec![Stmt::For {
            init: Some(Box::new(Stmt::VarDeclaration {
                varType: Type::Int,
                name: "i".to_string(),
                value: Some(int_lit(0)),
            })),
            condition: bin(ident("i"), "<", int_lit(10)),
            update: Some(unary("++", ident("i"))),
            body: Box::new(Stmt::Block(vec![Stmt::Continue])),
        }]
    );
}

#[test]
#[should_panic(expected = "expected ';' after 'break'")]
fn break_without_semicolon_panics() {
    parse_body("break");
}

#[test]
#[should_panic(expected = "expected ';' after 'continue'")]
fn continue_without_semicolon_panics() {
    parse_body("continue");
}

#[test]
fn empty_block_as_statement() {
    assert_eq!(parse_body("{}"), vec![Stmt::Block(vec![])]);
}

#[test]
fn block_nested_inside_if_body() {
    assert_eq!(
        parse_body("if (a) { { b = 1; } }"),
        vec![Stmt::If {
            condition: ident("a"),
            b_then: Box::new(Stmt::Block(vec![Stmt::Block(vec![expr_stmt(assign(
                ident("b"),
                "=",
                int_lit(1)
            ))])])),
            b_else: None,
        }]
    );
}

#[test]
fn deeply_nested_empty_blocks() {
    assert_eq!(
        parse_body("{ { {} } }"),
        vec![Stmt::Block(vec![Stmt::Block(vec![Stmt::Block(vec![])])])]
    );
}

#[test]
fn block_as_one_of_several_statements_in_function_body() {
    assert_eq!(
        parse_body("int a; { a = 1; } return a;"),
        vec![
            Stmt::VarDeclaration {
                varType: Type::Int,
                name: "a".to_string(),
                value: None,
            },
            Stmt::Block(vec![expr_stmt(assign(ident("a"), "=", int_lit(1)))]),
            Stmt::ReturnStmt(Some(ident("a"))),
        ]
    );
}

#[test]
#[should_panic(expected = "expected a type")]
fn missing_return_type_panics() {
    parse("main() {}");
}

#[test]
#[should_panic(expected = "expected a function name")]
fn missing_function_name_panics() {
    parse("int () {}");
}

#[test]
#[should_panic(expected = "expected '(' after function name")]
fn missing_open_paren_after_function_name_panics() {
    parse("int main { }");
}

#[test]
#[should_panic(expected = "expected ')' after function parameters")]
fn missing_close_paren_after_params_panics() {
    parse("int main(int a { }");
}

#[test]
#[should_panic(expected = "expected a parameter name")]
fn param_missing_identifier_panics() {
    parse("int foo(int) {}");
}

#[test]
#[should_panic(expected = "expected '{' to start function body")]
fn missing_open_brace_for_function_body_panics() {
    parse("int main()");
}

#[test]
#[should_panic(expected = "expected '}' to close function body")]
fn missing_close_brace_for_function_body_panics() {
    parse("int main() { return 0;");
}

#[test]
#[should_panic(expected = "expected an identifier")]
fn var_declaration_missing_identifier_panics() {
    parse_body("int = 5;");
}

#[test]
#[should_panic(expected = "expected ';' after variable declaration")]
fn missing_semicolon_after_var_declaration_panics() {
    parse_body("int a = 5");
}

#[test]
#[should_panic(expected = "expected ';' after return statement")]
fn missing_semicolon_after_return_panics() {
    parse_body("return 0");
}

#[test]
#[should_panic(expected = "expected ';' after expression")]
fn missing_semicolon_after_expression_panics() {
    parse_body("a = 1");
}

#[test]
#[should_panic(expected = "unexpected token Semicolon, expected an expression")]
fn empty_statement_panics() {
    parse_body(";");
}

#[test]
#[should_panic(expected = "unexpected end of input, expected an expression")]
fn unterminated_expression_at_end_of_input_panics() {
    parse("int main() { return");
}

#[test]
#[should_panic(expected = "expected ')' after function call arguments")]
fn missing_close_paren_in_function_call_panics() {
    parse_body("foo(1, 2;");
}

#[test]
#[should_panic(expected = "expected ')' after if condition")]
fn missing_close_paren_in_if_condition_panics() {
    parse_body("if (a { }");
}

#[test]
#[should_panic(expected = "expected '}' to close block")]
fn missing_close_brace_for_nested_block_panics() {
    parse("int main() { { b = 1;");
}

#[test]
#[should_panic(expected = "expected ':' after case value")]
fn switch_missing_colon_after_case_panics() {
    parse_body("switch (x) { case 1 a = 1; }");
}

#[test]
#[should_panic(expected = "expected '{' to start switch body")]
fn switch_missing_open_brace_panics() {
    parse_body("switch (x) a = 1;");
}

#[test]
#[should_panic(expected = "expected 'case', 'default', or '}' in switch body")]
fn switch_unexpected_token_in_body_panics() {
    parse_body("switch (x) { a = 1; }");
}

#[test]
#[should_panic(expected = "expected 'while' after 'do' body")]
fn do_while_missing_while_keyword_panics() {
    parse_body("do { a = 1; } (a < 10);");
}

#[test]
#[should_panic(expected = "expected ';' after do-while statement")]
fn do_while_missing_semicolon_panics() {
    parse_body("do { a = 1; } while (a < 10)");
}

#[test]
#[should_panic(expected = "expected ';' after for condition")]
fn for_loop_missing_semicolon_after_condition_panics() {
    parse_body("for (int i = 0; i < 10 i++) {}");
}
