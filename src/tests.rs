#[cfg(test)]
mod tests {
    use crate::scanner;
    use crate::parser;
    use crate::evaluator;

    use crate::enums::Env;
    use crate::enums::FunctionTable;
    use crate::enums::Value;

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
    fn test_zero_division() {
        let str = "x = 4 / 0";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に0で除算を試みた時点でエラーで止まること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
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
        let str = "x=\"Hello\"; print_str(x)";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に x = Hello が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["x"], Value::String("Hello".to_string()));
    }

    #[test]
    fn test_str_add() {
        let str = "x = \"abc\"; y = \"def\"; z = x + y";
        let mut env = Env::new();
        let mut ft = FunctionTable::new();

        // 実行後に z = abcdef が代入されていること
        evaluator::eval(parser::parser(scanner::scanner(str)), &mut env, &mut ft);
        assert_eq!(env["z"], Value::String("abcdef".to_string()));
    }
}
