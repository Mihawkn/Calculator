use crate::parser::Parser;
use crate::BinOp;
use crate::Expr;
use crate::Token;

impl Parser {
    ///
    /// AddExpr = MulExpr { AddOp MulExpr }
    /// AddOp = '+' | '-'
    ///
    pub(crate) fn add(&mut self) -> Expr {
        let mut expr = self.mul();
        loop {
            match self.current() {
                Some(Token::PLUS) => {
                    expr = self.plus(expr);
                }
                Some(Token::MINUS) => {
                    expr = self.minus(expr);
                }
                _ => {
                    break;
                }
            }
        }
        expr
    }

    fn plus(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::PLUS);
        Expr::Binary {
            op: BinOp::Add,
            lhs: Box::new(lhs),
            rhs: Box::new(self.mul()),
        }
    }

    fn minus(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::MINUS);
        Expr::Binary {
            op: BinOp::Sub,
            lhs: Box::new(lhs),
            rhs: Box::new(self.mul()),
        }
    }
}
