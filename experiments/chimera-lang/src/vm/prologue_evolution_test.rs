#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let genes0 = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        }];
        // Strand 1: [ Add, Sub ]
        let genes1 = vec![
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            Gene {
                op: OpCode::Sub,
                args: vec![],
            }
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![
                    Strand { genes: genes0 }, // Index 0
                    Strand { genes: genes1 }, // Index 1
                ],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_length_rune() {
        let mut vm = setup_vm();
        // Setup: 1 (Idx) -> ! -> l
        vm.grid[4][4] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("l".to_string());

        { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

        // Check if l outputted 2 (length of strand 1)
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Int(2)));
    }

    #[test]
    fn test_nucleotide_rune() {
        let mut vm = setup_vm();
        // Setup:
        //   1 (Gene Idx)
        //   !
        // 1 (Strand Idx) -> ! -> n

        vm.grid[3][5] = Value::Int(1); // Gene 1 (Sub)
        vm.grid[4][5] = Value::Str("!".to_string());

        vm.grid[4][4] = Value::Int(1); // Strand 1
        vm.grid[5][4] = Value::Str("!".to_string());

        vm.grid[5][5] = Value::Str("n".to_string());

        { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

        // Check if n outputted "sub" (OpCode of gene 1 in strand 1)
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Str("sub".to_string())));
    }

    #[test]
    fn test_genesis_rune() {
        let mut vm = setup_vm();
        // Setup: "strand temp { add }" -> ! -> G
        let code = "strand temp { add }";
        vm.grid[3][5] = Value::Str(code.to_string());
        vm.grid[4][5] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("G".to_string());

        { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

        if vm.prologue_state.signal_grid[5][5].is_none() {
            println!("VM Output: {:?}", vm.output);
        }

        // Check if G created a new strand (Index 2) and outputted 2
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Int(2)));
        assert_eq!(vm.dna.helix.strands.len(), 3);
    }

    #[test]
    fn test_evolve_rune() {
        let mut vm = setup_vm();
        // Setup: 1 (Idx) -> ! -> e
        vm.grid[4][4] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("e".to_string());

        { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

        // Check if e created a mutated copy (Index 2)
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Int(2)));
        assert_eq!(vm.dna.helix.strands.len(), 3);

        // Check if new strand is different or same length (mutation might change length)
        // Original strand 1 has length 2.
        // Mutation logic: 50% delete, 50% duplicate.
        // New length should be 1 or 3.
        let new_len = vm.dna.helix.strands[2].genes.len();
        assert!(new_len == 1 || new_len == 3);
    }
}
