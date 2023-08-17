use crate::parser::Parser;

use crate::enums::Expr;
use crate::enums::Statement;
use crate::enums::Token;

impl Parser {
    ///
    /// Statement = IfStatement | AssignStatement | CompoundStatement | ReturnStatement | FunctionDefineStatement
    /// CompoundStatement = Statement { ';' Statement }
    /// FunctionDefineStatement = 'fn' ID ParameterList '{' Statement '}'
    /// ReturnStatement       = 'return' AddExpr
    ///
    pub(crate) fn parse_state(&mut self) -> Statement {
        let result = match self.current() {
            Some(Token::PRINT) => self.parse_print(),
            Some(Token::IF) => self.parse_if(),
            Some(Token::RETURN) => self.parse_return(),
            Some(Token::FN) => self.parse_functiondefine(),
            Some(Token::IDENT(s)) => self.parse_ident(s),
            _ => Statement::Null,
        };

        // 文の後に ';' が続くようであれば次の文を扱う
        match self.current() {
            Some(Token::SEMICOLON) => self.parse_compound(result),
            _ => result,
        }
    }

    pub(crate) fn parse_expr(&mut self) -> Expr { return self.parse_relational(); }

    fn parse_print(&mut self) -> Statement {
        self.confirm(Token::PRINT);
        Statement::Print {
            expr: self.parse_expr(),
        }
    }

    fn parse_if(&mut self) -> Statement {
        self.confirm(Token::IF);
        let expr1 = self.parse_expr();

        self.confirm(Token::LBRACE);
        let state1 = self.parse_state();
        self.confirm(Token::RBRACE);

        self.confirm(Token::ELSE);
        self.confirm(Token::LBRACE);
        let state2 = self.parse_state();
        self.confirm(Token::RBRACE);

        Statement::If {
            condition: Box::new(expr1),
            then: Box::new(state1),
            els: Box::new(state2),
        }
    }

    fn parse_return(&mut self) -> Statement {
        self.confirm(Token::RETURN);
        Statement::Return {
            expr: Box::new(self.parse_expr()),
        }
    }

    fn parse_functiondefine(&mut self) -> Statement {
        self.confirm(Token::FN);

        // 関数名
        let id = match self.current() {
            Some(Token::IDENT(s)) => s,
            _ => panic!(
                "関数名を表すトークンが来ることを期待したが {:?} が出現した",
                self.current()
            ),
        };
        self.fix();

        // 引数 '(' ID { ',' ID } ')'
        self.confirm(Token::LPAR);
        let arg: Vec<String> = self.parse_id_list();
        self.confirm(Token::RPAR);

        // 関数の中身 '{' Statement '}'
        self.confirm(Token::LBRACE);
        let state = self.parse_state();
        self.confirm(Token::RBRACE);

        Statement::FunctionDefine {
            id: id,
            arg: arg,
            st: Box::new(state),
        }
    }

    fn parse_ident(&mut self, s: String) -> Statement {
        match self.next() {
            Some(Token::EQ) => {
                self.fix();
                self.confirm(Token::EQ);
                Statement::Assign {
                    id: s,
                    e: Box::new(self.parse_expr()),
                }
            }
            Some(Token::LPAR) => Statement::FunctionCall {
                expr: self.parse_expr(),
            },
            _ => Statement::Null,
        }
    }

    fn parse_compound(&mut self, st: Statement) -> Statement {
        self.confirm(Token::SEMICOLON);
        Statement::CompoundStatement {
            st1: Box::new(st),
            st2: Box::new(self.parse_state()),
        }
    }
}
