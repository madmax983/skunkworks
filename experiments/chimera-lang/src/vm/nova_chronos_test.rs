#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    #[cfg(feature = "nova")]
    fn test_chronostasis() {
        // [ push(5) chronostasis() push(10) consume() jump(2) ]
        // Loop to keep VM alive while timer ticks.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Chronostasis,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)], // Jump to strand 0
            },
        ];

        // Wait, Jump(0) jumps to (0,0).
        // That re-executes Push(5) and Chronostasis!
        // That resets the timer!
        // We want to jump to (0, 2).
        // Chimera `Jump` opcode takes Strand Index. It jumps to (strand_idx, 0).
        // `JumpS` jumps to strand index from stack.
        // There is no intra-strand jump to specific gene index in standard ops?
        // Ah, `jump` is only to start of strand.

        // Solution: Split into strands.
        // Strand 0: [ Push(5) Chronostasis Jump(1) ]
        // Strand 1: [ Push(10) Consume Jump(1) ]

        let s0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
                Gene { op: OpCode::Chronostasis, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
            ]
        };

        let s1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
                Gene { op: OpCode::Consume, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
            ]
        };

        let mut vm = ChimeraVM::new(Dna {
            helix: Helix {
                strands: vec![s0, s1],
            },
        });
        vm.energy = 1000; // Give enough energy

        // Setup environment
        let (cy, cx) = vm.context_loc;
        vm.waste_grid[cy][cx] = 100;

        // Step 1: Push(5)
        vm.step();
        // Step 2: Chronostasis
        vm.step();
        assert_eq!(vm.chronostasis_timer, 5);

        // Step 3: Jump(1)
        vm.step();
        // Now ip is (1,0)

        // Capture waste state
        let diff_check = vm.waste_grid[cy][cx];

        // Run loop in Strand 1.
        // Timer was 4 after Jump.
        // Step 4: Push(10). Timer -> 3.
        vm.step();
        assert_eq!(vm.chronostasis_timer, 3);
        assert_eq!(vm.waste_grid[cy][cx], diff_check);

        // Run until timer expires
        vm.step(); // Consume. Timer -> 3
        vm.step(); // Jump(1). Timer -> 2
        vm.step(); // Push(10). Timer -> 1
        vm.step(); // Consume. Timer -> 0

        assert_eq!(vm.chronostasis_timer, 0);

        // Step 9: Jump(1). Normal execution resumes.
        vm.step();
        // Step 10: Push(10). Env process runs.
        vm.step();

        // Now waste should change (diffusion + accumulation)
        assert_ne!(vm.waste_grid[cy][cx], diff_check);
    }
}
