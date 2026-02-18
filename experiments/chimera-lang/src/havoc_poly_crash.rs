#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_mitosis_memory_bomb() {
        // 👺 HAVOC: Unbounded Mitosis leading to OOM
        // The VM has limits for Spores (64), Organelles (256), Call Stack (100).
        // But it fails to limit the number of DNA strands created by Mitosis.
        // A simple loop can exponentialy (or linearly) consume memory until crash.

        let genes = vec![
            // Push(0) - target strand index to clone
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // Mitosis - clone strand 0
            Gene {
                op: OpCode::Mitosis,
                args: vec![],
            },
            // Jump(0) - loop forever
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Give it some energy so it doesn't starve immediately
        vm.energy = 1_000_000;

        let mut steps = 0;
        // Run until we create a ridiculous number of strands
        // In a real attack, this runs until OOM.
        // Here we prove the lack of limit by exceeding a reasonable safety margin (e.g. 1000).
        // Most "safe" limits in this VM are around 64-256.
        while steps < 100_000 && vm.dna.helix.strands.len() < 10_000 {
            vm.step();
            steps += 1;
        }

        println!("Strand count: {}", vm.dna.helix.strands.len());

        // We expect the strands to be capped at MAX_STRANDS (1024).
        // If it grows beyond that, the vulnerability exists.
        assert!(
            vm.dna.helix.strands.len() <= crate::vm::MAX_STRANDS,
            "Memory bomb exploded! Strands: {}",
            vm.dna.helix.strands.len()
        );
    }
}
