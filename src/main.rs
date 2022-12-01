mod evaluator;
mod parser;
mod scanner;

mod enums;
use crate::enums::BinOp;
use crate::enums::Env;
use crate::enums::Expr;
use crate::enums::Statement;
use crate::enums::Syntax;
use crate::enums::Token;

fn print_eval_result(str: &str) -> () {
    print!("-----------------------------------------\n");
    let mut env = Env::new();
    print!("計算対象：{:?}\n", str);
    print!("スキャン結果：{:?}\n", scanner::scanner(str));
    print!("パース結果：{:?}\n", parser::parser(scanner::scanner(str)));
    print!("結果：");
    evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
    print!("環境：{:?}\n", env);
    print!("-----------------------------------------\n");
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(str) => print_eval_result(str),
        _ => print!("usage: cargo run \"x = 1 + 2 + 3\"\n"),
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

        // 実行後に x = 123 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 123);
    }

    #[test]
    fn test_negative() {
        let str = "x = -1";
        let mut env = Env::new();

        // 実行後に x = -1 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], -1);
    }

    #[test]
    fn test_if() {
        let str = "if 0 { x = 2 } else { x = 3 }";
        let mut env = Env::new();

        // 実行後に x = 3 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 3);
    }

    #[test]
    fn test_addition() {
        let str = "x = 1 + 2 + 3";
        let mut env = Env::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 6);
    }

    #[test]
    fn test_subtraction() {
        let str = "x = 1 - 2 - 3";
        let mut env = Env::new();

        // 実行後に x = -4 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], -4);
    }

    #[test]
    fn test_multiplication() {
        let str = "x = 1 * 2 * 3";
        let mut env = Env::new();

        // 実行後に x = 6 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 6);
    }

    #[test]
    fn test_division() {
        let str = "x = 4 / 2 / 2";
        let mut env = Env::new();

        // 実行後に x = 1 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 1);
    }

    #[test]
    fn test_parenthesis() {
        let str = "x = 2 * (3 + 4) ";
        let mut env = Env::new();

        // 実行後に x = 14 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 14);
    }

    #[test]
    fn test_compound_statement() {
        let str = "if 0 { x = 0 } else { x = 1 } ; if x { x = 3 } else { x = 4 }";
        let mut env = Env::new();

        // 実行後に x = 3 が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env);
        assert_eq!(env["x"], 3);
    }
}
