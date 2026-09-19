use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Char,
    String,
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
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Function(Function),
    FunctionDecl(FunctionDecl),
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

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDecl {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecoratedExpr {
    pub expr: Expr,
    pub inferred_type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecoratedStmt {
    pub stmt: Stmt,
    pub inferred_type: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecoratedFunction {
    pub function: Function,
    pub body: Vec<DecoratedStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DecoratedItem {
    Function(DecoratedFunction),
    FunctionDecl(FunctionDecl),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DecoratedProgram {
    pub items: Vec<DecoratedItem>,
    pub symbol_table: HashMap<String, Type>,
}
