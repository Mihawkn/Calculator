use std::collections::HashMap;
pub type Env = HashMap<String, i32>;

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
    NUMBER(i32),
    IF,
    ELSE,
    IDENT(String),
    SEMICOLON,
    PRINT, // TODO 関数作ったら関数にする
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Number(i32),
    Var(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    CompoundStatement {
        st1: Box<Statement>,
        st2: Box<Statement>,
    },
    Print {
        // TODO 関数作ったら関数にする
        expr: Expr,
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
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    Statement(Statement),
}
