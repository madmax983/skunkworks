#[cfg(test)]
mod tests {
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use chimera_lang::opcode::OpCode;

    #[test]
    fn test_piet_dos() {
        println!("Testing Piet DoS...");

        // [ push(100_000) piet() ]
        // This program attempts to run Piet for 100,000 steps.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100_000)],
            },
            Gene {
                op: OpCode::Piet,
                args: vec![],
            },
        ];
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![Strand { genes }] }
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100_000;

        vm.step();

        // Check for clamping warning
        let clamped = vm.output.iter().any(|s| s.contains("PIET: Clamped steps to MAX"));
        assert!(clamped, "Piet steps should be clamped!");
    }
}
