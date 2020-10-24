pub enum TokenType {
    Char(char),
    Link(String, String),
    Img(String, String),
    H1(String),
    H2(String),
    H3(String),
}
pub struct Token {
    toktype: TokenType,
    lexeme: String,
    line: usize,
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
