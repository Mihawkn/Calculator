use std::collections::HashMap;

pub type Env = HashMap<String, Value>;
pub type FunctionTable = HashMap<String, Declaration>;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAR,
    RPAR,
    LBRACE,
    RBRACE,
    EQ,
    LT,
    NUMBER(i32),
    IF,
    ELSE,
    IDENT(String),
    STR(String),
    SEMICOLON,
    COMMA,
    FN,
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOp {
    Lt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Comparison {
        op: ComparisonOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number(i32),
    Var(String),
    Str(String),
    FunctionCall {
        id: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    CompoundStatement {
        st1: Box<Statement>,
        st2: Box<Statement>,
    },
    Assign {
        id: String,
        e: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then: Box<Statement>,
        els: Box<Statement>,
    },
    Return {
        expr: Box<Expr>,
    },
    FunctionDefine {
        id: String,
        arg: Vec<String>,
        st: Box<Statement>,
    },
    FunctionCall {
        expr: Expr,
    },
    Null,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Int(i32),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Function {
        arg: Vec<String>,
        st: Box<Statement>,
    },
    BuiltinFunction {
        id: String,
        r#fn: fn(Vec<Value>) -> Result<Value, String>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    Statement(Statement),
}
