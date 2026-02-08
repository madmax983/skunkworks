#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Ident,
    Punct,
    String,
    Comment,
    Whitespace,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
    pub line: usize,
    pub col: usize,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    let mut line = 0;
    let mut col = 0;

    while let Some(&c) = chars.peek() {
        let start_line = line;
        let start_col = col;

        if c.is_whitespace() {
            chars.next();
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            continue;
        }

        if c == '/' {
            // Check for comment
            chars.next(); // consume /
            col += 1;
            if let Some(&next_c) = chars.peek() {
                if next_c == '/' {
                    // Line comment
                    chars.next(); // consume second /
                    col += 1;
                    let mut text = String::from("//");
                    while let Some(&comment_char) = chars.peek() {
                        if comment_char == '\n' {
                            break;
                        }
                        text.push(comment_char);
                        chars.next();
                        col += 1;
                    }
                    tokens.push(Token {
                        text,
                        kind: TokenType::Comment,
                        line: start_line,
                        col: start_col,
                    });
                    continue;
                }
            }
            // Just a slash
            tokens.push(Token {
                text: "/".to_string(),
                kind: TokenType::Punct,
                line: start_line,
                col: start_col,
            });
            continue;
        }

        if c == '"' {
            // String literal
            chars.next();
            col += 1;
            let mut text = String::from("\"");
            while let Some(&str_char) = chars.peek() {
                text.push(str_char);
                chars.next();
                col += 1;
                if str_char == '"' {
                    break;
                }
            }
            tokens.push(Token {
                text,
                kind: TokenType::String,
                line: start_line,
                col: start_col,
            });
            continue;
        }

        if is_ident_start(c) {
            let mut text = String::new();
            while let Some(&id_char) = chars.peek() {
                if is_ident_char(id_char) {
                    text.push(id_char);
                    chars.next();
                    col += 1;
                } else {
                    break;
                }
            }
            let kind = if is_keyword(&text) { TokenType::Keyword } else { TokenType::Ident };
            tokens.push(Token {
                text,
                kind,
                line: start_line,
                col: start_col,
            });
            continue;
        }

        // Punctuation or unknown
        chars.next();
        col += 1;
        tokens.push(Token {
            text: c.to_string(),
            kind: TokenType::Punct,
            line: start_line,
            col: start_col,
        });
    }

    tokens
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_keyword(s: &str) -> bool {
    match s {
        "fn" | "let" | "mut" | "if" | "else" | "match" | "while" | "for" | "loop" | "return" | "struct" | "enum" | "impl" | "use" | "mod" | "pub" | "crate" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let code = "fn main() { let x = 5; }";
        let tokens = tokenize(code);

        assert_eq!(tokens[0].text, "fn");
        assert_eq!(tokens[0].kind, TokenType::Keyword);

        assert_eq!(tokens[1].text, "main");
        assert_eq!(tokens[1].kind, TokenType::Ident);

        assert_eq!(tokens[2].text, "(");
        assert_eq!(tokens[3].text, ")");
        assert_eq!(tokens[4].text, "{");

        assert_eq!(tokens[5].text, "let");
    }

    #[test]
    fn test_tokenize_string() {
        let code = "let s = \"hello\";";
        let tokens = tokenize(code);
        assert_eq!(tokens[3].text, "\"hello\"");
        assert_eq!(tokens[3].kind, TokenType::String);
    }

    #[test]
    fn test_tokenize_comment() {
        let code = "// comment\nfn";
        let tokens = tokenize(code);
        assert_eq!(tokens[0].kind, TokenType::Comment);
        assert_eq!(tokens[1].text, "fn");
        assert_eq!(tokens[1].line, 1);
    }
}
