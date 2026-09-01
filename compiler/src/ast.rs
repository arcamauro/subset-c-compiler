#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Char,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IntegerLiteral(i32),
    CharLiteral(char),
    StringLiteral(String),
    Identifier(String),
    Unary {
        op: String,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Assign {
        target: Box<Expr>,
        op: String,
        value: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase {
    pub value: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl {
        ty: Type,
        name: String,
        value: Option<Expr>,
    },
    Expr(Expr),
    Return(Option<Expr>),
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        cond: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Expr>,
        step: Option<Expr>,
        body: Box<Stmt>,
    },
    Switch {
        cond: Expr,
        cases: Vec<SwitchCase>,
    },
    Break,
    Continue,
    Block(Vec<Stmt>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Include(String),
    Define {
        name: String,
        value: Option<Expr>,
    },
    Function(Function),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}
