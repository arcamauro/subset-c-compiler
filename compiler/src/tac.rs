#[derive(Debug, PartialEq, Clone)]
pub enum OpCode{
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
    Return(Option<Operand>),
}
