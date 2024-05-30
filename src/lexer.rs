#[derive(Debug, Clone)]
pub enum TokenKind {
    Key(String),
    Ident(String),
    Literal(String),
    Separator(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
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

    fn is_separator(&self, ch: char) -> bool {
        "{}()[]".contains(ch)
    }

    fn is_cur_separator(&self) -> bool {
        self.is_separator(self.cur())
    }

    fn skip_whitespace(&mut self) {
        // FIXME: extremely clunky
        while self.cur < self.file.len() {
            if !self.cur().is_ascii_whitespace() {
                break;
            }

            if self.cur() == '\n' {
                self.line += 1;
                self.bol = self.cur;
            }

            self.cur += 1;
        }

        if self.cur >= self.file.len() {
            return;
        }

        if self.cur() == '#' {
            return self.skip_comment();
        }
    }

    fn skip_comment(&mut self) {
        if self.cur() != '#' {
            return;
        }

        self.cur += 1;

        while self.cur() != '\n' {
            self.cur += 1;
        }

        self.skip_whitespace();
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

    fn next_string_literal(&mut self) -> Token {
        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();
        let terminator = self.cur();
        let mut peek = self.cur + 1;

        buf.push(terminator); // for the parser
        while peek < self.file.len() && self.file[peek] != terminator {
            let ch = self.file[peek];

            if ch == '\\' {
                peek += 1;
                let next = self.file[peek];
                match next {
                    '"' => buf.push('"'),
                    '\'' => buf.push('\''),
                    'n' => buf.push('\n'),
                    'r' => buf.push('\r'),
                    _ => buf.push(next),
                }
            } else {
                buf.push(ch);
            }

            peek += 1;
        }
        buf.push(terminator);

        if peek >= self.file.len() && self.file[peek] != terminator {
            panic!("unterminated string literal at the end of file");
        }

        self.cur = peek + 1;
        let string = buf.iter().collect::<String>();
        return Token::new(TokenKind::Literal(string), line, col);
    }

    fn next_literal(&mut self) -> Option<Token> {
        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();
        let mut peek = self.cur;

        if self.cur() == '\"' || self.cur() == '\'' {
            return Some(self.next_string_literal());
        }

        while peek < self.file.len()
            && !self.file[peek].is_ascii_whitespace()
            && !self.is_separator(self.file[peek])
        {
            buf.push(self.file[peek]);
            peek += 1;
        }

        let string = buf.iter().collect::<String>().to_lowercase();

        if !"-1234567890".contains(string.chars().nth(0).unwrap())
            && !["yes", "no", "true", "false", "null", "nil"].contains(&string.as_ref())
        {
            return None;
        }

        self.cur = peek;

        return Some(Token::new(TokenKind::Literal(string), line, col));
    }

    fn next_ident(&mut self) -> Option<Token> {
        if self.cur() != '$' {
            return None;
        }

        let mut buf: Vec<char> = Vec::new();
        let line = self.line;
        let col = self.col();
        self.cur += 1;

        while self.cur < self.file.len()
            && !self.cur().is_ascii_whitespace()
            && !self.is_cur_separator()
        {
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

        while self.cur < self.file.len()
            && !self.cur().is_ascii_whitespace()
            && !self.is_cur_separator()
        {
            buf.push(self.cur());
            self.cur += 1;
        }

        return Token::new(TokenKind::Key(buf.iter().collect::<String>()), line, col);
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.cur >= self.file.len() {
            return None;
        }

        Some(if let Some(tok) = self.next_separator() {
            tok
        } else if let Some(tok) = self.next_literal() {
            tok
        } else if let Some(tok) = self.next_ident() {
            tok
        } else {
            self.next_key()
        })
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.cur + 1 < self.file.len() {
            if let Some(tok) = self.next_token() {
                tokens.push(tok);
            }
        }

        tokens
    }
}
