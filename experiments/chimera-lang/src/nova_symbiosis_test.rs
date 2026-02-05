#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>, strand2: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }, Strand { genes: strand2 }],
            },
        }
    }

    #[test]
    fn test_symbiosis_absorption() {
        // Strand 0: [ spawn(0, 1) push(0) push(1) symbiosis() ]
        // Strand 1: [ push(999) ] (Organelle code)

        let strand0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Strand 1
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Type Worker
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dx = 0 (same location as spawn inherits context)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dy = 0
            },
            Gene {
                op: OpCode::Symbiosis,
                args: vec![],
            },
        ];

        let strand1 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(strand0, strand1));
        vm.energy = 100; // Ensure enough energy for operations

        // Step 1: Push args
        vm.step();
        vm.step();
        // Step 3: Spawn
        vm.step();

        assert_eq!(vm.organelles.len(), 1, "Organelle should be spawned");

        // Step 4: Push dx, dy
        vm.step();
        vm.step();

        // Step 6: Symbiosis
        vm.step();

        assert_eq!(vm.organelles.len(), 0, "Organelle should be absorbed");
        assert_eq!(vm.symbiotes.len(), 1, "Symbiote should be added");
    }

    #[test]
    fn test_symbiosis_concurrent_execution() {
        // Strand 0: [ spawn(0, 1) push(0) push(0) symbiosis() push(100) ]
        // Strand 1: [ push(200) ] (Symbiote)
        // Expected behavior:
        // After symbiosis, VM executes push(100).
        // Symbiote executes push(200).
        // Stack should contain both.

        let strand0 = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Spawn, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Symbiosis, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            // Add a wait loop or more ops to allow symbiote to run
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Infinite loop to keep running
        ];

        let strand1 = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(200)] },
            Gene { op: OpCode::SIndex, args: vec![] },
            Gene { op: OpCode::JumpS, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(strand0, strand1));
        vm.energy = 100;

        // Advance to Symbiosis
        for _ in 0..6 {
            vm.step();
        }

        assert_eq!(vm.symbiotes.len(), 1, "Symbiote added");

        // Next step: VM does push(100). Symbiote does push(200).
        // Order depends on implementation. process_symbiotes is called AFTER execute_gene.
        // So VM pushes 100 first, then Symbiote pushes 200.

        vm.step();

        assert!(vm.stack.contains(&Value::Int(100)), "Main thread ran");
        assert!(vm.stack.contains(&Value::Int(200)), "Symbiote ran");
    }

    #[test]
    fn test_lysis() {
        // Strand 0: [ spawn(0, 1) push(0) push(0) symbiosis() lysis() ]
        // Strand 1: [ push(999) ]

         let strand0 = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Spawn, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Symbiosis, args: vec![] },
            Gene { op: OpCode::Lysis, args: vec![] },
        ];

        let strand1 = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(999)] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let mut vm = ChimeraVM::new(make_dna(strand0, strand1));
        vm.energy = 100;

        // Run until lysis
        for _ in 0..7 {
            vm.step();
        }

        assert_eq!(vm.symbiotes.len(), 0, "Symbiote should be ejected");
        assert_eq!(vm.organelles.len(), 1, "Organelle should be created");
    }
}
