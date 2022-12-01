use crate::parser::Parser;  
use crate::BinOp;
use crate::Expr;
use crate::Token;

impl Parser {
    pub(crate) fn mul(&mut self) -> Expr {
        let expr = self.primary();
        match self.current() {
            Some(Token::STAR) => self.star(expr),
            Some(Token::SLASH) => self.slash(expr),
            _ => expr,
        }
    }

    fn star(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::STAR);
        Expr::Binary {
            op: BinOp::Mul,
            lhs: Box::new(lhs),
            rhs: Box::new(self.primary()),
        }
    }

    fn slash(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::SLASH);
        Expr::Binary {
            op: BinOp::Div,
            lhs: Box::new(lhs),
            rhs: Box::new(self.primary()),
        }
    }
}
