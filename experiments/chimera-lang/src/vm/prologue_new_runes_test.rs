#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_clock_rune() {
        let mut vm = make_vm();
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.tick_counter = 13;

        exec_prologue_tick(&mut vm);

        // Check if signal emitted
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
            assert_eq!(*v, 3); // 13 % 10 = 3
        } else {
            panic!("Clock rune did not emit signal");
        }
    }

    #[test]
    fn test_directional_runes() {
        let mut vm = make_vm();
        // Setup: Signal -> N
        // N acts as a diode allowing signal from South to Self (and thus North).
        // (6,4) has Signal 42.
        // (5,4) is N.

        // Note: We must use delayed_signals because exec_prologue_tick clears signal_grid at start
        vm.prologue_state.delayed_signals[6][4] = Some(Value::Int(42));
        vm.grid[5][4] = Value::Str("N".to_string());

        exec_prologue_tick(&mut vm);

        // N should pick up signal from South (6,4) and output to Self (5,4).
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][4] {
            assert_eq!(*v, 42);
        } else {
            panic!("N rune did not propagation signal from South");
        }
    }

    #[test]
    fn test_warp_rune() {
        let mut vm = make_vm();
        // Setup:
        // (4,5) = 100 (North val)
        // (5,4) = 1 (Signal West)
        // (5,5) = ( (Warp)
        // (6,5) = 200 (South val)

        vm.grid[4][5] = Value::Int(100);
        vm.grid[6][5] = Value::Int(200);
        vm.grid[5][5] = Value::Str("(".to_string());

        // Provide signal from West via delay
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(1));

        exec_prologue_tick(&mut vm);

        // Check swap
        assert_eq!(vm.grid[4][5], Value::Int(200));
        assert_eq!(vm.grid[6][5], Value::Int(100));
    }

    #[test]
    fn test_crossover_rune() {
        let mut vm = make_vm();

        // Create 2 strands
        let strand_a = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
            ],
        };
        let strand_b = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(3)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(4)],
                },
            ],
        };
        vm.dna.helix.strands.push(strand_a); // idx 0
        vm.dna.helix.strands.push(strand_b); // idx 1

        // Setup Crossover
        // West Signal: 0 (Strand A)
        // East Signal: 1 (Strand B)
        // Rune X at (5,5)

        vm.grid[5][5] = Value::Str("X".to_string());
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(0));
        vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(1));

        exec_prologue_tick(&mut vm);

        // Should create new strand at idx 2
        assert_eq!(vm.dna.helix.strands.len(), 3);
        let new_strand = &vm.dna.helix.strands[2];
        // Midpoint of 2 is 1.
        // A[..1] = [Push(1)]
        // B[1..] = [Push(4)]
        // Result: [Push(1), Push(4)]

        assert_eq!(new_strand.genes.len(), 2);
        assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(1));
        assert_eq!(new_strand.genes[1].args[0], Nucleotide::Number(4));

        // Check output signal at Self (5,5)
        if let Some(Value::Int(idx)) = &vm.prologue_state.signal_grid[5][5] {
            assert_eq!(*idx, 2);
        }

        // Check grid write at South (6,5)
        assert_eq!(vm.grid[6][5], Value::Int(2));
    }
}
