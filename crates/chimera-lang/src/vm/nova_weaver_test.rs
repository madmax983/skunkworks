#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_weaver::exec_weave_op;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_weave_pattern_string() {
        let mut vm = make_vm();

        // Strand 0 (A): [ Push(1) ]
        let s0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        // Strand 1 (B): [ Push(2) ]
        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }],
        };

        vm.dna.helix.strands.push(s0);
        vm.dna.helix.strands.push(s1);

        // Pattern: "ABA"
        vm.stack.push(Value::Int(0)); // A
        vm.stack.push(Value::Int(1)); // B
        vm.stack.push(Value::Str("ABA".to_string())); // Pattern

        exec_weave_op(&mut vm, OpCode::Weave, &[]);

        // Should create Strand 2: [ Push(1), Push(2), Push(1) ]
        // Wait, strand A only has 1 gene. "ABA" requests A (idx 0) -> Push(1). ptr_a=1.
        // Then B (idx 0) -> Push(2). ptr_b=1.
        // Then A (idx 1) -> Out of bounds.

        // Let's verify what happens. My implementation checks `if ptr_a < len`. If not, it skips.
        // So result should be [ Push(1), Push(2) ].

        assert_eq!(vm.dna.helix.strands.len(), 3);
        let s2 = &vm.dna.helix.strands[2];
        assert_eq!(s2.genes.len(), 2);

        assert_eq!(s2.genes[0].args[0], Nucleotide::Number(1));
        assert_eq!(s2.genes[1].args[0], Nucleotide::Number(2));
    }

    #[test]
    fn test_weave_pattern_strand() {
        let mut vm = make_vm();

        // Strand 0 (A): [ Push(1) ]
        let s0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        // Strand 1 (B): [ Push(2) ]
        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }],
        };

        // Strand 2 (Pattern): [ Add(A..), Brz(B..) ] -> Pattern "AB"
        let s2_pat = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Add,
                    args: vec![],
                }, // Starts with 'A'
                Gene {
                    op: OpCode::Brz,
                    args: vec![],
                }, // Starts with 'B'
            ],
        };

        vm.dna.helix.strands.push(s0);
        vm.dna.helix.strands.push(s1);
        vm.dna.helix.strands.push(s2_pat);

        // Stack: A=0, B=1, Pattern=2
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Int(2));

        exec_weave_op(&mut vm, OpCode::Weave, &[]);

        assert_eq!(vm.dna.helix.strands.len(), 4);
        let s3 = &vm.dna.helix.strands[3];

        // Should correspond to "AB"
        // 0 -> A -> Push(1)
        // 1 -> B -> Push(2)
        assert_eq!(s3.genes.len(), 2);
        assert_eq!(s3.genes[0].args[0], Nucleotide::Number(1));
        assert_eq!(s3.genes[1].args[0], Nucleotide::Number(2));
    }

    #[test]
    fn test_unravel() {
        let mut vm = make_vm();

        // Strand 0: 3 genes
        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
            ],
        };
        vm.dna.helix.strands.push(s0);
        vm.energy = 0;

        vm.stack.push(Value::Int(0));
        exec_weave_op(&mut vm, OpCode::Unravel, &[]);

        // Strand should be empty
        assert!(vm.dna.helix.strands[0].genes.is_empty());
        // Energy should be 3
        assert_eq!(vm.energy, 3);
    }
}
