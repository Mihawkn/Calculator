mod addexpr;
mod mulexpr;
mod paramlist;
mod primaryexpr;
mod relationalexpr;
mod state;

use crate::enums::Syntax;
use crate::enums::Token;

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

    fn current(&self) -> Option<Token> {
        if self.pos < self.input.len() {
            return Some(self.input[self.pos].clone());
        }
        None
    }

    fn next(&self) -> Option<Token> {
        if self.pos + 1 < self.input.len() {
            return Some(self.input[self.pos + 1].clone());
        }
        None
    }

    fn fix(&mut self) {
        self.pos += 1;
    }

    fn confirm(&mut self, expect: Token) {
        match self.current() {
            Some(token) if token == expect => self.fix(),
            _ => panic!(
                "{:?} を想定していたが想定外のトークン {:?} がきた",
                expect,
                self.current()
            ),
        };
    }
}

pub fn parser(toks: Vec<Token>) -> Syntax {
    let mut parser = Parser::new(toks);
    return Syntax::Statement(parser.state());
}
