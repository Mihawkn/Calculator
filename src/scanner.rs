use crate::enums::Token;

struct Scanner {
    input: Vec<char>,
    pos: usize,
}
impl Scanner {
    fn new(s: &str) -> Self {
        Scanner {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    /// 次のトークンを読んで返却する
    fn next(&mut self) -> Option<Token> {
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                ' ' => {
                    self.pos += 1;
                }
                '+' => {
                    self.pos += 1;
                    return Some(Token::PLUS);
                }
                '-' => {
                    // TODO 負の数を扱えるようにしたい
                    self.pos += 1;
                    return Some(Token::MINUS);
                }
                '*' => {
                    self.pos += 1;
                    return Some(Token::STAR);
                }
                '/' => {
                    self.pos += 1;
                    return Some(Token::SLASH);
                }
                '<' => {
                    self.pos += 1;
                    return Some(Token::LT);
                }
                '(' => {
                    self.pos += 1;
                    return Some(Token::LPAR);
                }
                ')' => {
                    self.pos += 1;
                    return Some(Token::RPAR);
                }
                '{' => {
                    self.pos += 1;
                    return Some(Token::LBRACE);
                }
                '}' => {
                    self.pos += 1;
                    return Some(Token::RBRACE);
                }
                '=' => {
                    self.pos += 1;
                    return Some(Token::EQ);
                }
                ',' => {
                    self.pos += 1;
                    return Some(Token::COMMA);
                }
                ';' => {
                    self.pos += 1;
                    return Some(Token::SEMICOLON);
                }
                '\\' => {
                    // エスケープ文字はとりあえず無視しておく
//                    self.pos += 1;
                }
                '"' => {
                    self.pos += 1;
                    let start_idx = self.pos;

                    // 次の " が来るまで読む
                    while self.pos < self.input.len() {
                        match self.input[self.pos] {
                            '"' => break,
                            _ => self.pos += 1,
                        }
                    }
                    let _str: String = self.input[start_idx..self.pos]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    self.pos += 1;
                    return Some(Token::STR(_str));
                }
                '0'..='9' => {
                    let start_idx = self.pos;
                    // 数字が続く限り次を読む
                    while self.pos < self.input.len() {
                        match self.input[self.pos] {
                            '0'..='9' => self.pos += 1,
                            _ => break,
                        }
                    }
                    let num: i32 = self.input[start_idx..self.pos]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    return Some(Token::NUMBER(num));
                }

                _ => {
                    let start_idx = self.pos;
                    // 文字が続く限り次を読む
                    while self.pos < self.input.len() {
                        match self.input[self.pos] {
                            'a'..='z' => self.pos += 1,
                            '_' => self.pos += 1,
                            _ => break,
                        }
                    }
                    let _word: String = self.input[start_idx..self.pos]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    // 予約語
                    return match &*_word {
                        "if" => Some(Token::IF),
                        "else" => Some(Token::ELSE),
                        "return" => Some(Token::RETURN),
                        "fn" => Some(Token::FN),
                        _ => Some(Token::IDENT(_word)),
                    };
                }
            }
        }
        return None;
    }
}

pub fn scanner(s: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut scanner = Scanner::new(s);
    while let Some(token) = scanner.next() {
        result.push(token);
    }

    result
}
