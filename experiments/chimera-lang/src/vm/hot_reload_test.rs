#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_patch_dna_update() {
        // Initial: [ push(10) ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Execute push(10)
        assert_eq!(vm.stack.pop(), Some(Value::Int(10)));
        // vm.step() increments IP after execution if no jump.
        // Initial len 1. IP starts (0,0). execute -> IP (0,1).
        assert_eq!(vm.ip, (0, 1));

        // Patch: [ push(20) push(30) ]
        let new_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(30)] },
        ];
        let new_dna = make_dna(new_genes);

        vm.patch_dna(new_dna);

        // VM state (stack, energy) should be preserved.
        // IP was (0, 1). New strand len is 2. IP remains (0, 1).
        assert_eq!(vm.ip, (0, 1));

        // Continue execution from (0,1) which is push(30)
        vm.step(); // push(30)
        assert_eq!(vm.stack.pop(), Some(Value::Int(30)));
    }

    #[test]
    fn test_patch_dna_clamp_ip() {
        // Initial: [ push(1) push(2) push(3) ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // push(1) -> IP (0,1)
        vm.step(); // push(2) -> IP (0,2)
        assert_eq!(vm.ip, (0, 2)); // Pointing to push(3)

        // Patch: [ push(99) ] (Shorter, len 1)
        let new_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(99)] },
        ];
        let new_dna = make_dna(new_genes);

        vm.patch_dna(new_dna);

        // IP was 2. New len is 1. IP should be clamped to 0 (len - 1).
        assert_eq!(vm.ip, (0, 0));

        // Next step executes the new gene at 0
        vm.step();
        assert_eq!(vm.stack.pop(), Some(Value::Int(99)));
    }
}
