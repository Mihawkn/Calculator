use crate::parser::Parser;

use crate::enums::Expr;
use crate::enums::Token;

impl Parser {
    ///
    /// パラメーターリストをパースする
    ///
    /// paramList =  Expr { ',' Expr }
    ///
    /// * None - 引数なし
    ///
    pub(crate) fn parse_param_list(&mut self) -> Vec<Expr> {
        let mut param: Vec<Expr> = Vec::new();

        // ひとつめのパラメーターを格納する
        param.push(self.expr());

        // ふたつめ以降のパラメーターを格納する
        while let Some(Token::COMMA) = self.current() {
            self.confirm(Token::COMMA);
            param.push(self.expr());
        }

        param
    }

    ///
    /// パラメーターリストをパースする
    ///
    /// paramList = ID { ',' OD }
    ///
    /// * None - 引数なし
    ///
    pub(crate) fn parse_id_list(&mut self) -> Vec<String> {
        let mut param: Vec<String> = Vec::new();

        // ひとつめのパラメーターを格納する
        match self.current() {
            Some(Token::IDENT(s)) => {
                param.push(s);
                self.fix();
            }
            _ => {
                panic!("ID を期待したが予期せぬ {:?} が出現した", self.current());
            }
        }

        // ふたつめ以降のパラメーターを格納する
        while let Some(Token::COMMA) = self.current() {
            self.confirm(Token::COMMA);
            match self.current() {
                Some(Token::IDENT(s)) => {
                    param.push(s);
                    self.fix();
                }
                _ => {
                    panic!("ID を期待したが予期せぬ {:?} が出現した", self.current());
                }
            }
        }

        param
    }
}
