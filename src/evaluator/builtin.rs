use crate::enums::Declaration;
use crate::enums::Value;
use crate::FunctionTable;

#[derive(Clone)]
struct Function<'a> {
    id: &'a str,
    r#fn: fn(Vec<Value>) -> Result<Value, String>,
}

// 組み込み関数の一覧
static BUILTIN_LIST: [Function; 2] = [
    Function {
        id: "print_int",
        r#fn: print_int,
    },
    Function {
        id: "print_str",
        r#fn: print_str,
    },
];

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
fn print_int(params: Vec<Value>) -> Result<Value, String> {
    print!("{:?}\n", params[0]);
    Ok(Value::Int(0))
}

///
/// print_str
///
fn print_str(params: Vec<Value>) -> Result<Value, String> {
    print!("{:?}\n", params[0]);
    Ok(Value::Int(0))
}
