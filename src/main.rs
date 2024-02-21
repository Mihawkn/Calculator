mod enums;
mod evaluator;
mod parser;
mod scanner;
mod tests;

use crate::enums::Env;
use crate::enums::FunctionTable;

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
    print!("実行：\n");
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

