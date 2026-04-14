#![cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_scramble() {
        // [ push(1) push(2) push(3) push(4) push(5) scramble() ]
        let mut genes = Vec::new();
        for i in 1..=5 {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i)],
            });
        }
        genes.push(Gene {
            op: OpCode::Scramble,
            args: vec![],
        });

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute pushes
        for _ in 0..5 {
            vm.step();
        }
        // Execute scramble
        vm.step();

        assert_eq!(vm.stack.len(), 5);
        // Check if elements are preserved
        let mut sum = 0;
        for val in &vm.stack {
            if let Value::Int(n) = val {
                sum += n;
            }
        }
        assert_eq!(sum, 1 + 2 + 3 + 4 + 5);
    }
}
