#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Char,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IntegerLit(i32),
    CharLit(char),
    StringLit(String),
    Identifier(String),
        Assignment {
        target: Box<Expr>,
        op: String,
        value: Box<Expr>
    },
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>
    },
    UnaryOp {
        op:String,
        expr: Box<Expr>
    },
    FunCall {
        callee: String,
        args: Vec<Expr>
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDeclaration {
        varType: Type,
        name: String,
        value: Option<Expr>
    },
    ReturnStmt(Option<Expr>),
    Expr(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        b_then: Box<Stmt>,
        b_else: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    },
    DoWhile {
        body: Box<Stmt>,
        condition: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Expr,
        update: Option<Expr>,
        body: Box<Stmt>,
    },
    Switch {
        cond: Expr,
        cases: Vec<SwitchCase>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Function(Function)
}


#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub items: Vec<Item>
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase {
    pub value: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub param_type: Type,
    pub name: String,
}
