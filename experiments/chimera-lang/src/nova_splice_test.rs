#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Strand>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix { strands },
        }
    }

    #[test]
    fn test_splice_interleave() {
        // Strand 0: [ push(1) push(1) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
            ],
        };
        // Strand 1: [ push(2) push(2) ]
        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
            ],
        };

        let dna = make_dna(vec![strand0, strand1]);
        let mut vm = ChimeraVM::new(dna);

        // Execute Splice(0, 1, 0) -> Interleave
        // Stack: [0, 1, 0]
        vm.stack.push(Value::Int(0)); // Strand A
        vm.stack.push(Value::Int(1)); // Strand B
        vm.stack.push(Value::Int(0)); // Method 0

        vm.execute_gene_inner(OpCode::Splice, &[]);

        // Should have created Strand 2
        assert_eq!(vm.dna.helix.strands.len(), 3);
        let child = &vm.dna.helix.strands[2];
        assert_eq!(child.genes.len(), 4);

        // Expected: 1, 2, 1, 2
        // We check the first arg of each gene (which is push(N))
        if let Nucleotide::Number(n) = &child.genes[0].args[0] {
            assert_eq!(n, &1);
        } else {
            panic!("Gene 0 arg mismatch");
        }
        if let Nucleotide::Number(n) = &child.genes[1].args[0] {
            assert_eq!(n, &2);
        } else {
            panic!("Gene 1 arg mismatch");
        }
        if let Nucleotide::Number(n) = &child.genes[2].args[0] {
            assert_eq!(n, &1);
        } else {
            panic!("Gene 2 arg mismatch");
        }
        if let Nucleotide::Number(n) = &child.genes[3].args[0] {
            assert_eq!(n, &2);
        } else {
            panic!("Gene 3 arg mismatch");
        }
    }

    #[test]
    fn test_splice_midpoint() {
        // Strand 0: [ push(1) push(1) push(1) push(1) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
            ],
        };
        // Strand 1: [ push(2) push(2) push(2) push(2) ]
        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
            ],
        };

        let dna = make_dna(vec![strand0, strand1]);
        let mut vm = ChimeraVM::new(dna);

        // Execute Splice(0, 1, 2) -> Midpoint
        // Stack: [0, 1, 2]
        vm.stack.push(Value::Int(0)); // Strand A
        vm.stack.push(Value::Int(1)); // Strand B
        vm.stack.push(Value::Int(2)); // Method 2

        vm.execute_gene_inner(OpCode::Splice, &[]);

        assert_eq!(vm.dna.helix.strands.len(), 3);
        let child = &vm.dna.helix.strands[2];
        assert_eq!(child.genes.len(), 4);

        // Expected: 1, 1, 2, 2
        if let Nucleotide::Number(n) = &child.genes[0].args[0] {
            assert_eq!(n, &1);
        }
        if let Nucleotide::Number(n) = &child.genes[1].args[0] {
            assert_eq!(n, &1);
        }
        if let Nucleotide::Number(n) = &child.genes[2].args[0] {
            assert_eq!(n, &2);
        }
        if let Nucleotide::Number(n) = &child.genes[3].args[0] {
            assert_eq!(n, &2);
        }
    }
}
