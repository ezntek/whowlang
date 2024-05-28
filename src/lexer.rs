#[derive(Debug)]
pub enum TokenKind {
    Key(String),
    Ident(String),
    Literal(String),
    Separator(char),
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    line: usize,
    col: usize,
}

pub struct Lexer {
    file: Vec<char>,
    cur: usize,
    line: usize,
    bol: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_whitespace_work() {
        let is_whitespace = |ch: char| "\n\t ".contains(ch);
        assert!(is_whitespace('a'));
    }
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let file = src.chars().collect::<Vec<char>>();

        Self {
            file,
            cur: 0,
            bol: 0,
            line: 1,
        }
    }

    fn col(&self) -> usize {
        return self.cur - self.bol;
    }

    fn cur(&self) -> char {
        return self.file[self.cur];
    }

    fn is_cur_whitespace(&self) -> bool {
        "\n\t ".contains(self.cur())
    }

    fn is_cur_separator(&self) -> bool {
        "{}()[]".contains(self.cur())
    }

    fn skip_whitespace(&mut self) {
        while self.cur < self.file.len() && self.is_cur_whitespace() {
            if self.cur() == '\n' {
                self.line += 1;
                self.bol = self.cur;
            }

            self.cur += 1;
        }
    }

    fn next_separator(&mut self) -> Option<Token> {
        if !"[]{}()".contains(self.cur()) {
            return None;
        }

        let res = Some(Token::new(
            TokenKind::Separator(self.cur()),
            self.line,
            self.col(),
        ));

        self.cur += 1;

        return res;
    }

    fn next_literal(&mut self) -> Option<Token> {
        if !"'\"1234567890".contains(self.cur()) {
            return None;
        }

        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();

        while self.cur < self.file.len() && !self.is_cur_whitespace() && !self.is_cur_separator() {
            buf.push(self.cur());
            self.cur += 1;
        }

        return Some(Token::new(
            TokenKind::Literal(buf.iter().collect::<String>()),
            line,
            col,
        ));
    }

    fn next_ident(&mut self) -> Option<Token> {
        if self.cur() != '$' {
            return None;
        }

        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();
        self.cur += 1;

        while self.cur < self.file.len() && !self.is_cur_whitespace() && !self.is_cur_separator(){
            buf.push(self.cur());
            self.cur += 1;
        }

        return Some(Token::new(
            TokenKind::Ident(buf.iter().collect::<String>()),
            line,
            col,
        ));
    }

    fn next_key(&mut self) -> Token {
        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();

        while self.cur < self.file.len() && !self.is_cur_whitespace() && !self.is_cur_separator(){
            buf.push(self.cur());
            self.cur += 1;
        }

        return Token::new(TokenKind::Key(buf.iter().collect::<String>()), line, col);
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(tok) = self.next_separator() {
            return tok;
        }

        if let Some(tok) = self.next_literal() {
            return tok;
        }

        if let Some(tok) = self.next_ident() {
            return tok;
        }
        return self.next_key();
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.cur + 1 < self.file.len() {
            tokens.push(self.next_token());
        }

        tokens
    }
}
