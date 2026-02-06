#[cfg(test)]
mod tests {
    use crate::ChimeraParser;
    use crate::Rule;
    use pest::Parser;

    #[test]
    fn test_parse_huge_number_safety() {
        let huge_number = "1".repeat(50); // Much larger than i64::MAX
        let code = format!("[ push({}) ]", huge_number);

        // This should NOT panic, but return an error.
        let pairs = ChimeraParser::parse(Rule::strand, &code).expect("Pest parsing failed");

        for pair in pairs {
            let result = crate::ast::Strand::try_from_pair(pair);
            assert!(result.is_err(), "Parser should return error on huge number, but succeeded");

            if let Err(msg) = result {
                assert!(msg.contains("Invalid number"), "Error message should mention invalid number, got: {}", msg);
            }
        }
    }
}
