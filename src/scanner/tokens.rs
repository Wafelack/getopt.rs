pub enum Tokens {
    PARAGRAPHER(String),
    BR,
    VLINE,
    IMG(String, String),
    LINK(String, String),
    CHAR(char),
    END,
}
