use crate::enums::Declaration;
use crate::FunctionTable;

#[derive(Clone)]
struct Function<'a> {
    id: &'a str,
    r#fn: fn(Vec<i32>) -> i32,
}

// 組み込み関数の一覧
static BUILTIN_LIST: [Function; 1] = [Function {
    id: "print_int",
    r#fn: print_int,
}];

///
/// 組み込み関数を関数テーブルに設定する
///
pub fn register(ft: &mut FunctionTable) -> () {
    for builtin in BUILTIN_LIST.clone() {
        ft.insert(
            builtin.id.to_string(),
            Declaration::BuiltinFunction {
                id: builtin.id.to_string(),
                r#fn: builtin.r#fn,
            },
        );
    }
}

///
/// print_int
///
fn print_int(params: Vec<i32>) -> i32 {
    print!("{:?}\n", params[0]);
    0
}
