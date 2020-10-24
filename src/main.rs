mod scanner;

use scanner::{scanner::*, tokens::*};

fn main() {
    let mut scanner = Scanner::new("![alt](link)".to_string());
    scanner.scan_tokens();
    for tok in scanner.tokens {
        match tok.toktype {
            TokenType::Img(alt, link) => println!("<img src=\"{}\" alt=\"{}\"/>", link, alt),
            _ => (),
        }
    }
}
