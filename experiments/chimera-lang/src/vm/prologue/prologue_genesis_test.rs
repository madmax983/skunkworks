#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value, GRID_SIZE};

    #[test]
    fn test_genesis_rune() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Genesis Circuit:
        // "push" -> G
        //           ^
        //           |
        //          123

        // West: OpCode String "push"
        vm.grid[5][4] = Value::Str("push".to_string());
        // North: Arg Value 123
        vm.grid[4][5] = Value::Int(123);
        // Center: Genesis Rune 'G'
        vm.grid[5][5] = Value::Str("G".to_string());
        // South: Sink '?' to capture result
        vm.grid[6][5] = Value::Str("?".to_string());

        // Connect inputs with Source '!'
        vm.grid[5][3] = Value::Str("!".to_string()); // Source for "push" (at 5,3 pushing to 5,4)
                                                     // Wait, '!' source emits West value to Self.
                                                     // So:
                                                     // 5,2: "push"
                                                     // 5,3: "!" (emits "push" to 5,3 signal grid)
                                                     // 5,4: "~" (wire carries to 5,5)
                                                     // 5,5: "G"

        // Pre-load signals into delayed_signals so they persist past prepare_signals
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("push".to_string()));
        vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(123));

        // Ensure G is on the grid so it gets scanned
        vm.grid[5][5] = Value::Str("G".to_string());

        exec_prologue_tick(&mut vm);

        // Check output signal at South (6,5)
        // G writes to next_signals South.
        // In propagation, it might not move further unless there's a wire.
        // But we check signal_grid directly.
        // Wait, exec_prologue_tick runs multiple propagation iterations.
        // G should write to South in one of them.

        let south_sig = &vm.prologue_state.signal_grid[6][5];
        assert!(south_sig.is_some(), "Genesis failed to emit signal");

        if let Some(val) = south_sig {
            match val {
                Value::Junction(JunctionType::All, list) => {
                    assert_eq!(list.len(), 2, "Expected [Op, Args]");
                    assert_eq!(list[0], Value::Str("push".to_string()));
                    // Args should be wrapped in Junction(All)
                    match &list[1] {
                        Value::Junction(JunctionType::All, args) => {
                            assert_eq!(args.len(), 1);
                            assert_eq!(args[0], Value::Int(123));
                        }
                        _ => panic!("Expected Args Junction, got {:?}", list[1]),
                    }
                }
                _ => panic!("Expected Gene Tuple Junction, got {:?}", val),
            }
        }
    }

    #[test]
    fn test_ligation_rune() {
        // Setup Ligation Circuit:
        // GeneTuple -> Z -> Success
        //              ^
        //              |
        //          StrandIdx(0)

        let strand = Strand { genes: vec![] };
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Gene Tuple: ["add", []]
        let gene_tuple = Value::Junction(
            JunctionType::All,
            vec![
                Value::Str("add".to_string()),
                Value::Junction(JunctionType::All, vec![]),
            ],
        );

        // Pre-load signals into delayed_signals
        vm.prologue_state.delayed_signals[5][4] = Some(gene_tuple); // West
        vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(0)); // North (Strand 0)

        // Rune 'Z' at 5,5
        vm.grid[5][5] = Value::Str("Z".to_string());
        // Z is a sink, so it will be processed after propagation.

        exec_prologue_tick(&mut vm);

        // Check DNA
        let strands = &vm.dna.helix.strands;
        assert_eq!(strands.len(), 1);
        assert_eq!(strands[0].genes.len(), 1, "Gene not appended");
        assert_eq!(strands[0].genes[0].op, OpCode::Add);

        // Check Success Signal (East 5,6)
        // Z writes 1 to self signal grid to show activity?
        // Or writes 1 to East?
        // Let's implement writing to East.
        // exec_prologue_tick calls apply_sink_rune multiple times (epigenetics).
        // It might not propagate signal East unless we explicitly set it.
        // Let's check if 5,5 lit up.
        assert!(vm.prologue_state.signal_grid[5][5].is_some());
    }
}
