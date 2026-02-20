#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_sigil::Sigil;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_sigil_activation() {
        let mut vm = make_vm();

        // Strand 0: Push(100) -> GWrite(2,2)
        let strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
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
                    op: OpCode::GWrite,
                    args: vec![],
                },
            ],
        };
        vm.dna.helix.strands.push(strand);

        // Define Sigil "TestSigil"
        let pattern = vec![
            (-1, 0, Value::Str("A".to_string())),
            (1, 0, Value::Str("B".to_string())),
        ];

        let sigil = Sigil {
            pattern,
            strand_idx: 0,
            auto_cast: false,
        };

        vm.sigil_registry.insert("TestSigil".to_string(), sigil);

        vm.grid[0][1] = Value::Str("A".to_string());
        vm.grid[1][1] = Value::Str("§".to_string());
        vm.grid[2][1] = Value::Str("B".to_string());

        // Signal §
        vm.signal_grid[1][1] = 1;

        // Process signals manually first to trigger call
        process_signals(&mut vm);

        // Verify IP jump
        assert_eq!(vm.ip, (0, 0));

        // Check if Pattern Consumed
        match &vm.grid[0][1] {
            Value::Int(0) => {}
            _ => panic!(
                "Expected pattern consumption (A->0), got {:?}",
                vm.grid[0][1]
            ),
        }

        // Run the strand (4 steps)
        // Disable orca mode to avoid interference during execution test
        vm.orca_mode = false;
        for _ in 0..5 {
            vm.step();
        }

        match &vm.grid[2][2] {
            Value::Int(n) => assert_eq!(*n, 100),
            Value::Str(s) => assert_eq!(s, "100"),
            _ => panic!("Expected result 100 at (2,2), got {:?}", vm.grid[2][2]),
        }
    }

    #[test]
    #[cfg(feature = "oracle")]
    fn test_kb_operator() {
        let mut vm = make_vm();

        let strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(99)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(3)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(3)],
                },
                Gene {
                    op: OpCode::GWrite,
                    args: vec![],
                },
            ],
        };
        vm.dna.helix.strands.push(strand);

        let fact = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("operator".to_string()),
                Value::Str("MYOP".to_string()),
                Value::Int(0),
            ],
        );
        vm.knowledge_base.push(fact);

        vm.grid[1][1] = Value::Str("MYOP".to_string());
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.ip, (0, 0));

        vm.orca_mode = false;
        for _ in 0..5 {
            vm.step();
        }

        match &vm.grid[3][3] {
            Value::Int(n) => assert_eq!(*n, 99),
            Value::Str(s) => assert_eq!(s, "99"),
            _ => panic!("Expected result 99 at (3,3), got {:?}", vm.grid[3][3]),
        }
    }
}
