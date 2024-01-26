mod enums;
mod evaluator;
mod parser;
mod scanner;

use crate::enums::Env;
use crate::enums::FunctionTable;
 use crate::enums::Value;

fn print_eval_result(str: &str) -> () {
    print!("-----------------------------------------\n");
    print!("計算対象：\n {:?}\n", str);
    print!("スキャン結果：\n {:?}\n", scanner::scanner(str));
    print!(
        "パース結果：\n {:?}\n",
        parser::parser(scanner::scanner(str))
    );

    let mut env = Env::new();
    let mut ft = FunctionTable::new();
    print!("実行：");
    evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);

    print!(
        "実行後の状態：\n　環境：{:?}\n　関数テーブル：{:?}\n",
        env, ft
    );
    print!("-----------------------------------------\n");
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(str) => print_eval_result(str),
        _ => print!("usage: cargo run \"x = 1 + 2 + 3; print(x)\"\n"),
    }

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign() {
        let str = "x = 123";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 123 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(123));
    }

    #[test]
    fn test_negative() {
        let str = "x = -1";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = -1 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(-1));
    }

    #[test]
    fn test_if() {
        let str = "if 0 { x = 2 } else { x = 3 }";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 3 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(3));
    }

    #[test]
    fn test_addition() {
        let str = "x = 1 + 2 + 3";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(6));
    }

    #[test]
    fn test_subtraction() {
        let str = "x = 1 - 2 - 3";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = -4 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(-4));
    }

    #[test]
    fn test_multiplication() {
        let str = "x = 1 * 2 * 3";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(6));
    }

    #[test]
    fn test_division() {
        let str = "x = 4 / 2 / 2";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 1 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(1));
    }

    #[test]
    fn test_parenthesis() {
        let str = "x = 2 * (3 + 4) ";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 14 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(14));
    }

    #[test]
    fn test_compound_statement() {
        let str = "if 0 { x = 0 } else { x = 1 } ; if x { x = 3 } else { x = 4 }";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 3 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(3));
    }

    #[test]
    fn test_function_define() {
        let str = "fn test(i) { return i + 1 }; x = (test(2) + 3)";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(6));
    }

    #[test]
    fn test_function_call() {
        let str = "fn fib(n) { if ( n < 3 ) { return 1 } else { return fib( n - 1 ) + fib( n - 2 ) } }; x = fib(22)";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 17711 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(17711));
    }

    #[test]
    fn test_builtin_function_call() {
        let str = "x = 1 + 2 + 3; print_int(x)";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Int(6));
    } 

    #[test]
    fn test_str_value() {
        let str="x=\"Hello\"; print_str(x)";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = Hello が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::Text("Hello".to_string()));
    }

}
