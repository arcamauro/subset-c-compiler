use crate::ast::{Expr, Stmt};

#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Integer(i32),
    Char(char),
    String(String),
    Variable(String),
    Temporary(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Copy {
        destination: Operand,
        source: Operand,
    },
    Binary {
        destination: Operand,
        op: OpCode,
        left: Operand,
        right: Operand,
    },
    Label(u32),
    Goto(u32),
    IfFalseGoto {
        condition: Operand,
        label: u32,
    },
    Call {
        destination: Operand,
        callee: String,
        args: Vec<Operand>,
    },
    Return(Option<Operand>),
}

#[derive(Default)]
pub struct TacGenerator {
    pub instructions: Vec<Instruction>,
    next_temp: u32,
    next_label: u32,
}

impl TacGenerator {
    pub fn lower_expr(&mut self, expr: &Expr) -> Operand {
        match expr {
            Expr::IntegerLit(value) => Operand::Integer(*value),
            Expr::CharLit(value) => Operand::Char(*value),
            Expr::StringLit(value) => Operand::String(value.clone()),
            Expr::Identifier(name) => Operand::Variable(name.clone()),
            Expr::UnaryOp { op, expr } => {
                let operand = self.lower_expr(expr);
                match op.as_str() {
                    "+" => operand,
                    "-" => {
                        let destination = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: destination.clone(),
                            op: OpCode::Sub,
                            left: Operand::Integer(0),
                            right: operand,
                        });

                        destination
                    }
                    "!" => {
                        let destination = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: destination.clone(),
                            op: OpCode::Equal,
                            left: operand,
                            right: Operand::Integer(0),
                        });
                        destination
                    }
                    "++" => {
                        let destination = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: destination.clone(),
                            op: OpCode::Add,
                            left: operand.clone(),
                            right: Operand::Integer(1),
                        });
                        self.instructions.push(Instruction::Copy {
                            destination: operand,
                            source: destination.clone(),
                        });
                        destination
                    }
                    "--" => {
                        let destination = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: destination.clone(),
                            op: OpCode::Sub,
                            left: operand.clone(),
                            right: Operand::Integer(1),
                        });
                        self.instructions.push(Instruction::Copy {
                            destination: operand,
                            source: destination.clone(),
                        });
                        destination
                    }

                    _ => panic!("Unary operator not supported: {}", op),
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_operand = self.lower_expr(left);
                let right_operand = self.lower_expr(right);
                let op_code = match op.as_str() {
                    "+" => OpCode::Add,
                    "-" => OpCode::Sub,
                    "*" => OpCode::Mul,
                    "/" => OpCode::Div,
                    _ => panic!("Binary Operator not handled: {}", op),
                };
                let destination = Operand::Temporary(self.next_temp);
                self.next_temp += 1;
                self.instructions.push(Instruction::Binary {
                    destination: destination.clone(),
                    op: op_code,
                    left: left_operand,
                    right: right_operand,
                });
                destination
            }
            Expr::Assignment { target, op, value } => {
                let target = match target.as_ref() {
                    Expr::Identifier(name) => Operand::Variable(name.clone()),
                    _ => panic!("A value can be assigned only to a variable"),
                };

                let result = match op.as_str() {
                    "=" => {
                        let source = self.lower_expr(value);
                        if matches!(&source, Operand::Variable(_)) {
                            let saved = Operand::Temporary(self.next_temp);
                            self.next_temp += 1;
                            self.instructions.push(Instruction::Copy {
                                destination: saved.clone(),
                                source,
                            });
                            saved
                        } else {
                            source
                        }
                    }
                    "+=" | "-=" => {
                        let previous = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Copy {
                            destination: previous.clone(),
                            source: target.clone(),
                        });
                        let source = self.lower_expr(value);
                        let result = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: result.clone(),
                            op: if op == "+=" { OpCode::Add } else { OpCode::Sub },
                            left: previous,
                            right: source,
                        });
                        result
                    }
                    _ => todo!("Assignment operator not supported: {}", op),
                };

                self.instructions.push(Instruction::Copy {
                    destination: target,
                    source: result.clone(),
                });
                result
            }
            Expr::FunCall { callee, args } => {
                let mut arguments = Vec::new();

                for arg in args {
                    let operand = self.lower_expr(arg);
                    let saved = if matches!(&operand, Operand::Variable(_)) {
                        let temporary = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Copy {
                            destination: temporary.clone(),
                            source: operand,
                        });
                        temporary
                    } else {
                        operand
                    };
                    arguments.push(saved);
                }

                let destination = Operand::Temporary(self.next_temp);
                self.next_temp += 1;

                self.instructions.push(Instruction::Call {
                    destination: destination.clone(),
                    callee: callee.clone(),
                    args: arguments,
                });

                destination
            }
        }
    }

    pub fn lower_stmt(&mut self, stmt: &Stmt) {}
}
