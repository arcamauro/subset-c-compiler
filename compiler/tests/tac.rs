use compiler::ast::Expr;
use compiler::tac::{Instruction, OpCode, Operand, TacGenerator};

fn assign(name: &str, op: &str, value: Expr) -> Expr {
    Expr::Assignment {
        target: Box::new(Expr::Identifier(name.into())),
        op: op.into(),
        value: Box::new(value),
    }
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
