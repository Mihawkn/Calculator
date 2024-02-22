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
    GT,
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
    Gt,
    Eq,
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

pub trait AsBool {
    fn as_bool(&self) -> bool;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Int(i32),
    String(String),
    Bool(bool),
    Unit,
}
impl AsBool for Value {
    fn as_bool(&self) -> bool {
        match self {
            Value::Int(i) if *i == 0 => false,
            Value::Int(_i) => true,
            Value::String(_s) => true,
            Value::Bool(b) => *b,
            Value::Unit => false,
        }
    }
}
impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(i) => format!("{}", i),
            Value::String(s) => format!("{}", s),
            Value::Bool(b) => format!("{}", b),
            Value::Unit => format!(""),
        }
    }
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
