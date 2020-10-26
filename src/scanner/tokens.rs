#[derive(PartialEq, Eq)]
pub enum TokenType {
    Char(char),
    Link(String, String),
    Img(String, String),
    H1(String),
    H2(String),
    H3(String),
    Hr,
    Br,
    Pre(String),
    Code(String),
    Em(String),
    Bold(String),
}

pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    pub line: usize,
}
impl Token {
    pub fn new(toktype: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            toktype,
            lexeme,
            line,
        }
    }
}
