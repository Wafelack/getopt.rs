use crate::scanner::tokens::Tokens;

struct Scanner {
    source: Vec<String>,
    current: u32,
    line: u32,
    tokens: Vec<Tokens>,
}
impl Scanner {
    pub fn new(source: Vec<String>) -> Scanner {
        Scanner {
            source,
            current: 0,
            line: 0,
            tokens: vec![],
        }
    }
    pub fn parse_line(&self) -> Tokens {
        if self.source.len() == self.line as usize {
            return Tokens::END;
        }

        Tokens::END
    }
}
