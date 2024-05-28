use crate::lexer::{Token, TokenKind};
use core::panic;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum WlangType {
    StringLiteral(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    BooleanLiteral(bool),
    Table(HashMap<String, WlangType>),
    Array(Vec<WlangType>),
}

pub struct Parser {
    tokens: Vec<Token>,
    res: HashMap<String, WlangType>,
    variables: HashMap<String, WlangType>,
    cur: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            variables: HashMap::new(),
            res: HashMap::new(),
            cur: 0,
        }
    }

    pub fn cur(&self) -> &Token {
        &self.tokens[self.cur]
    }

    pub fn parse_literal(&mut self, str: &str) -> WlangType {
        if str.as_bytes()[0] == b'\"' {
            let mut buf: Vec<char> = Vec::new();

            for ch in str[1..].chars() {
                if ch == '\"' {
                    break;
                }

                buf.push(ch);
            }

            return WlangType::StringLiteral(buf.iter().collect::<String>());
        }

        if str == "true" || str == "yes" {
            return WlangType::BooleanLiteral(true);
        } else if str == "false" || str == "no" {
            return WlangType::BooleanLiteral(false);
        }

        let mut decimals = 0;

        for ch in str.chars() {
            if ch == '.' {
                decimals += 1;
            }

            if !"1234567890".contains(ch) {
                panic!("invalid literal detected");
            }
        }

        if decimals >= 2 {
            panic!("too many decimals");
        } else if decimals == 1 {
            let val = str.parse::<f32>().unwrap();
            self.cur += 1;
            WlangType::FloatLiteral(val)
        } else {
            let val = str.parse::<i32>().unwrap();
            self.cur += 1;
            WlangType::IntLiteral(val)
        }
    }

    pub fn parse_sep(&mut self, ch: char) -> WlangType {
        if ch == '(' {
            panic!("round brackets not implemented");
        } else if ch == '[' {
            let mut elems: Vec<WlangType> = Vec::new();

            loop {
                if let TokenKind::Separator(ch) = self.cur().kind {
                    if ch == ']' {
                        break;
                    }
                } else if let TokenKind::Literal(s) = self.cur().clone().kind {
                    elems.push(self.parse_literal(&s))
                } else if let TokenKind::Ident(s) = self.cur().clone().kind {
                    let Some(val) = self.variables.get(&s) else {
                        panic!("invalid variable name");
                    };

                    elems.push(val.clone());
                } else {
                    panic!("expected identifier or literal within list declaration");
                }
            }

            WlangType::Array(elems)
        } else {
            panic!("recursive parsing not implemented");
        }
    }

    pub fn parse(mut self) -> HashMap<String, WlangType> {
        while self.cur < self.tokens.len() {
            if let TokenKind::Key(s) = self.cur().clone().kind {
                self.cur += 1;
                let next = self.cur().clone();
                let val: WlangType;

                if let TokenKind::Literal(s) = next.kind {
                    val = self.parse_literal(&s);
                } else if let TokenKind::Ident(s) = next.kind {
                    let Some(v) = self.variables.get(&s) else {
                        panic!("invalid variable name");
                    };

                    val = v.clone();
                } else {
                    panic!("expected identifier or literal after key definition");
                }

                self.res.insert(s, val);
            } else if let TokenKind::Ident(s) = self.cur().clone().kind {
                self.cur += 1;
                let next = self.cur().clone();
                let val: WlangType;

                if let TokenKind::Literal(s) = next.kind {
                    val = self.parse_literal(&s);
                } else if let TokenKind::Ident(s) = next.kind {
                    let Some(v) = self.variables.get(&s) else {
                        panic!("invalid variable name");
                    };

                    val = v.clone();
                } else {
                    panic!("expected identifier or literal after variable definition");
                }
                self.variables.insert(s, val);
            } else if let TokenKind::Separator(ch) = self.cur().clone().kind {
                self.parse_sep(ch);
            } else {
                panic!("invalid token found in stream")
            }

            self.cur += 1;
        }

        self.res
    }
}
