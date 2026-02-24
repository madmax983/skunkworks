#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    #[ignore]
    fn test_nightmare_trigger() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // strand_idx
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // ticks
            },
            Gene {
                op: OpCode::Dream,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.metamorphism_enabled = false;
        vm.energy = 200;

        // Set Entropy > 50 at (8,8) (default context_loc)
        // High value to survive diffusion
        vm.entropy_grid[8][8] = 1000;

        // Exec
        vm.step(); // push
        vm.step(); // push
        vm.step(); // dream

        assert!(!vm.dream_traces.is_empty());
        let trace = vm.dream_traces.last().unwrap();
        assert!(
            trace.is_nightmare,
            "Should be a nightmare due to high entropy"
        );
        assert!(trace.accepted, "Nightmares should be forced accepted");
    }

    #[test]
    fn test_lucid_cleansing() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1000)], // amount (overkill)
            },
            Gene {
                op: OpCode::Lucid,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.metamorphism_enabled = false;
        vm.energy = 2000;

        // Set Entropy
        vm.entropy_grid[8][8] = 100;

        vm.step(); // push
        vm.step(); // lucid

        assert_eq!(vm.entropy_grid[8][8], 0, "Lucid should clear entropy");
        assert!(vm.energy < 2000, "Lucid should cost energy");
    }
}
