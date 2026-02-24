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
    fn test_claim_and_sovereignty() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Sovereignty,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000; // Give enough energy for claim

        vm.step();
        vm.step();

        if vm.sovereignty_grid[8][8] != Some(0) {
            println!("Output: {:?}", vm.output);
        }
        assert_eq!(vm.sovereignty_grid[8][8], Some(0));
        assert_eq!(vm.sovereignty_grid[8][9], Some(0));
        assert_eq!(vm.sovereignty_grid[8][10], None);

        vm.step();
        vm.step();
        vm.step();

        let result = vm.stack.pop();
        if result != Some(Value::Int(0)) {
            println!("Output: {:?}", vm.output);
            println!("Stack: {:?}", vm.stack);
        }
        assert_eq!(result, Some(Value::Int(0)));
    }

    #[test]
    fn test_cede() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Cede,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Sovereignty,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        for _ in 0..5 {
            vm.step();
        }

        if vm.sovereignty_grid[8][8] != None {
            println!("Output after Cede: {:?}", vm.output);
        }
        assert_eq!(vm.sovereignty_grid[8][8], None);
        assert_eq!(vm.sovereignty_grid[8][9], Some(0));

        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.stack.pop(), Some(Value::Int(-1)));
    }

    #[test]
    fn test_taxation() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Tax,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ];

        let s0 = Strand { genes };

        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Migrate,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // Setup (5 ticks)
        for _ in 0..5 {
            vm.step();
        }

        assert_eq!(vm.ip.0, 1);

        // Execute Strand 1 (3 ticks)
        for _ in 0..3 {
            vm.step();
        }

        let balance = vm.market.get_balance(0);
        assert_eq!(balance, 15);
    }
}
