use crate::scanner::tokens::{Token, TokenType};

pub fn parse_tokens(toks: Vec<Token>) -> String {
    let mut content = String::new();
    for i in 0..toks.len() {
        let tok = &toks[i];
        if i == 0 {
            match tok.toktype {
                TokenType::Char(c) => {
                    content.push_str("<p>");
                    content.push(c);
                }
                TokenType::Br => (),
                TokenType::H1(ref title) => {
                    content.push_str(format!("<h1>{}</h1>\n", title).as_str())
                }
                TokenType::H2(ref title) => {
                    content.push_str(format!("<h2>{}</h2>\n", title).as_str())
                }
                TokenType::H3(ref title) => {
                    content.push_str(format!("<h3>{}</h3>\n", title).as_str())
                }
                TokenType::Img(ref alt, ref link) => {
                    content.push_str(format!("<img src=\"{}\" alt=\"{}\"/>\n", link, alt).as_str())
                }
                TokenType::Link(ref alt, ref link) => {
                    content.push_str(format!("<a href=\"{}\">{}</a>\n", link, alt).as_str())
                }
                TokenType::Hr => content.push_str("<hr>\n"),
                TokenType::Code(ref code) => {
                    content.push_str(format!("<code>{}</code>", code).as_str())
                }
                TokenType::Pre(ref code) => {
                    content.push_str(format!("<pre>{}</pre>", code).as_str())
                }
            }
        } else {
            match tok.toktype {
                TokenType::Char(c) => {
                    if let TokenType::Char(_) = toks[i - 1].toktype {
                        content.push(c);
                    } else if let TokenType::Code(_) = toks[i - 1].toktype {
                        content.push(c);
                    } else if let TokenType::Link(_, _) = toks[i - 1].toktype {
                        content.push(c);
                    } else {
                        content.push_str("<p>");
                        content.push(c);
                    }
                    if i + 1 >= toks.len() {
                        content.push_str("</p>\n");
                    } else if let TokenType::Char(_) = toks[i + 1].toktype {
                        continue;
                    } else if let TokenType::Code(_) = toks[i + 1].toktype {
                        content.push(c);
                    } else if let TokenType::Link(_, _) = toks[i + 1].toktype {
                        content.push(c);
                    } else {
                        content.push_str("</p>\n")
                    }
                }
                TokenType::Br => (),
                TokenType::H1(ref title) => content.push_str(
                    format!(
                        "<h1 id=\"{}\">{}</h1>\n",
                        title.as_str().replace(" ", "-").to_lowercase(),
                        title
                    )
                    .as_str(),
                ),
                TokenType::H2(ref title) => content.push_str(
                    format!(
                        "<h2 id=\"{}\">{}</h2>\n",
                        title.as_str().replace(" ", "-").to_lowercase(),
                        title
                    )
                    .as_str(),
                ),
                TokenType::H3(ref title) => content.push_str(
                    format!(
                        "<h3 id=\"{}\">{}</h3>\n",
                        title.as_str().replace(" ", "-").to_lowercase(),
                        title
                    )
                    .as_str(),
                ),
                TokenType::Img(ref alt, ref link) => {
                    content.push_str(format!("<img src=\"{}\" alt=\"{}\"/>\n", link, alt).as_str())
                }
                TokenType::Link(ref alt, ref link) => {
                    content.push_str(format!("<a href=\"{}\">{}</a><br>\n", link, alt).as_str())
                }
                TokenType::Hr => content.push_str("<hr>\n"),
                TokenType::Code(ref code) => {
                    content.push_str(format!("<code>{}</code>", code).as_str())
                }
                TokenType::Pre(ref code) => {
                    content.push_str(format!("<pre>{}</pre>", code).as_str())
                }
            }
        }
    }
    content
}
