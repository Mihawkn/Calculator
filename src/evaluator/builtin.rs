use crate::enums::Declaration;
use crate::enums::Value;
use crate::FunctionTable;

#[derive(Clone)]
struct Function<'a> {
    id: &'a str,
    r#fn: fn(Vec<Value>) -> Result<Value, String>,
}

// 組み込み関数の一覧
static BUILTIN_LIST: [Function; 3] = [
    Function {
        id: "print_int",
        r#fn: print,
    },
    Function {
        id: "print_str",
        r#fn: print,
    },
    Function {
        id: "print",
        r#fn: print,
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
/// print
///
fn print(params: Vec<Value>) -> Result<Value, String> {
    match &params[0] {
        Value::String(s) => {
            let mut str: String = s.to_string();

            // 文字列中に含まれる{}を第2引数以降に変換する
            for v in &params[1..] {
                str = str.replacen("{}", &v.to_string(), 1);
            }

            // 出力する
            print!("{:?}\n", str);
            Ok(Value::Unit)
        }
        _ => Err(format!("print 関数にテキスト以外が渡された")),
    }
}
