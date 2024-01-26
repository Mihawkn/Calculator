mod builtin;

use crate::Env;
use crate::FunctionTable;

use crate::enums::BinOp;
use crate::enums::ComparisonOp;
use crate::enums::Declaration;
use crate::enums::Expr;
use crate::enums::Statement;
use crate::enums::Syntax;
use crate::enums::Value;

pub fn eval(syntax: Syntax, env: &mut Env, ft: &mut FunctionTable) -> () {
    // 準備
    builtin::register(ft);

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
            if calc(*condition, env, ft) > Value::Int(0) {
                exec(*then, env, ft)
            } else {
                exec(*els, env, ft)
            }
        }
        _ => panic!("実行できない Statement: {:?} を実行しようとした", statement),
    }
}

// 式を計算する
fn calc(expr: Expr, env: &mut Env, ft: &mut FunctionTable) -> Value {
    match expr {
        Expr::Binary { op, lhs, rhs } => match op {
            BinOp::Add => match (calc(*lhs, env, ft), calc(*rhs, env, ft)) {
                (Value::Int(lhs_n), Value::Int(rhs_n)) => Value::Int(lhs_n + rhs_n),
                _ => panic!("加算処理が未定義")
            },
            BinOp::Sub => match (calc(*lhs, env, ft), calc(*rhs, env, ft)) {
                (Value::Int(lhs_n), Value::Int(rhs_n)) => Value::Int(lhs_n - rhs_n),
                _ => panic!("減算処理が未定義")
            },
            BinOp::Mul => match (calc(*lhs, env, ft), calc(*rhs, env, ft)) {
                (Value::Int(lhs_n), Value::Int(rhs_n)) => Value::Int(lhs_n * rhs_n),
                _ => panic!("乗算処理が未定義")
            },
            BinOp::Div => match (calc(*lhs, env, ft), calc(*rhs, env, ft)) {
                (Value::Int(lhs_n), Value::Int(rhs_n)) => Value::Int(lhs_n / rhs_n),
                _ => panic!("除算処理が未定義")
            },
        },
        Expr::Comparison { op, lhs, rhs } => match op {
            ComparisonOp::Lt => {
                return if calc(*lhs, env, ft) < calc(*rhs, env, ft) {
                    Value::Int(1)
                } else {
                    Value::Int(0)
                }
            }
        },
        Expr::Number(n) => Value::Int(n),
        Expr::Var(s) => match env.get(&s.to_string()) {
            Some(num) => num.clone(),
            None => panic!(
                "環境 env: {:?} に変数名 {:?} が登録されていない",
                env,
                &s.to_string()
            ),
        },
        Expr::Str(s) => Value::Text(s),
        Expr::FunctionCall { id, args } => {
            // ローカル環境を用意する
            let mut local_env = Env::new();

            match ft.get(&id.to_string()) {
                Some(Declaration::BuiltinFunction { id: _, r#fn }) => {
                    // 組み込み関数を実行する
                    r#fn(
                        args.iter()
                            .map(|x| calc(x.clone(), env, &mut ft.clone()))
                            .collect(),
                    )
                }
                Some(Declaration::Function { arg, st }) => {
                    // 引数として渡した値をセットする
                    for (expr, param) in args.iter().zip(arg.clone().iter()) {
                        local_env.insert(param.to_string(), calc(expr.clone(), env, &mut ft.clone()));
                    }

                    // 関数を実行する
                    exec(*st.clone(), &mut local_env, &mut ft.clone());
                    match local_env.get(&"return".to_string()) {
                        Some(i) => i.clone(),
                        None => todo!("関数が値を返さない場合の挙動が未定義"),
                    }
                }
                None => panic!("関数テーブル ft に関数名 {:?} が登録されていない", id),
            }
        }
    }
}
