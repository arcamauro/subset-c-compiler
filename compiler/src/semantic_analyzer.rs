use std::collections::HashMap;
use crate::ast::{Program, Item, Function, Param, Type, Expr, Stmt};

#[derive(Debug, PartialEq, Clone)]
struct Symbol {
    name: String,
    ty: Type,
    kind: Kind,
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Variable,
    Function {
        params: Vec<Type>,
        return_value: Type,
    },
    Parameter,
}

pub struct SemanticAnalyzer {
    errors: Vec<String>,
    scopes: Vec<HashMap<String, Symbol>>,
    current_return_type: Option<Type>,
    loop_depth: u32,
    switch_depth: u32,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            scopes: vec![HashMap::new()],
            current_return_type: None,
            loop_depth: 0,
            switch_depth: 0,
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }

    fn declare(&mut self, name: String, symbol: Symbol) {
        let current = self.scopes.last_mut().unwrap();
        if current.contains_key(&name) {
            self.errors.push(format!("Redeclaration of '{}' in same scope", name));
        } else {
            current.insert(name, symbol);
        }
    }

    fn declare_function_signature(&mut self, name: &str, ret_type: &Type, params: &[Param]) {
        self.declare(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                ty: ret_type.clone(),
                kind: Kind::Function {
                    params: params.iter().map(|p| p.param_type.clone()).collect(),
                    return_value: ret_type.clone(),
                },
            },
        );
    }

    pub fn analyze(&mut self, program: &Program) -> Result<Program, Vec<String>> {
        self.errors.clear();
        self.scopes.clear();
        self.scopes.push(HashMap::new());
        self.current_return_type = None;
        self.loop_depth = 0;
        self.switch_depth = 0;

        for item in &program.items {
            match item {
                Item::Function(func) => {
                    self.declare_function_signature(&func.name, &func.ret_type, &func.params);
                }
                Item::FunctionDecl(decl) => {
                    self.declare_function_signature(&decl.name, &decl.ret_type, &decl.params);
                }
            }
        }

        for item in &program.items {
            match item {
                Item::Function(func) => self.check_function(func),
                Item::FunctionDecl(_) => {}
            }
        }

        if self.errors.is_empty() {
            Ok(program.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_function(&mut self, func: &Function) {
        self.push_scope();
        self.current_return_type = Some(func.ret_type.clone());

        for param in &func.params {
            self.declare(
                param.name.clone(),
                Symbol {
                    name: param.name.clone(),
                    ty: param.param_type.clone(),
                    kind: Kind::Parameter,
                },
            );
        }

        for stmt in &func.body {
            self.check_stmt(stmt);
        }

        if !func.body.iter().any(Self::stmt_always_returns) {
            self.error(format!(
                "Function '{}' may not return a value on all paths",
                func.name
            ));
        }

        self.current_return_type = None;
        self.pop_scope();
    }

    fn stmt_always_returns(stmt: &Stmt) -> bool {
        match stmt {
            Stmt::ReturnStmt(_) => true,
            Stmt::Block(stmts) => stmts.iter().any(Self::stmt_always_returns),
            Stmt::If { b_then, b_else, .. } => match b_else {
                Some(b_else) => {
                    Self::stmt_always_returns(b_then) && Self::stmt_always_returns(b_else)
                }
                None => false,
            },
            Stmt::VarDeclaration { .. }
            | Stmt::Expr(_)
            | Stmt::While { .. }
            | Stmt::DoWhile { .. }
            | Stmt::For { .. }
            | Stmt::Switch { .. }
            | Stmt::Break
            | Stmt::Continue => false,
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDeclaration { varType, name, value } => {
                if let Some(expr) = value {
                    self.check_expr(expr);
                }
                self.declare(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        ty: varType.clone(),
                        kind: Kind::Variable,
                    },
                );
            }
            Stmt::ReturnStmt(value) => {
                let expected = self
                    .current_return_type
                    .clone()
                    .expect("return statement checked outside of a function");
                match value {
                    Some(expr) => {
                        let actual = self.check_expr(expr);
                        if actual != expected {
                            self.error(format!(
                                "Return type mismatch: expected {:?}, found {:?}",
                                expected, actual
                            ));
                        }
                    }
                    None => {
                        self.error(format!("Missing return value, expected {:?}", expected));
                    }
                }
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr);
            }
            Stmt::Block(stmts) => {
                self.push_scope();
                for s in stmts {
                    self.check_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::If { condition, b_then, b_else } => {
                self.check_expr(condition);
                self.check_stmt(b_then);
                if let Some(b_else) = b_else {
                    self.check_stmt(b_else);
                }
            }
            Stmt::While { condition, body } => {
                self.check_expr(condition);
                self.loop_depth += 1;
                self.check_stmt(body);
                self.loop_depth -= 1;
            }
            Stmt::DoWhile { body, condition } => {
                self.loop_depth += 1;
                self.check_stmt(body);
                self.loop_depth -= 1;
                self.check_expr(condition);
            }
            Stmt::For { init, condition, update, body } => {
                self.push_scope();
                if let Some(init) = init {
                    self.check_stmt(init);
                }
                self.check_expr(condition);
                if let Some(update) = update {
                    self.check_expr(update);
                }
                self.loop_depth += 1;
                self.check_stmt(body);
                self.loop_depth -= 1;
                self.pop_scope();
            }
            Stmt::Switch { cond, cases } => {
                self.check_expr(cond);
                self.switch_depth += 1;
                for case in cases {
                    if let Some(value) = &case.value {
                        self.check_expr(value);
                    }
                    for s in &case.body {
                        self.check_stmt(s);
                    }
                }
                self.switch_depth -= 1;
            }
            Stmt::Break => {
                if self.loop_depth == 0 && self.switch_depth == 0 {
                    self.error("'break' used outside of a loop or switch".to_string());
                }
            }
            Stmt::Continue => {
                if self.loop_depth == 0 {
                    self.error("'continue' used outside of a loop".to_string());
                }
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::IntegerLit(_) => Type::Int,
            Expr::CharLit(_) => Type::Char,
            Expr::StringLit(_) => Type::String,
            Expr::Identifier(name) => match self.lookup(name) {
                Some(sym) => sym.ty.clone(),
                None => {
                    self.error(format!("Undeclared identifier '{}'", name));
                    Type::Int
                }
            },
            Expr::Assignment { target, op: _, value } => {
                if !matches!(target.as_ref(), Expr::Identifier(_)) {
                    self.error("Left-hand side of assignment must be a variable".to_string());
                }
                let target_ty = self.check_expr(target);
                let value_ty = self.check_expr(value);
                if target_ty != value_ty {
                    self.error(format!(
                        "Cannot assign value of type {:?} to variable of type {:?}",
                        value_ty, target_ty
                    ));
                }
                target_ty
            }
            Expr::BinaryOp { left, op: _, right } => {
                let left_ty = self.check_expr(left);
                let right_ty = self.check_expr(right);
                if left_ty != right_ty {
                    self.error(format!(
                        "Type mismatch in binary expression: {:?} vs {:?}",
                        left_ty, right_ty
                    ));
                }
                left_ty
            }
            Expr::UnaryOp { op: _, expr } => self.check_expr(expr),
            Expr::FunCall { callee, args } => match self.lookup(callee).cloned() {
                Some(Symbol {
                    kind: Kind::Function { params, return_value },
                    ..
                }) => {
                    if params.len() != args.len() {
                        self.error(format!(
                            "Function '{}' expects {} argument(s), found {}",
                            callee,
                            params.len(),
                            args.len()
                        ));
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_ty = self.check_expr(arg);
                        if let Some(expected_ty) = params.get(i) {
                            if *expected_ty != arg_ty {
                                self.error(format!(
                                    "Argument {} of '{}' expected type {:?}, found {:?}",
                                    i + 1,
                                    callee,
                                    expected_ty,
                                    arg_ty
                                ));
                            }
                        }
                    }
                    return_value
                }
                Some(_) => {
                    self.error(format!("'{}' is not a function", callee));
                    Type::Int
                }
                None => {
                    self.error(format!("Undeclared function '{}'", callee));
                    Type::Int
                }
            },
        }
    }
}
