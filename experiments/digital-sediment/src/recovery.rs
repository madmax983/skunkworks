use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::many0,
    IResult,
};

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn recover(text: &str) -> Vec<(String, bool)> {
        // Group 1: Keywords
        fn keywords(input: &str) -> IResult<&str, (String, bool)> {
            alt((
                map(tag("fn"), |s: &str| (s.to_string(), true)),
                map(tag("let"), |s: &str| (s.to_string(), true)),
                map(tag("mut"), |s: &str| (s.to_string(), true)),
                map(tag("pub"), |s: &str| (s.to_string(), true)),
                map(tag("struct"), |s: &str| (s.to_string(), true)),
                map(tag("impl"), |s: &str| (s.to_string(), true)),
                map(tag("use"), |s: &str| (s.to_string(), true)),
                map(tag("mod"), |s: &str| (s.to_string(), true)),
                map(tag("crate"), |s: &str| (s.to_string(), true)),
                map(tag("match"), |s: &str| (s.to_string(), true)),
                map(tag("if"), |s: &str| (s.to_string(), true)),
                map(tag("else"), |s: &str| (s.to_string(), true)),
                map(tag("return"), |s: &str| (s.to_string(), true)),
                map(tag("for"), |s: &str| (s.to_string(), true)),
                map(tag("while"), |s: &str| (s.to_string(), true)),
                map(tag("loop"), |s: &str| (s.to_string(), true)),
            ))(input)
        }

        // Group 2: Types
        fn types(input: &str) -> IResult<&str, (String, bool)> {
            alt((
                map(tag("String"), |s: &str| (s.to_string(), true)),
                map(tag("Option"), |s: &str| (s.to_string(), true)),
                map(tag("Result"), |s: &str| (s.to_string(), true)),
                map(tag("Vec"), |s: &str| (s.to_string(), true)),
                map(tag("u8"), |s: &str| (s.to_string(), true)),
                map(tag("i32"), |s: &str| (s.to_string(), true)),
                map(tag("f64"), |s: &str| (s.to_string(), true)),
                map(tag("usize"), |s: &str| (s.to_string(), true)),
                map(tag("bool"), |s: &str| (s.to_string(), true)),
            ))(input)
        }

        // Group 3: Macros & Symbols
        fn symbols(input: &str) -> IResult<&str, (String, bool)> {
            alt((
                map(tag("println!"), |s: &str| (s.to_string(), true)),
                map(tag("format!"), |s: &str| (s.to_string(), true)),
                map(tag("::"), |s: &str| (s.to_string(), true)),
                map(tag("->"), |s: &str| (s.to_string(), true)),
                map(tag("=>"), |s: &str| (s.to_string(), true)),
                map(tag("{"), |s: &str| (s.to_string(), true)),
                map(tag("}"), |s: &str| (s.to_string(), true)),
                map(tag("("), |s: &str| (s.to_string(), true)),
                map(tag(")"), |s: &str| (s.to_string(), true)),
                map(tag("["), |s: &str| (s.to_string(), true)),
                map(tag("]"), |s: &str| (s.to_string(), true)),
                map(tag(";"), |s: &str| (s.to_string(), true)),
            ))(input)
        }

        // Fallback
        fn fallback(input: &str) -> IResult<&str, (String, bool)> {
            map(anychar, |c: char| (c.to_string(), false))(input)
        }

        let mut parser = many0(alt((
            keywords,
            types,
            symbols,
            fallback
        )));

        match parser(text) {
            Ok((_, tokens)) => tokens,
            Err(_) => vec![(text.to_string(), false)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_keywords() {
        let text = "fn main() { let x = 5; }";
        let recovered = RecoveryEngine::recover(text);

        let has_let = recovered.iter().any(|(s, b)| s == "let" && *b);
        assert!(has_let, "Should recover 'let'");

        let has_fn = recovered.iter().any(|(s, b)| s == "fn" && *b);
        assert!(has_fn, "Should recover 'fn'");
    }
}
