#![cfg(test)]
#[cfg(feature = "nova")]
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
    fn test_evolve_blinker() {
        // Setup Blinker pattern (Vertical line of 3)
        // Center at 5,5.
        // (4,5), (5,5), (6,5)
        let setup_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Evolve,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(setup_genes));

        // Execute writes (4 ops * 3 = 12 steps)
        for _ in 0..12 {
            vm.step();
        }
        // Execute Evolve
        vm.step();

        // Check new state: Horizontal line (5,4), (5,5), (5,6)
        match vm.grid[5][4] {
            Value::Int(1) => {}
            _ => panic!("(5,4) should be alive"),
        }
        match vm.grid[5][5] {
            Value::Int(1) => {}
            _ => panic!("(5,5) should be alive"),
        }
        match vm.grid[5][6] {
            Value::Int(1) => {}
            _ => panic!("(5,6) should be alive"),
        }

        // Check old vertical tips died
        match vm.grid[4][5] {
            Value::Int(0) => {}
            _ => panic!("(4,5) should be dead"),
        }
        match vm.grid[6][5] {
            Value::Int(0) => {}
            _ => panic!("(6,5) should be dead"),
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
