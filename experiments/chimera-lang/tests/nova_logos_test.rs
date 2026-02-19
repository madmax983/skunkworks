#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix, JunctionType, Strand};

    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[cfg(feature = "oracle")]
    #[test]
    fn test_logos_reaction() {
        let mut vm = make_empty_vm();

        // 1. Define Reaction: reaction(acid, base, salt)
        // Fact: Junction(Any, ["reaction", "acid", "base", "salt"])
        let reaction_fact = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("reaction".to_string()),
                Value::Str("acid".to_string()),
                Value::Str("base".to_string()),
                Value::Str("salt".to_string()),
            ],
        );
        vm.knowledge_base.push(reaction_fact);

        // 2. Setup Grid
        // (5, 5) = acid
        // (5, 6) = base
        vm.grid[5][5] = Value::Str("acid".to_string());
        vm.grid[5][6] = Value::Str("base".to_string());

        // 3. Enable Logos Mode (Optional if we call process_logos directly, but let's test integration)
        vm.logos_mode = true;

        // 4. Run Step (calls process_logos)
        // We need to enable "nova" feature for this test to run
        // Step calls process_logos
        vm.step();

        // 5. Verify Result
        // (5, 5) should be "salt" (Agent transforms)
        // (5, 6) should be 0 (Reagent consumed)

        // Note: process_logos iterates.
        // If (5,5) is agent, neighbor (5,6) is base. Match.
        // Updates: (5,5)->salt, (5,6)->0.
        // If (5,6) is agent, neighbor (5,5) is acid.
        // Does reaction(base, acid, ?) match? No, unless we define commutative rule.
        // So order matters. (5,5) comes before (5,6) in iteration.

        assert_eq!(vm.grid[5][5], Value::Str("salt".to_string()));
        assert_eq!(vm.grid[5][6], Value::Int(0));
    }

    #[cfg(feature = "oracle")]
    #[test]
    fn test_logos_complex_reaction() {
        let mut vm = make_empty_vm();

        // reaction(combine(A), combine(B), merged(A, B))
        // Rule: reaction(combine(?A), combine(?B), merged(?A, ?B))
        // We need to use "rule" format because it has variables?
        // No, if we use "rule", it implies Body.
        // Here we just want a Fact with variables that Unify with anything.
        // Ideally: reaction(combine(?A), combine(?B), merged(?A, ?B)) in KB.

        let reaction_fact = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("reaction".to_string()),
                Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("combine".to_string()),
                        Value::Str("?A".to_string()),
                    ],
                ),
                Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("combine".to_string()),
                        Value::Str("?B".to_string()),
                    ],
                ),
                Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("merged".to_string()),
                        Value::Str("?A".to_string()),
                        Value::Str("?B".to_string()),
                    ],
                ),
            ],
        );
        vm.knowledge_base.push(reaction_fact);

        vm.grid[2][2] = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("combine".to_string()), Value::Int(1)],
        );
        vm.grid[2][3] = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("combine".to_string()), Value::Int(2)],
        );

        vm.logos_mode = true;
        vm.step();

        // Expected: merged(1, 2) at [2][2]
        let expected = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("merged".to_string()),
                Value::Int(1),
                Value::Int(2),
            ],
        );
        assert_eq!(vm.grid[2][2], expected);
        assert_eq!(vm.grid[2][3], Value::Int(0));
    }
}
