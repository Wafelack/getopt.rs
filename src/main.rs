mod parser;
mod scanner;

use lines_from_file::lines_from_file;
use parser::parser::parse_tokens;
use scanner::scanner::Scanner;
use scanner::tokens::TokenType;
use std::path::Path;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenprinting() {
        let mut src =
            "#Test\n##Test2\n###Test3 #notworking\n[my website](https://wafelack.fr)![alt](link)"
                .to_string();

        let mut scanner = Scanner::new(src.clone());
        scanner.scan_tokens();

        println!("Base string : {:?}\n", src);

        for tok in scanner.tokens {
            match tok.toktype {
                TokenType::Img(alt, link) => println!("<img src=\"{}\" alt=\"{}\"/>", link, alt),
                TokenType::H1(title) => println!("<h1>{}</h1>", title),
                TokenType::H2(title) => println!("<h2>{}</h2>", title),
                TokenType::H3(title) => println!("<h3>{}</h3>", title),
                TokenType::Char(c) => println!("`{}`", c),
                TokenType::Link(alt, link) => println!("<a href=\"{}\">{}</a>", link, alt),
                TokenType::Br => println!("<br>"),
            }
        }
    }
    #[test]
    fn parsedprinting() {
        let mut src =
            "#Test\n##Test2\n###Test3 #notworking\n[my website](https://wafelack.fr)![alt](link) This is some text"
                .to_string();
        let mut scanner = Scanner::new(src.clone());
        scanner.scan_tokens();
        let parsed = parse_tokens(scanner.tokens);
        println!("Base string : {:?}\n", src);
        println!("Parsed string : {}", parsed)
    }
}

fn usage() {
    eprintln!("Usage: marsdown <filename>.md");
    std::process::exit(-65);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage();
    }
    if !Path::new(&args[1]).exists() {
        eprintln!("File not found");
        usage();
    }
    let lines: String = lines_from_file(&args[1]).join("\n");

    let mut scanner = Scanner::new(lines);
    let content = parse_tokens(scanner.tokens);
}
