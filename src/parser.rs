use crate::lexer::{Token, TokenKind};
use core::panic;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Table(HashMap<String, Value>),
    Array(Vec<Value>),
    Null,
}

pub struct Parser {
    tokens: Vec<Token>,
    res: HashMap<String, Value>,
    variables: HashMap<String, Value>,
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

    pub fn parse_literal(&mut self, str: &str) -> Value {
        let terminator = str.as_bytes()[0];
        if terminator == b'"' || terminator == b'\'' {
            let mut buf: Vec<char> = Vec::new();

            for ch in str[1..].chars() {
                if ch as u8 == terminator {
                    break;
                }

                buf.push(ch);
            }

            return Value::String(buf.iter().collect::<String>());
        }

        if str == "true" || str == "yes" {
            return Value::Bool(true);
        } else if str == "false" || str == "no" {
            return Value::Bool(false);
        } else if str == "nil" || str == "null" {
            return Value::Null;
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
            Value::Float(val)
        } else {
            let val = str.parse::<i32>().unwrap();
            Value::Int(val)
        }
    }

    pub fn parse_sep(&mut self, ch: char) -> Value {
        let mut sqbracket_depth = 0 as usize;
        let mut curlybracket_depth = 0 as usize;

        if ch == '(' {
            panic!("round brackets not implemented");
        } else if ch == '[' {
            let mut elems: Vec<Value> = Vec::new();
            sqbracket_depth += 1;

            loop {
                self.cur += 1;
                let cur = self.cur().clone();

                if self.cur >= self.tokens.len() {
                    panic!("unexpected end of token stream");
                }

                if let TokenKind::Separator(ch) = cur.kind {
                    if ch == ']' {
                        sqbracket_depth -= 1;
                        if sqbracket_depth == 0 {
                            break;
                        }
                    } else if ch == '[' {
                        sqbracket_depth += 1;
                    } else {
                        elems.push(self.parse_sep(ch));
                    }
                } else if let TokenKind::Literal(s) = cur.kind {
                    elems.push(self.parse_literal(&s))
                } else if let TokenKind::Ident(s) = cur.kind {
                    let Some(val) = self.variables.get(&s) else {
                        panic!("invalid variable name");
                    };

                    elems.push(val.clone());
                } else {
                    panic!("expected identifier or literal expression within list declaration");
                }
            }

            Value::Array(elems)
        } else {
            let mut toks: Vec<Token> = Vec::new();
            curlybracket_depth += 1;

            loop {
                self.cur += 1;

                if self.cur >= self.tokens.len() {
                    panic!("unexpected end of token stream");
                }

                if let TokenKind::Separator(ch) = self.cur().kind {
                    if ch == '}' {
                        curlybracket_depth -= 1;
                        if curlybracket_depth == 0 {
                            break;
                        }
                    } else if ch == '{' {
                        curlybracket_depth += 1;
                    }

                    toks.push(self.cur().clone())
                } else {
                    toks.push(self.cur().clone())
                }
            }

            let parsed = Parser::new(toks).parse();
            return Value::Table(parsed);
        }
    }

    pub fn parse(mut self) -> HashMap<String, Value> {
        while self.cur < self.tokens.len() {
            if let TokenKind::Key(s) = self.cur().clone().kind {
                self.cur += 1;

                if self.cur >= self.tokens.len() {
                    panic!("unexpected end of token stream")
                }

                let next = self.cur().clone();
                let val: Value;

                if let TokenKind::Literal(s) = next.kind {
                    val = self.parse_literal(&s);
                } else if let TokenKind::Ident(s) = next.kind {
                    let Some(v) = self.variables.get(&s) else {
                        panic!("invalid variable name");
                    };

                    val = v.clone();
                } else if let TokenKind::Separator(c) = next.kind {
                    val = self.parse_sep(c);
                } else {
                    panic!("expected identifier or literal after key definition");
                }

                self.res.insert(s, val);
            } else if let TokenKind::Ident(s) = self.cur().clone().kind {
                self.cur += 1;

                if self.cur >= self.tokens.len() {
                    panic!("unexpected end of token stream")
                }

                let next = self.cur().clone();
                let val: Value;

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
