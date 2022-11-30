use crate::BinOp;
use crate::Expr;
use crate::Statement;
use crate::Syntax;
use crate::Token;

struct Parser {
    input: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            input: tokens,
            pos: 0,
        }
    }

    // 現在のトークンを取得する
    fn current_token(&self) -> Option<Token> {
        if self.pos < self.input.len() {
            return Some(self.input[self.pos].clone());
        }
        None
    }

    // 現在のトークンを確定済みにする
    fn fix(&mut self) {
        self.pos += 1;
    }

    // 次のトークンを確認する
    fn next_token(&self) -> Option<Token> {
        if self.pos + 1 < self.input.len() {
            return Some(self.input[self.pos + 1].clone());
        }
        None
    }

    // State = CompoundStatement | IfStatement | CompoundStatement'
    fn state(&mut self) -> Statement {
        let mut result = Statement::Null;

        while let Some(token) = self.current_token() {
            match token {
                // TODO 関数作ったら消す
                Token::PRINT => {
                    self.fix(); // PRINT を確定
                    result = Statement::Print { expr: self.expr() };
                }

                // State = IF Expr '{' State '}' ELSE '{' State '}'
                Token::IF => {
                    self.fix(); // IF を確定
                    let expr1 = self.expr();
                    self.fix(); // '{' を確定
                    let state1 = self.state();
                    self.fix(); // '}' を確定
                    self.fix(); // ELSE を確定
                    self.fix(); // '{' を確定
                    let state2 = self.state();
                    self.fix(); // '}' を確定

                    result = Statement::If {
                        condition: Box::new(expr1),
                        then: Box::new(state1),
                        els: Box::new(state2),
                    };
                }

                // State = ID '=' Expr
                Token::IDENT(s) => {
                    while let Some(next_token) = self.next_token() {
                        if next_token == Token::EQ {
                            self.fix(); // IDENT を確定
                            self.fix(); // EQ を確定
                            result = Statement::Assign {
                                id: s,
                                e: Box::new(self.expr()),
                            };
                        }
                        break;
                    }
                }

                _ => {}
            }
            break;
        }

        // 文の後に ';' が続くようであれば次の文を扱う
        return if self.current_token() == Some(Token::SEMICOLON) {
            self.fix();
            Statement::CompoundStatement {
                st1: Box::new(result),
                st2: Box::new(self.state()),
            }
        } else {
            result
        };
    }

    // Expr = AddExpr
    fn expr(&mut self) -> Expr {
        return self.add();
    }

    // AddExpr = MulExpr {AddOp MulExpr}
    fn add(&mut self) -> Expr {
        let mut result = self.mul(); // 最初のMulExprは確定

        // AddOp Term が続く限り評価する
        while let Some(token) = self.current_token() {
            match token {
                Token::PLUS => {
                    self.fix();
                    result = Expr::Binary {
                        op: BinOp::Add,
                        lhs: Box::new(result),
                        rhs: Box::new(self.mul()),
                    }
                }
                Token::MINUS => {
                    self.fix();
                    result = Expr::Binary {
                        op: BinOp::Sub,
                        lhs: Box::new(result),
                        rhs: Box::new(self.mul()),
                    }
                }
                _ => {
                    break;
                }
            }
        }

        return result;
    }

    /// PrimaryExpr = '(' AddExpr ')' | NUMBER | ID
    fn primary(&mut self) -> Expr {
        let token = match self.current_token() {
            Some(token) => token,
            None => panic!("PrimaryExpr を判定する際にトークンひとつも取得できない"),
        };

        return match token {
            Token::LPAR => {
                self.fix();
                let result = self.expr();
                if self.current_token() != Some(Token::RPAR) {
                    panic!("LPAR に対応する RPAR が見つからない")
                };
                self.fix();
                result
            }
            Token::LBRACE => {
                self.fix();
                let result = self.expr();
                if self.current_token() != Some(Token::RBRACE) {
                    panic!("LPAR に対応する RBRACE が見つからない")
                };
                self.fix();
                result
            }
            Token::NUMBER(n) => {
                self.fix();
                Expr::Number(n)
            }
            Token::IDENT(str) => {
                self.fix();
                Expr::Var(str)
            }
            _ => {
                panic!("PrimaryExpr を判定する際に想定外のトークンがきた");
            }
        };
    }

    // MulExpr = PrimaryExpr {MulOp PrimaryExpr}
    fn mul(&mut self) -> Expr {
        let mut result = self.primary();

        // 次のトークンを評価する
        while let Some(token) = self.current_token() {
            match token {
                Token::STAR => {
                    self.fix(); // 確定したので次へ
                    result = Expr::Binary {
                        op: BinOp::Mul,
                        lhs: Box::new(result),
                        rhs: Box::new(self.primary()),
                    }
                }
                Token::SLASH => {
                    self.fix(); // 確定したので次へ
                    result = Expr::Binary {
                        op: BinOp::Div,
                        lhs: Box::new(result),
                        rhs: Box::new(self.primary()),
                    }
                }
                _ => {
                    break;
                }
            }
        }

        return result;
    }
}

pub fn parser(toks: Vec<Token>) -> Syntax {
    let mut parser = Parser::new(toks);
    return Syntax::Statement(parser.state());
}
