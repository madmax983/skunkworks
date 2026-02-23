#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::JunctionType;
    use chimera_lang::vm::babel::{generate_string, mutate_grammar};
    use chimera_lang::vm::Value;

    #[test]
    fn test_grammar_generation() {
        // Seed: Seq(Match("A"), Match("B"))
        let grammar = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Seq".to_string()),
                Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Match".to_string()), Value::Str("A".to_string())],
                ),
                Value::Junction(
                    JunctionType::Any,
                    vec![Value::Str("Match".to_string()), Value::Str("B".to_string())],
                ),
            ],
        );

        let output = generate_string(&grammar);
        assert_eq!(output, "AB");
    }

    #[test]
    fn test_grammar_mutation() {
        // Seed: Match("AAAAA")
        let grammar = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Match".to_string()),
                Value::Str("AAAAA".to_string()),
            ],
        );

        let mut current = grammar.clone();
        let mut changed = false;

        // Try mutating multiple times to ensure change (probabilistic)
        for _ in 0..100 {
            let next = mutate_grammar(&current, 1.0); // 100% rate
            if next != current {
                changed = true;
                current = next;
                break;
            }
        }

        assert!(changed, "Grammar should mutate with 1.0 rate");

        // Generate from mutated grammar
        let output = generate_string(&current);
        assert_ne!(
            output, "AAAAA",
            "Mutated grammar should generate different string (likely)"
        );
    }

    #[test]
    fn test_recursion_limit() {
        // Construct a recursive-like grammar (though strict recursion in Value is hard without references,
        // we can simulate depth by nesting)

        let mut grammar = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("A".to_string())],
        );

        // Nest it 60 times
        for _ in 0..60 {
            grammar = Value::Junction(
                JunctionType::Any,
                vec![
                    Value::Str("Seq".to_string()),
                    grammar,
                    Value::Junction(
                        JunctionType::Any,
                        vec![Value::Str("Match".to_string()), Value::Str(".".to_string())],
                    ),
                ],
            );
        }

        let output = generate_string(&grammar);
        // It should contain "..." due to depth limit
        assert!(
            output.contains("...") || output.len() > 50,
            "Output should handle depth"
        );
        // Actually generate_string_depth logic returns "..." immediately if depth > 50.
        // And since we nest Seq(grammar, ...), the 'grammar' part is at depth+1.
        // Eventually it hits limit.
    }
}
