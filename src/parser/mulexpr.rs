use crate::parser::Parser;
use crate::BinOp;
use crate::Expr;
use crate::Token;

impl Parser {
    ///
    /// MulExpr = PrimaryExpr { MulOp PrimaryExpr }
    /// MulOp = '*' | '/'
    ///
    pub(crate) fn mul(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            match self.current() {
                Some(Token::STAR) => {
                    expr = self.star(expr);
                }
                Some(Token::SLASH) => {
                    expr = self.slash(expr);
                }
                _ => {
                    break;
                }
            }
        }
        expr
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
