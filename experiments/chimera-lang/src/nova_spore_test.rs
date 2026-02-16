#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_spore_backtracking() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Swap,
                args: vec![],
            },
            Gene {
                op: OpCode::Germinate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        while !vm.halted {
            vm.step();
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_spore_mechanics() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        vm.step();
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.spores.len(), 0);

        vm.step();
        assert_eq!(vm.spores.len(), 1);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[1], Value::Int(0));

        let spore = &vm.spores[0];
        assert_eq!(spore.stack.len(), 1);
        assert_eq!(spore.stack[0], Value::Int(10));
        assert_eq!(spore.ip, (0, 1));

        vm.step();
        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.stack[2], Value::Int(20));

        vm.stack.push(Value::Int(0));

        vm.execute_gene_inner(OpCode::Germinate, &[]);

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.ip, (0, 1));

        vm.step();
        assert_eq!(vm.spores.len(), 2);
        assert_eq!(vm.stack.len(), 2);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_dna_restoration() {
        let genes = vec![
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Transcribe,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        vm.step();
        assert_eq!(vm.spores.len(), 1);

        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(99)));

        vm.step();
        vm.step();
        vm.step();

        vm.stack.push(Value::Int(77));

        vm.execute_gene_inner(OpCode::Transcribe, &[]);

        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[1].args[0] {
            assert_eq!(*n, 77);
        } else {
            panic!("DNA mutation failed");
        }

        vm.stack.push(Value::Int(0));
        vm.execute_gene_inner(OpCode::Germinate, &[]);

        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[1].args[0] {
            assert_eq!(*n, 99);
        } else {
            panic!("DNA restoration failed");
        }
    }
}
