#[cfg(test)]
mod tests {

    use chimera_lang::compiler::compile;
    use chimera_lang::opcode::OpCode;

    #[test]
    fn test_chaos_block() {
        let src = r#"
        strand main {
            chaos {
                push(1)
            }
        }
        "#;
        let dna = compile(src, None).expect("Compilation failed");
        let genes = &dna.helix.strands[0].genes;

        // Structure:
        // Push(0.1) -> HavocRate -> ... block ... -> Push(0) -> HavocRate

        assert_eq!(genes[0].op, OpCode::Push); // 0.1
        assert_eq!(genes[1].op, OpCode::HavocRate);

        // Inner block wrapped in strand? No, parser flattens or wraps?
        // `parse_chaos_block` recursively parses and extends genes.
        // So the inner genes are direct.
        assert_eq!(genes[2].op, OpCode::Push); // The inner push(1)

        assert_eq!(genes[3].op, OpCode::Push); // 0
        assert_eq!(genes[4].op, OpCode::HavocRate);
    }

    #[test]
    #[cfg(feature = "oracle")]
    fn test_oracle_block() {
        let src = r#"
        strand main {
            oracle {
                fact(human("socrates"))
                rule(mortal(?x)) :- human(?x)
            }
        }
        "#;
        let dna = compile(src, None).expect("Compilation failed");
        let genes = &dna.helix.strands[0].genes;

        // Fact assertion
        // Push(Junction(Fact)) -> Assert
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[1].op, OpCode::Assert);

        // Rule definition
        // Push(Head) -> Push(Body) -> Rule
        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[3].op, OpCode::Push);
        assert_eq!(genes[4].op, OpCode::Rule);
    }
}
