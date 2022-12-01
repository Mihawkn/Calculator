use crate::parser::Parser;
use crate::Statement;
use crate::Expr;
use crate::Token;

impl Parser {
    pub(crate) fn state(&mut self) -> Statement {
        let result = match self.current() {
            Some(Token::PRINT) => self.print(), // TODO 関数を作ったら消す
            Some(Token::IF) => self.sif(),
            Some(Token::IDENT(s)) => self.sident(s),
            _ => Statement::Null,
        };

        // 文の後に ';' が続くようであれば次の文を扱う
        match self.current() {
            Some(Token::SEMICOLON) => self.compound(result),
            _ => result,
        }
    }

    pub(crate) fn expr(&mut self) -> Expr {
          return self.add();
     }

    fn print(&mut self) -> Statement {
        self.confirm(Token::PRINT);
        Statement::Print { expr: self.expr() }
    }

    fn sif(&mut self) -> Statement {
        self.confirm(Token::IF);
        let expr1 = self.expr();

        self.confirm(Token::LBRACE);
        let state1 = self.state();
        self.confirm(Token::RBRACE);

        self.confirm(Token::ELSE);
        self.confirm(Token::LBRACE);
        let state2 = self.state();
        self.confirm(Token::RBRACE);

        Statement::If {
            condition: Box::new(expr1),
            then: Box::new(state1),
            els: Box::new(state2),
        }
    }

    fn sident(&mut self, s: String) -> Statement {
        match self.next() {
            Some(Token::EQ) => {
                self.fix();
                self.confirm(Token::EQ);
                Statement::Assign {
                    id: s,
                    e: Box::new(self.expr()),
                }
            }
            _ => Statement::Null,
        }
    }

    fn compound(&mut self, st: Statement) -> Statement {
        self.confirm(Token::SEMICOLON);
        Statement::CompoundStatement {
            st1: Box::new(st),
            st2: Box::new(self.state()),
        }
    }
}

