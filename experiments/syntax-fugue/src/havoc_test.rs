#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::parser::CodeParser;

    proptest! {
        #[test]
        fn test_parser_havoc(s in "\\PC*") {
            let _ = CodeParser::parse_str(&s);
        }
    }
}
