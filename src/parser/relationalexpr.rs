use crate::parser::Parser;

use crate::enums::ComparisonOp;
use crate::enums::Expr;
use crate::enums::Token;

impl Parser {
    ///
    /// RelationalExpr = AddExpr { ComparisonOp AddExpr }
    /// ComparisonOp = '<'
    ///
    pub(crate) fn relational(&mut self) -> Expr {
        let expr = self.add();

        match self.current() {
            Some(Token::LT) => self.lt(expr),
            _ => expr,
        }
    }

    fn lt(&mut self, lhs: Expr) -> Expr {
        self.confirm(Token::LT);
        Expr::Comparison {
            op: ComparisonOp::Lt,
            lhs: Box::new(lhs),
            rhs: Box::new(self.add()),
        }
    }
}
