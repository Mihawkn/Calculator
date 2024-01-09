 mod builtin;

use crate::Env;
use crate::FunctionTable;

use crate::evaluator::builtin::is_builtin;
use crate::evaluator::builtin::exec_builtin;

use crate::enums::BinOp;
use crate::enums::ComparisonOp;
use crate::enums::Declaration;
use crate::enums::Expr;
use crate::enums::Statement;
use crate::enums::Syntax;

pub fn eval(syntax: Syntax, env: &mut Env, ft: &mut FunctionTable) -> () {
    match syntax {
        Syntax::Statement(st) => {
            exec(st, env, ft);
        }
    }
}

// 文を実行する
fn exec(statement: Statement, env: &mut Env, ft: &mut FunctionTable) -> () {
    match statement {
        Statement::CompoundStatement { st1, st2 } => {
            exec(*st1, env, ft);
            exec(*st2, env, ft);
        }
        Statement::FunctionDefine { id, arg, st } => {
            ft.insert(id, Declaration::Function { arg, st });
        }
        Statement::FunctionCall { expr } => {
            calc(expr, env, ft);
        }
        Statement::Return { expr } => {
            let value = calc(*expr, env, ft);
            env.insert("return".to_string(), value);
        }
        Statement::Assign { id, e } => {
            let value = calc(*e, env, ft);
            env.insert(id, value);
        }
        Statement::If {
            condition,
            then,
            els,
        } => {
            if calc(*condition, env, ft) > 0 {
                exec(*then, env, ft)
            } else {
                exec(*els, env, ft)
            }
        }
        _ => panic!("実行できない Statement: {:?} を実行しようとした", statement),
    }
}

// 式を計算する
fn calc(expr: Expr, env: &mut Env, ft: &mut FunctionTable) -> i32 {
    match expr {
        Expr::Binary { op, lhs, rhs } => match op {
            BinOp::Add => return calc(*lhs, env, ft) + calc(*rhs, env, ft),
            BinOp::Sub => return calc(*lhs, env, ft) - calc(*rhs, env, ft),
            BinOp::Mul => return calc(*lhs, env, ft) * calc(*rhs, env, ft),
            BinOp::Div => return calc(*lhs, env, ft) / calc(*rhs, env, ft),
        },
        Expr::Comparison { op, lhs, rhs } => match op {
            ComparisonOp::Lt => {
                return if calc(*lhs, env, ft) < calc(*rhs, env, ft) {
                    1
                } else {
                    0
                }
            }
        },
        Expr::Number(n) => n,
        Expr::Var(s) => match env.get(&s.to_string()) {
            Some(num) => *num,
            None => panic!(
                "環境 env: {:?} に変数名 {:?} が登録されていない",
                env,
                &s.to_string()
            ),
        },
        Expr::FunctionCall { id, args } => {
            // ローカル環境を用意する
            let mut local_env = Env::new();

            if is_builtin(id.to_string()) {
                   exec_builtin(id.to_string(), args)
            } else {         

            match ft.get(&id.to_string()) {
                Some(Declaration::Function { arg, st }) => {
                    // 引数として渡した値をセットする
                    for (expr, param) in args.iter().zip(arg.clone().iter()) {
                        local_env.insert(param.to_string(), calc(expr.clone(), env, &mut ft.clone()));
                    }

                    // 関数を実行する
                    exec(*st.clone(), &mut local_env, &mut ft.clone());
                    match local_env.get(&"return".to_string()) {
                        Some(i) => *i,
                        None => todo!("関数が値を返さない場合の挙動が未定義"),
                    }
                }
                None => panic!("関数テーブル ft に関数名 {:?} が登録されていない", id),
            }
          }
        }
    }
}
