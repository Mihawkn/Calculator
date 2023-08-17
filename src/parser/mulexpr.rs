use crate::parser::Parser;

use crate::enums::BinOp;
use crate::enums::Expr;
use crate::enums::Token;

impl Parser {
    ///
    /// MulExpr = PrimaryExpr { MulOp PrimaryExpr }
    /// MulOp = '*' | '/'
    ///
    pub(crate) fn parse_mul(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        loop {
            match self.current() {
                Some(Token::STAR) => {
                    expr = self.parse_star(expr);
                }
                Some(Token::SLASH) => {
                    expr = self.parse_slash(expr);
                }
                _ => {
                    break;
                }
            }
        }
        expr
    }

    fn parse_star(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::STAR);
        Expr::Binary {
            op: BinOp::Mul,
            lhs: Box::new(lhs),
            rhs: Box::new(self.parse_primary()),
        }
    }

    fn parse_slash(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::SLASH);
        Expr::Binary {
            op: BinOp::Div,
            lhs: Box::new(lhs),
            rhs: Box::new(self.parse_primary()),
        }
    }
}
