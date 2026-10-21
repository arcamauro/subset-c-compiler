use crate::ast::{Expr, Stmt};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
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

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::Equal => "==",
            Self::NotEqual => "!=",
        })
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Char(value) => write!(f, "{value:?}"),
            Self::String(value) => write!(f, "{value:?}"),
            Self::Variable(name) => f.write_str(name),
            Self::Temporary(id) => write!(f, "t{id}"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Copy {
                destination,
                source,
            } => write!(f, "{destination} = {source}"),
            Self::Binary {
                destination,
                op,
                left,
                right,
            } => {
                write!(f, "{destination} = {left} {op} {right}")
            }
            Self::Label(label) => write!(f, "L{label}:"),
            Self::Goto(label) => write!(f, "goto L{label}"),
            Self::IfFalseGoto { condition, label } => {
                write!(f, "ifFalse {condition} goto L{label}")
            }
            Self::Call {
                destination,
                callee,
                args,
            } => {
                write!(f, "{destination} = call {callee}(")?;
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                f.write_str(")")
            }
            Self::Return(Some(value)) => write!(f, "return {value}"),
            Self::Return(None) => f.write_str("return"),
        }
    }
}

#[derive(Default)]
pub struct TacGenerator {
    pub instructions: Vec<Instruction>,
    next_temp: u32,
    next_label: u32,
    break_labels: Vec<u32>,
    continue_labels: Vec<u32>,
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
                if op == "&&" || op == "||" {
                    let destination = Operand::Temporary(self.next_temp);
                    self.next_temp += 1;
                    let next = self.new_label();
                    let end = self.new_label();
                    let left = self.lower_expr(left);
                    self.instructions.push(Instruction::Copy {
                        destination: destination.clone(),
                        source: Operand::Integer(if op == "&&" { 0 } else { 1 }),
                    });
                    self.instructions.push(Instruction::IfFalseGoto {
                        condition: left,
                        label: if op == "&&" { end } else { next },
                    });
                    if op == "||" {
                        self.instructions.push(Instruction::Goto(end));
                        self.instructions.push(Instruction::Label(next));
                    }
                    let right = self.lower_expr(right);
                    self.instructions.push(Instruction::Binary {
                        destination: destination.clone(),
                        op: OpCode::NotEqual,
                        left: right,
                        right: Operand::Integer(0),
                    });
                    self.instructions.push(Instruction::Label(end));
                    return destination;
                }
                let left_operand = self.lower_expr(left);
                let right_operand = self.lower_expr(right);
                let op_code = match op.as_str() {
                    "+" => OpCode::Add,
                    "-" => OpCode::Sub,
                    "*" => OpCode::Mul,
                    "/" => OpCode::Div,
                    "%" => OpCode::Mod,
                    "<" => OpCode::LessThan,
                    "<=" => OpCode::LessThanOrEqual,
                    ">" => OpCode::GreaterThan,
                    ">=" => OpCode::GreaterThanOrEqual,
                    "==" => OpCode::Equal,
                    "!=" => OpCode::NotEqual,
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

    pub fn lower_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDeclaration { name, value, .. } => {
                if let Some(value) = value {
                    let source = self.lower_expr(value);
                    self.instructions.push(Instruction::Copy {
                        destination: Operand::Variable(name.clone()),
                        source,
                    });
                }
            }
            Stmt::ReturnStmt(value) => {
                let value = value.as_ref().map(|expr| self.lower_expr(expr));
                self.instructions.push(Instruction::Return(value));
            }
            Stmt::Expr(expr) => {
                self.lower_expr(expr);
            }
            Stmt::Block(statements) => {
                for statement in statements {
                    self.lower_stmt(statement);
                }
            }
            Stmt::If {
                condition,
                b_then,
                b_else,
            } => {
                let otherwise = self.new_label();
                let condition = self.lower_expr(condition);
                self.instructions.push(Instruction::IfFalseGoto {
                    condition,
                    label: otherwise,
                });
                self.lower_stmt(b_then);
                if let Some(b_else) = b_else {
                    let end = self.new_label();
                    self.instructions.push(Instruction::Goto(end));
                    self.instructions.push(Instruction::Label(otherwise));
                    self.lower_stmt(b_else);
                    self.instructions.push(Instruction::Label(end));
                } else {
                    self.instructions.push(Instruction::Label(otherwise));
                }
            }
            Stmt::While { condition, body } => {
                let start = self.new_label();
                let end = self.new_label();
                self.instructions.push(Instruction::Label(start));
                let condition = self.lower_expr(condition);
                self.instructions.push(Instruction::IfFalseGoto {
                    condition,
                    label: end,
                });
                self.lower_loop_body(body, end, start);
                self.instructions.push(Instruction::Goto(start));
                self.instructions.push(Instruction::Label(end));
            }
            Stmt::DoWhile { body, condition } => {
                let start = self.new_label();
                let test = self.new_label();
                let end = self.new_label();
                self.instructions.push(Instruction::Label(start));
                self.lower_loop_body(body, end, test);
                self.instructions.push(Instruction::Label(test));
                let condition = self.lower_expr(condition);
                self.instructions.push(Instruction::IfFalseGoto {
                    condition,
                    label: end,
                });
                self.instructions.push(Instruction::Goto(start));
                self.instructions.push(Instruction::Label(end));
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    self.lower_stmt(init);
                }
                let start = self.new_label();
                let step = self.new_label();
                let end = self.new_label();
                self.instructions.push(Instruction::Label(start));
                let condition = self.lower_expr(condition);
                self.instructions.push(Instruction::IfFalseGoto {
                    condition,
                    label: end,
                });
                self.lower_loop_body(body, end, step);
                self.instructions.push(Instruction::Label(step));
                if let Some(update) = update {
                    self.lower_expr(update);
                }
                self.instructions.push(Instruction::Goto(start));
                self.instructions.push(Instruction::Label(end));
            }
            Stmt::Switch { cond, cases } => {
                let end = self.new_label();
                let labels: Vec<_> = cases.iter().map(|_| self.new_label()).collect();
                let source = self.lower_expr(cond);
                let value = Operand::Temporary(self.next_temp);
                self.next_temp += 1;
                self.instructions.push(Instruction::Copy {
                    destination: value.clone(),
                    source,
                });
                let mut default = end;
                for (case, &label) in cases.iter().zip(&labels) {
                    if let Some(expr) = &case.value {
                        let right = self.lower_expr(expr);
                        let comparison = Operand::Temporary(self.next_temp);
                        self.next_temp += 1;
                        self.instructions.push(Instruction::Binary {
                            destination: comparison.clone(),
                            op: OpCode::NotEqual,
                            left: value.clone(),
                            right,
                        });
                        self.instructions.push(Instruction::IfFalseGoto {
                            condition: comparison,
                            label,
                        });
                    } else {
                        default = label;
                    }
                }
                self.instructions.push(Instruction::Goto(default));
                self.break_labels.push(end);
                for (case, label) in cases.iter().zip(labels) {
                    self.instructions.push(Instruction::Label(label));
                    for statement in &case.body {
                        self.lower_stmt(statement);
                    }
                }
                self.break_labels.pop();
                self.instructions.push(Instruction::Label(end));
            }
            Stmt::Break => {
                let label = *self
                    .break_labels
                    .last()
                    .expect("break outside a loop or switch");
                self.instructions.push(Instruction::Goto(label));
            }
            Stmt::Continue => {
                let label = *self
                    .continue_labels
                    .last()
                    .expect("continue outside a loop");
                self.instructions.push(Instruction::Goto(label));
            }
        }
    }

    fn new_label(&mut self) -> u32 {
        let label = self.next_label;
        self.next_label += 1;
        label
    }

    fn lower_loop_body(&mut self, body: &Stmt, end: u32, next: u32) {
        self.break_labels.push(end);
        self.continue_labels.push(next);
        self.lower_stmt(body);
        self.continue_labels.pop();
        self.break_labels.pop();
    }
}
