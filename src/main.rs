mod parser;
mod scanner;

use lines_from_file::lines_from_file;
use parser::parser::parse_tokens;
use scanner::scanner::Scanner;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(test)]
mod test {
    use super::*;
    use scanner::tokens::TokenType;

    #[test]
    fn tokenprinting() {
        let src =
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
                TokenType::Hr => println!("<hr>"),
            }
        }
    }
    #[test]
    fn parsedprinting() {
        let src =
            "#Test\n##Test2\n###Test3 #notworking\n---\n[my website](https://wafelack.fr)![alt](link) This is some text\ntest"
                .to_string();
        let mut scanner = Scanner::new(src.clone());
        scanner.scan_tokens();
        let parsed = parse_tokens(scanner.tokens);
        println!("Base string : {:?}\n", src);
        println!("Parsed string : {}", parsed)
    }
}

fn usage() {
    eprintln!("Usage: marsdown <filename>.md [title]");
    std::process::exit(-65);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }
    if !Path::new(&args[1]).exists() {
        eprintln!("File not found");
        usage();
    }

    let mut title = "TITLE".to_string();

    if args.len() >= 3 {
        title = "".to_string();
        for i in 2..args.len() {
            if i != args.len() - 1 {
                title.push_str(format!("{} ", args[i]).as_str());
            } else {
                title.push_str(args[i].as_str());
            }
        }
    }

    let lines: String = lines_from_file(&args[1]).join("\n");

    let mut scanner = Scanner::new(lines);
    scanner.scan_tokens();
    let content = parse_tokens(scanner.tokens);

    // Code below is for me to have a direct header for my articles, if you don't want that, you can delete until the next comment

    let full = format!(
        "<!DOCTYPE html>\n
    <html lang=\"fr\">\n
    <head>\n
        <meta charset=\"UTF-8\">\n
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n
        <title>{}</title>\n
        <link
          href=\"https://unpkg.com/tailwindcss@^1.0/dist/base.min.css\"
          rel=\"stylesheet\"
        />\n
        <link rel=\"stylesheet\" href=\"../../css/articles.css\">\n
    </head>\n
    <body>\n
        <articletitle>{}</articletitle>\n
        <article class=\"articlewrapper\">\n
        {}\n</article>\n</body>\n</html>",
        title, title, content
    );
    // End of personnal code

    let mut file = File::create("index.html")?;
    file.write_all(full.as_bytes())?; // If you deleted code above, replace `full` by `content`

    Ok(())
}
