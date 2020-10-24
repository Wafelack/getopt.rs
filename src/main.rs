mod scanner;

use scanner::{scanner::*, tokens::*};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn img() {
        let mut scanner = Scanner::new("![alt](link)".to_string());
        scanner.scan_tokens();

        println!("{}", scanner.tokens[0].lexeme);

        assert_eq!(scanner.tokens.len(), 1);
        match &scanner.tokens[0].toktype {
            TokenType::Img(alt, link) => println!("Alt : {} ; Link : {}", alt, link),
            _ => panic!("Not expected type"),
        }
    }
}

fn main() {
    let mut scanner = Scanner::new("# Test\n![alt](link)".to_string());
    scanner.scan_tokens();

    for tok in scanner.tokens {
        match tok.toktype {
            TokenType::Img(alt, link) => println!("<img src=\"{}\" alt=\"{}\"/>", link, alt),
            TokenType::H1(title) => println!("<h1>{}</h1>", title),
            TokenType::Char(c) => println!("`{}`", c),
            _ => (),
        }
    }
}
