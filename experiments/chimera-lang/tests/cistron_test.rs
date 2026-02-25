#[cfg(feature = "cistron")]
mod tests {
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::ast::{Dna, Gene, Helix, Strand, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::cistron_compiler;

    #[test]
    fn test_protein_synthesis() {
        let source = r#"
            protein Energy { decay: 0.1 }
        "#;

        let dna = cistron_compiler::compile(source).expect("Failed to compile");
        let mut vm = ChimeraVM::new(dna);

        // Run init genes (SynthesizeProtein)
        vm.step();

        let protein = vm.cistron_state.proteins.get("Energy").expect("Energy protein not created");
        // Initial amount 50 -> 0.5. Decay 0.1.
        // Step 1: Init (0.5), Decay (0.45).
        assert!(protein.concentration >= 0.44);
    }

    #[test]
    fn test_repression() {
        // Strand 0 (Main): defines regulation, then jumps to 1
        // Strand 1 (Target): Should be repressed

        let source = r#"
            protein Repressor { decay: 0.0 }
            gene Strand_1 {
                repress: Repressor 1.0
            }
        "#;

        let dna_init = cistron_compiler::compile(source).expect("Failed to compile");

        // Append Jump to Strand 1 to Strand 0
        let mut genes_0 = dna_init.helix.strands[0].genes.clone();
        genes_0.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] });

        // Strand 1: Push 42
        let genes_1 = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
            Gene { op: OpCode::Nop, args: vec![] }
        ];

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![
                    Strand { genes: genes_0 },
                    Strand { genes: genes_1 }
                ]
            }
        };

        let mut vm = ChimeraVM::new(dna);

        // Step 1: Synthesize Repressor (Conc 0.5)
        vm.step();
        // Step 2: Register Regulation (Strand_1, Repress, 1.0)
        vm.step();
        // Step 3: Jump to Strand 1
        vm.step();

        assert_eq!(vm.ip.0, 1);

        // Calculate expected expression:
        // Base 0.5 - (Conc 0.5 * Strength 1.0) = 0.0.
        // Decay is 0.0, so Conc stays 0.5.

        // Step 4: Try to execute Strand 1 Gene 0 (Push 42)
        // Should be skipped (100% prob)
        vm.step();

        assert!(vm.stack.is_empty(), "Stack should be empty (Instruction Skipped)");
        assert_eq!(vm.ip.1, 1); // Advanced to next gene

        // Verify state
        let expr = vm.cistron_state.gene_expression.get("Strand_1").unwrap();
        assert_eq!(*expr, 0.0);
    }
}
