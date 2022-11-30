use crate::BinOp;
use crate::Env;
use crate::Expr;
use crate::Statement;
use crate::Syntax;

pub fn eval(syntax: Syntax, env: &mut Env) -> () {
    match syntax {
        Syntax::Statement(st) => {
            exec(st, env);
        }
    }
}

// 文を実行する
fn exec(statement: Statement, env: &mut Env) -> () {
    match statement {
        Statement::CompoundStatement { st1, st2 } => {
            exec(*st1, env);
            exec(*st2, env);
        }
        // TODO 関数作ったら消す
        Statement::Print { expr } => {
            print!("{:?}\n", calc(expr, env));
        }
        Statement::Assign { id, e } => {
            let value = calc(*e, env);
            env.insert(id, value);
        }
        Statement::If {
            condition,
            then,
            els,
        } => {
            if calc(*condition, env) > 0 {
                exec(*then, env)
            } else {
                exec(*els, env)
            }
        }
        _ => panic!("実行できない Statement を実行しようとした"),
    }
}

// 式を計算する
fn calc(expr: Expr, env: &mut Env) -> i32 {
    match expr {
        Expr::Binary { op, lhs, rhs } => match op {
            BinOp::Add => return calc(*lhs, env) + calc(*rhs, env),
            BinOp::Sub => return calc(*lhs, env) - calc(*rhs, env),
            BinOp::Mul => return calc(*lhs, env) * calc(*rhs, env),
            BinOp::Div => return calc(*lhs, env) / calc(*rhs, env),
        },
        Expr::Number(n) => n,
        Expr::Var(s) => match env.get(&s.to_string()) {
            Some(num) => *num,
            None => panic!("環境 env に変数名 {:?} が登録されていない", &s.to_string()),
        },
    }
}
