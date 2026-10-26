use compiler::ast::Expr;
use compiler::tac::{Instruction, OpCode, Operand, TacGenerator};

fn assign(name: &str, op: &str, value: Expr) -> Expr {
    Expr::Assignment {
        target: Box::new(Expr::Identifier(name.into())),
        op: op.into(),
        value: Box::new(value),
    }
}

fn binary(left: Expr, op: &str, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: op.into(),
        right: Box::new(right),
    }
}

#[test]
fn literals_and_identifiers_need_no_instructions() {
    for (expr, expected) in [
        (Expr::IntegerLit(42), Operand::Integer(42)),
        (Expr::CharLit('a'), Operand::Char('a')),
        (
            Expr::StringLit("hello".into()),
            Operand::String("hello".into()),
        ),
        (Expr::Identifier("x".into()), Operand::Variable("x".into())),
    ] {
        let mut generator = TacGenerator::default();
        assert_eq!(generator.lower_expr(&expr), expected);
        assert!(generator.instructions.is_empty());
    }
}

#[test]
fn arithmetic_operators_preserve_operand_order() {
    for (operator, opcode) in [
        ("+", OpCode::Add),
        ("-", OpCode::Sub),
        ("*", OpCode::Mul),
        ("/", OpCode::Div),
    ] {
        let mut generator = TacGenerator::default();
        let expr = binary(Expr::IntegerLit(12), operator, Expr::IntegerLit(3));
        assert_eq!(generator.lower_expr(&expr), Operand::Temporary(0));
        assert_eq!(
            generator.instructions,
            vec![Instruction::Binary {
                destination: Operand::Temporary(0),
                op: opcode,
                left: Operand::Integer(12),
                right: Operand::Integer(3),
            }]
        );
    }
}

#[test]
fn parsed_nested_arithmetic_uses_child_result() {
    use compiler::ast::{Item, Stmt};
    use compiler::lexer::Lexer;
    use compiler::parser::Parser;
    use compiler::semantic_analyzer::SemanticAnalyzer;

    let mut lexer = Lexer::new("int main() { return 2 + 3 * 4; }");
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    let program = Parser::new(tokens).parse_program();
    SemanticAnalyzer::new().analyze(&program).unwrap();
    let Item::Function(function) = &program.items[0] else {
        panic!()
    };
    let Stmt::ReturnStmt(Some(expr)) = &function.body[0] else {
        panic!()
    };
    let mut generator = TacGenerator::default();
    assert_eq!(generator.lower_expr(expr), Operand::Temporary(1));
    assert_eq!(
        generator.instructions,
        vec![
            Instruction::Binary {
                destination: Operand::Temporary(0),
                op: OpCode::Mul,
                left: Operand::Integer(3),
                right: Operand::Integer(4),
            },
            Instruction::Binary {
                destination: Operand::Temporary(1),
                op: OpCode::Add,
                left: Operand::Integer(2),
                right: Operand::Temporary(0),
            },
        ]
    );
}

#[test]
fn unary_minus_and_not_emit_correct_operation() {
    for (operator, opcode, left, right) in [
        ("-", OpCode::Sub, Operand::Integer(0), Operand::Integer(5)),
        ("!", OpCode::Equal, Operand::Integer(5), Operand::Integer(0)),
    ] {
        let mut generator = TacGenerator::default();
        let expr = Expr::UnaryOp {
            op: operator.into(),
            expr: Box::new(Expr::IntegerLit(5)),
        };
        assert_eq!(generator.lower_expr(&expr), Operand::Temporary(0));
        assert_eq!(
            generator.instructions,
            vec![Instruction::Binary {
                destination: Operand::Temporary(0),
                op: opcode,
                left,
                right,
            }]
        );
    }
}

#[test]
fn nested_calls_are_emitted_before_the_outer_call() {
    let expr = Expr::FunCall {
        callee: "outer".into(),
        args: vec![Expr::FunCall {
            callee: "inner".into(),
            args: vec![],
        }],
    };
    let mut generator = TacGenerator::default();
    assert_eq!(generator.lower_expr(&expr), Operand::Temporary(1));
    assert_eq!(
        generator.instructions,
        vec![
            Instruction::Call {
                destination: Operand::Temporary(0),
                callee: "inner".into(),
                args: vec![],
            },
            Instruction::Call {
                destination: Operand::Temporary(1),
                callee: "outer".into(),
                args: vec![Operand::Temporary(0)],
            },
        ]
    );
}

#[test]
fn call_saves_argument_before_a_later_argument_changes_it() {
    let expr = Expr::FunCall {
        callee: "f".into(),
        args: vec![
            Expr::Identifier("x".into()),
            assign("x", "=", Expr::IntegerLit(7)),
        ],
    };
    let mut generator = TacGenerator::default();
    assert_eq!(generator.lower_expr(&expr), Operand::Temporary(1));
    assert_eq!(
        generator.instructions,
        vec![
            Instruction::Copy {
                destination: Operand::Temporary(0),
                source: Operand::Variable("x".into()),
            },
            Instruction::Copy {
                destination: Operand::Variable("x".into()),
                source: Operand::Integer(7),
            },
            Instruction::Call {
                destination: Operand::Temporary(1),
                callee: "f".into(),
                args: vec![Operand::Temporary(0), Operand::Integer(7)],
            },
        ]
    );
}

#[test]
fn chained_assignment_copies_to_both_variables() {
    let expression = assign("a", "=", assign("b", "=", Expr::IntegerLit(5)));
    let mut generator = TacGenerator::default();
    assert_eq!(generator.lower_expr(&expression), Operand::Integer(5));
    assert_eq!(
        generator.instructions,
        vec![
            Instruction::Copy {
                destination: Operand::Variable("b".into()),
                source: Operand::Integer(5),
            },
            Instruction::Copy {
                destination: Operand::Variable("a".into()),
                source: Operand::Integer(5),
            },
        ]
    );
}

#[test]
fn compound_assignment_computes_and_stores_the_result() {
    for (operator, opcode) in [("+=", OpCode::Add), ("-=", OpCode::Sub)] {
        let mut generator = TacGenerator::default();
        let result = generator.lower_expr(&assign("x", operator, Expr::IntegerLit(2)));
        assert_eq!(result, Operand::Temporary(1));
        assert_eq!(
            generator.instructions,
            vec![
                Instruction::Copy {
                    destination: Operand::Temporary(0),
                    source: Operand::Variable("x".into()),
                },
                Instruction::Binary {
                    destination: Operand::Temporary(1),
                    op: opcode,
                    left: Operand::Temporary(0),
                    right: Operand::Integer(2),
                },
                Instruction::Copy {
                    destination: Operand::Variable("x".into()),
                    source: Operand::Temporary(1),
                },
            ]
        );
    }
}

#[test]
fn assignment_from_variable_returns_a_saved_value() {
    let mut generator = TacGenerator::default();
    let result = generator.lower_expr(&assign("x", "=", Expr::Identifier("y".into())));
    assert_eq!(result, Operand::Temporary(0));
    assert_eq!(
        generator.instructions,
        vec![
            Instruction::Copy {
                destination: Operand::Temporary(0),
                source: Operand::Variable("y".into()),
            },
            Instruction::Copy {
                destination: Operand::Variable("x".into()),
                source: Operand::Temporary(0),
            },
        ]
    );
}

fn lower_body(source: &str) -> Vec<Instruction> {
    use compiler::ast::Item;
    use compiler::lexer::Lexer;
    use compiler::parser::Parser;
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }
    let program = Parser::new(tokens).parse_program();
    let Item::Function(function) = &program.items[0] else {
        panic!()
    };
    let mut generator = TacGenerator::default();
    for statement in &function.body {
        generator.lower_stmt(statement);
    }
    generator.instructions
}

#[test]
fn while_rechecks_condition_and_exits_before_following_code() {
    let code = lower_body("int main() { while (x < 10) { x += 1; } return x; }");
    assert_eq!(code[0], Instruction::Label(0));
    assert_eq!(
        code[1],
        Instruction::Binary {
            destination: Operand::Temporary(0),
            op: OpCode::LessThan,
            left: Operand::Variable("x".into()),
            right: Operand::Integer(10),
        }
    );
    assert_eq!(
        code[2],
        Instruction::IfFalseGoto {
            condition: Operand::Temporary(0),
            label: 1
        }
    );
    assert_eq!(
        &code[code.len() - 3..],
        &[
            Instruction::Goto(0),
            Instruction::Label(1),
            Instruction::Return(Some(Operand::Variable("x".into()))),
        ]
    );
}

#[test]
fn for_continue_runs_update_and_break_exits() {
    let code =
        lower_body("int main() { for (int x = 0; x < 3; x += 1) { continue; break; } return 0; }");
    assert_eq!(code[4], Instruction::Goto(1));
    assert_eq!(code[5], Instruction::Goto(2));
    assert_eq!(code[6], Instruction::Label(1));
    assert_eq!(
        &code[code.len() - 3..],
        &[
            Instruction::Goto(0),
            Instruction::Label(2),
            Instruction::Return(Some(Operand::Integer(0))),
        ]
    );
}

#[test]
fn do_while_continue_targets_condition_after_body() {
    let code = lower_body("int main() { do { continue; } while (x != 0); return 0; }");
    assert_eq!(code[0], Instruction::Label(0));
    assert_eq!(code[1], Instruction::Goto(1));
    assert_eq!(code[2], Instruction::Label(1));
    assert_eq!(
        code[4],
        Instruction::IfFalseGoto {
            condition: Operand::Temporary(0),
            label: 2
        }
    );
    assert_eq!(code[5], Instruction::Goto(0));
}

#[test]
fn switch_break_and_continue_use_different_enclosing_targets() {
    let code = lower_body(
        "int main() { while (1) { switch (x) { case 1: continue; default: break; } break; } return 0; }",
    );
    assert!(
        code.windows(2)
            .any(|pair| pair == [Instruction::Label(3), Instruction::Goto(0)])
    );
    assert!(
        code.windows(2)
            .any(|pair| pair == [Instruction::Label(4), Instruction::Goto(2)])
    );
    assert!(
        code.windows(2)
            .any(|pair| pair == [Instruction::Label(2), Instruction::Goto(1)])
    );
}

#[test]
fn switch_cases_fall_through_and_default_is_dispatch_fallback() {
    let code = lower_body("int main() { switch (x) { default: x = 3; case 1: x = 4; } return x; }");
    assert_eq!(code[3], Instruction::Goto(1));
    assert_eq!(
        &code[4..9],
        &[
            Instruction::Label(1),
            Instruction::Copy {
                destination: Operand::Variable("x".into()),
                source: Operand::Integer(3)
            },
            Instruction::Label(2),
            Instruction::Copy {
                destination: Operand::Variable("x".into()),
                source: Operand::Integer(4)
            },
            Instruction::Label(0),
        ]
    );
}

#[test]
fn if_else_skips_the_other_branch() {
    let code = lower_body("int main() { if (x) { return 1; } else { return 2; } }");
    assert_eq!(
        code,
        vec![
            Instruction::IfFalseGoto {
                condition: Operand::Variable("x".into()),
                label: 0
            },
            Instruction::Return(Some(Operand::Integer(1))),
            Instruction::Goto(1),
            Instruction::Label(0),
            Instruction::Return(Some(Operand::Integer(2))),
            Instruction::Label(1),
        ]
    );
}

#[test]
fn logical_and_skips_rhs_side_effects_when_left_is_false() {
    let code = lower_body("int main() { if (0 && f()) { return 1; } return 0; }");
    let Instruction::IfFalseGoto {
        condition: Operand::Integer(0),
        label,
    } = code[1]
    else {
        panic!()
    };
    let call = code
        .iter()
        .position(|i| matches!(i, Instruction::Call { .. }))
        .unwrap();
    let end = code
        .iter()
        .position(|i| *i == Instruction::Label(label))
        .unwrap();
    assert!(call > 1 && call < end);
}
