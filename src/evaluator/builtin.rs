use crate::evaluator::Expr;
use std::collections::HashMap;

#[derive(Clone)]
struct Function<'a> {
    name: &'a str,
    expr: fn(Vec<Expr>),
}

// 組み込み関数の一覧
static BUILTIN_LIST: [Function; 1] = [Function {
    name: "hoge",
    expr: print_int,
}];

// 組み込み関数の一覧をハッシュマップにして取得
fn builtin_list_map() -> HashMap<String, fn(Vec<Expr>)> {
    let mut map = HashMap::new();
    for func in BUILTIN_LIST.clone() {
        map.entry(func.name.to_string()).or_insert(func.expr);
    }
    map // TODO 一回作ったマップを使い回すようにする
}

// 引数に指定した関数名の組み込み関数が存在するか
// TODO 関数があれば関数の実体を返す、に変更する
pub fn is_builtin(name: String) -> bool {
      match builtin_list_map().get(&name.to_string()) {
          Some(f) => true,
          None => false,
      }
}

//  引数に指定した関数名の組み込み関数を実行する
pub fn exec_builtin(name: String, params: Vec<Expr>) -> i32 {
    match builtin_list_map().get(&name.to_string()) {
        Some(f) => f(params),
        None => panic!("関数テーブル ft に関数名 {:?} が登録されていない", name),
    };
    0
}

///
/// print_int
///
fn print_int(params: Vec<Expr>) -> () {
    // TODO Exprをそのまま出すのではなく計算してから書き出すようにする
    print!("{:?}\n", params[0]);
}
