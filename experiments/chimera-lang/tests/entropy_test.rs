#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::vm::nova::OrganelleType;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_entropy_basics() {
        // [ entropy() disintegrate(0, 0) entropy() stabilize(50) entropy() ]
        let genes = vec![
            // 1. Check initial entropy
            Gene { op: OpCode::Entropy, args: vec![] },
            // 2. Disintegrate (0,0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Disintegrate, args: vec![] },
            // 3. Check entropy (should be 100)
            Gene { op: OpCode::Entropy, args: vec![] },
            // 4. Stabilize 50
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
            Gene { op: OpCode::Stabilize, args: vec![] },
            // 5. Check entropy (should be 50)
            Gene { op: OpCode::Entropy, args: vec![] },
        ];
        let gene_count = genes.len();
        let mut vm = ChimeraVM::new(make_dna(genes));
        // Ensure context_loc is (0,0) for the test
        vm.context_loc = (0, 0);
        vm.energy = 1000; // Enough energy

        while !vm.halted && vm.ip.1 < gene_count {
            vm.step();
        }

        // Stack: [0, high_entropy, low_entropy]
        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.stack[0], Value::Int(0));

        let high_e = if let Value::Int(v) = vm.stack[1] { v } else { -1 };
        assert!(high_e > 40, "Entropy should be high (was {})", high_e);

        let low_e = if let Value::Int(v) = vm.stack[2] { v } else { -1 };
        assert!(low_e < high_e, "Entropy should be reduced (was {})", low_e);
    }

    #[test]
    fn test_void_trail() {
        // Spawn Void, let it tick. Check entropy.
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Strand
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] }, // Type Void
            Gene { op: OpCode::Spawn, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8);
        vm.energy = 1000;

        // Run spawn
        vm.step(); vm.step(); vm.step();

        assert_eq!(vm.organelles.len(), 1);

        // Let it tick a few times
        // Organelles tick in vm.step() if not frozen
        for _ in 0..5 {
            vm.step();
        }

        // We can't easily predict exact location (Brownian motion),
        // but we can check if total entropy > 0.
        let mut total_entropy = 0;
        for row in &vm.entropy_grid {
            for cell in row {
                total_entropy += cell;
            }
        }
        assert!(total_entropy > 0, "Void should create entropy trail");
    }
}
