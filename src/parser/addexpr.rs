use crate::parser::Parser;

use crate::enums::BinOp;
use crate::enums::Expr;
use crate::enums::Token;

impl Parser {
    ///
    /// AddExpr = MulExpr { AddOp MulExpr }
    /// AddOp = '+' | '-'
    ///
    pub(crate) fn parse_add(&mut self) -> Expr {
        let mut expr = self.parse_mul();
        loop {
            match self.current() {
                Some(Token::PLUS) => {
                    expr = self.parse_plus(expr);
                }
                Some(Token::MINUS) => {
                    expr = self.parse_minus(expr);
                }
                _ => {
                    break;
                }
            }
        }
        expr
    }

    fn parse_plus(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::PLUS);
        Expr::Binary {
            op: BinOp::Add,
            lhs: Box::new(lhs),
            rhs: Box::new(self.parse_mul()),
        }
    }

    fn parse_minus(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::MINUS);
        Expr::Binary {
            op: BinOp::Sub,
            lhs: Box::new(lhs),
            rhs: Box::new(self.parse_mul()),
        }
    }
}
