#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, MAX_STRANDS};

    #[test]
    fn test_cambrian_unbounded_strands_dos() {
        let genes = vec![
            Gene {
                op: OpCode::Cambrian,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 10000000; // Infinite energy

        // Loop Cambrian. Each call clears organelles but ADDS strands.

        let start_strands = vm.dna.helix.strands.len();

        // Run enough steps to definitely overflow MAX_STRANDS (1024)
        // 100 Cambrians * 128 strands = 12800 strands
        for _ in 0..200 {
            vm.step();
        }

        let end_strands = vm.dna.helix.strands.len();

        println!("Strands: {} -> {}", start_strands, end_strands);

        assert!(end_strands <= MAX_STRANDS + 128, "Strand count exploded beyond safety margin! Got {}", end_strands);
    }
}
