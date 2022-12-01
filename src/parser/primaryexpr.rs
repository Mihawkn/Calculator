use crate::parser::Parser;
use crate::Expr;
use crate::Token;

impl Parser {
    pub(crate) fn primary(&mut self) -> Expr {
        return match self.current() {
            Some(Token::LPAR) => self.par(),
            Some(Token::LBRACE) => self.brace(),
            Some(Token::NUMBER(n)) => self.number(n),
            Some(Token::IDENT(str)) => self.ident(str),
            _ => {
                panic!("PrimaryExpr を判定する際に想定外のトークンがきた");
            }
        };
    }

    fn par(&mut self) -> Expr {
        self.confirm(Token::LPAR);
        let result = self.expr();
        self.confirm(Token::RPAR);
        result
    }

    fn brace(&mut self) -> Expr {
        self.confirm(Token::LBRACE);
        let result = self.expr();
        self.confirm(Token::RBRACE);
        result
    }

    fn number(&mut self, n: i32) -> Expr {
        self.confirm(Token::NUMBER(n));
        Expr::Number(n)
    }

    fn ident(&mut self, str: String) -> Expr {
        self.fix();
        Expr::Var(str)
    }
}
