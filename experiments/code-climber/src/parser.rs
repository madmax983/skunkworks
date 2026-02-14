#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    Comment,
    Whitespace,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
}

pub fn parse_line(line: &str, line_index: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    // Y position: line index * height (say 1.0).
    // We use negative Y so line 0 is at 0, line 1 is at -1, etc.
    let y_pos = -(line_index as f32) * 1.5; // 1.5 spacing

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Comments
        if i + 1 < chars.len() && chars[i] == '/' && chars[i+1] == '/' {
            let text: String = chars[i..].iter().collect();
            tokens.push(Token {
                x: i as f32,
                y: y_pos,
                width: text.len() as f32,
                token_type: TokenType::Comment,
                text,
            });
            break; // Rest of line is comment
        }

        // Identifiers / Keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            let token_type = match text.as_str() {
                "fn" | "let" | "pub" | "struct" | "enum" | "mod" | "use" | "impl" | "for" | "match" | "if" | "else" => TokenType::Keyword,
                _ => TokenType::Identifier,
            };
            tokens.push(Token {
                x: start as f32,
                y: y_pos,
                width: (i - start) as f32,
                token_type,
                text,
            });
            continue;
        }

        // Symbols
        let start = i;
        let text = chars[i].to_string();
        tokens.push(Token {
            x: start as f32,
            y: y_pos,
            width: 1.0,
            token_type: TokenType::Symbol,
            text,
        });
        i += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_simple() {
        let line = "fn main() {";
        let tokens = parse_line(line, 0);

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].text, "fn");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);

        assert_eq!(tokens[1].text, "main");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);

        assert_eq!(tokens[2].text, "(");
        assert_eq!(tokens[2].token_type, TokenType::Symbol);
    }

    #[test]
    fn test_parse_comment() {
        let line = "let x = 1; // comment";
        let tokens = parse_line(line, 1);
        let last = tokens.last().unwrap();
        assert_eq!(last.token_type, TokenType::Comment);
        assert_eq!(last.text, "// comment");
    }
}
