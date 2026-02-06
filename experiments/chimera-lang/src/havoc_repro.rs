#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::ChimeraVM;
    use crate::opcode::OpCode;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_junction_blowup() {
        // Attack: Exponential Junction Blowup via OpCode::Add (Cross Product)
        // 1. Create a Junction of size 2: [1, 1]
        // 2. Loop: Dup, Add.
        // Size sequence: 2 -> 4 -> 16 -> 256 -> 65536 -> 4,294,967,296 (4B) -> OOM

        let genes = vec![
            // Init: Push(1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            // Make it [1, 1] using Map("[ dup() ]")
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("[ dup() ]".to_string())] },
            Gene { op: OpCode::Map, args: vec![] },

            // Iteration 1: 2 -> 4
            Gene { op: OpCode::Dup, args: vec![] },
            Gene { op: OpCode::Add, args: vec![] },

            // Iteration 2: 4 -> 16
            Gene { op: OpCode::Dup, args: vec![] },
            Gene { op: OpCode::Add, args: vec![] },

            // Iteration 3: 16 -> 256
            Gene { op: OpCode::Dup, args: vec![] },
            Gene { op: OpCode::Add, args: vec![] },

            // Iteration 4: 256 -> 65536 (Wait, 65536 is > 1024, so it should be blocked here)
            Gene { op: OpCode::Dup, args: vec![] },
            Gene { op: OpCode::Add, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        let mut steps = 0;
        while !vm.halted && steps < 1000 {
            vm.step();
            steps += 1;
        }

        // Verify we hit the complexity limit
        let has_error = vm.output.iter().any(|s|
            s.contains("complexity limit") || s.contains("limit exceeded")
        );
        assert!(has_error, "Expected complexity/size limit error, got: {:?}", vm.output);
    }
}
