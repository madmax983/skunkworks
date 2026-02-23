#[cfg(test)]
mod tests {
    use chimera_lang::ast::Nucleotide;
    use chimera_lang::compiler::compile;
    use chimera_lang::opcode::OpCode;

    #[test]
    fn test_polyglot_grammar() {
        let src = r#"
        grammar my_math {
            Map(
                Seq(
                    Int(Regex("[0-9]+")),
                    Regex("[ \t\n]+"),
                    Int(Regex("[0-9]+")),
                    Regex("[ \t\n]+"),
                    Match("add")
                ),
                all(
                    any("push", ?1),
                    any("push", ?3),
                    any("add")
                )
            )
        }

        strand main {
            polyglot my_math {
                10 20 add
            }
        }
        "#;

        let dna = compile(src, None).expect("Compilation failed");
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes.len(), 3);

        assert_eq!(genes[0].op, OpCode::Push);
        // Should be Number(10)
        assert_eq!(genes[0].args[0], Nucleotide::Number(10));

        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(20));

        assert_eq!(genes[2].op, OpCode::Add);
    }
}
