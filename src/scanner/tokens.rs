pub enum TokenType {
    Char,
    Bang,
    Alt,
    Link,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    H1,
    H2,
    H3,
}
pub struct Token {
    toktype: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<String>,
}
impl Token {
    pub fn new(toktype: TokenType, lexeme: String, line: usize, literal: Option<String>) -> Token {
        Token {
            toktype,
            lexeme,
            line,
            literal,
        }
    }
}
