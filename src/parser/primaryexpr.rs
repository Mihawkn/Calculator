use crate::parser::Parser;

use crate::enums::Expr;
use crate::enums::Token;

impl Parser {
    ///
    /// PrimaryExpr = '(' Expr ')' | NUMBER | ID | FunctionCall
    ///
    pub(crate) fn parse_primary(&mut self) -> Expr {
        return match self.current() {
            Some(Token::LPAR) => self.parse_par(),
            Some(Token::LBRACE) => self.parse_brace(),
            Some(Token::NUMBER(n)) => self.parse_number(n),
            Some(Token::IDENT(str)) => self.parse_id(str),
            Some(Token::MINUS) => self.parse_negative(),
            _ => {
                panic!("PrimaryExpr を判定する際に想定外のトークンがきた");
            }
        };
    }

    fn parse_par(&mut self) -> Expr {
        self.confirm(Token::LPAR);
        let result = self.parse_expr();
        self.confirm(Token::RPAR);
        result
    }

    fn parse_brace(&mut self) -> Expr {
        self.confirm(Token::LBRACE);
        let result = self.parse_expr();
        self.confirm(Token::RBRACE);
        result
    }

    fn parse_number(&mut self, n: i32) -> Expr {
        self.confirm(Token::NUMBER(n));
        Expr::Number(n)
    }

    fn parse_id(&mut self, str: String) -> Expr {
        self.fix();

        match self.current() {
            Some(Token::LPAR) => {
                self.confirm(Token::LPAR);
                let args = self.parse_param_list();
                self.confirm(Token::RPAR);
                Expr::FunctionCall {
                    id: str,
                    args: args,
                }
            }
            _ => Expr::Var(str),
        }
    }

    // TODO 負の数考え中
    fn parse_negative(&mut self) -> Expr {
        self.confirm(Token::MINUS);
        match self.current() {
            Some(Token::NUMBER(n)) => {
                self.fix();
                Expr::Number(-1 * n)
            }
            _ => {
                panic!("PrimaryExpr を判定する際に想定外のトークンがきた");
            }
        }
    }
}
