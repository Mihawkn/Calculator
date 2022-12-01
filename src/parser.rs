mod state;
mod primaryexpr;
mod addexpr;
mod mulexpr;

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
            _ => panic!("想定外のトークンがきた"),
        };
    }
}


pub fn parser(toks: Vec<Token>) -> Syntax {
    let mut parser = Parser::new(toks);
    return Syntax::Statement(parser.state());
}
