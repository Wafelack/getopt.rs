use crate::scanner::tokens::*;

pub struct Scanner {
    pub source: String,
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.clone(),
            chars: source.clone().chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, toktype: TokenType) {
        let text: String = self.source[self.start..self.current].into();
        self.tokens.push(Token::new(toktype, text, self.line));
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }
    fn goback(&mut self) -> char {
        self.current -= 1;
        self.chars[self.current - 1]
    }
    fn next_is(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        true
    }
    fn prev_is(&mut self, expected: char) -> bool {
        if self.current < 2 {
            return false;
        }
        if self.chars[self.current - 2] != expected {
            return false;
        }
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current - 1]
    }
    fn peek_next(&self) -> char {
        if self.current >= self.source.len() || self.is_at_end() {
            return '\0';
        }
        self.chars[self.current]
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => {
                if self.prev_is(']') {
                    // do link stuff
                } else {
                    self.add_token(TokenType::Char('('));
                }
            }
            ')' => self.add_token(TokenType::Char(')')),
            '[' => self.add_token(TokenType::Char('[')), // temp
            ']' => self.add_token(TokenType::Char(']')),
            '!' => {
                if self.next_is('[') {
                    self.advance();
                    let mut alt = String::new();

                    while !self.next_is(']') {
                        alt.push(self.advance());
                    }
                    self.advance();
                    if self.next_is('(') {
                        let mut link = String::new();
                        self.advance();
                        while !self.next_is(')') {
                            link.push(self.advance());
                        }
                        self.advance();
                        self.add_token(TokenType::Img(alt, link));
                    } else {
                        self.add_token(TokenType::Char('!'));
                        self.add_token(TokenType::Char('['));
                        let chars: Vec<char> = alt.chars().collect();
                        for c in chars {
                            self.add_token(TokenType::Char(c));
                        }
                        self.add_token(TokenType::Char(']'));
                    }
                } else {
                    self.add_token(TokenType::Char('!'));
                }
            }
            '#' => {
                if self.next_is('#') {
                    self.advance();
                    if self.peek_next() == '#' {
                        self.advance();
                        let mut title = String::new();

                        while self.peek_next() != '\n' {
                            title.push(self.peek());
                            self.advance();
                        }
                        self.add_token(TokenType::H3(title));
                    } else {
                        let mut title = String::new();

                        while self.peek() != '\n' {
                            title.push(self.peek());
                            self.advance();
                        }
                        self.add_token(TokenType::H2(title));
                    }
                } else {
                    let mut title = String::new();
                    self.advance();

                    while self.peek() != '\n' {
                        title.push(self.peek());
                        self.advance();
                    }
                    self.add_token(TokenType::H1(title));
                }
            }
            '\n' => {
                self.line += 1;
            }
            x => self.add_token(TokenType::Char(x)),
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token();
        }
    }
}
