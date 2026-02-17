#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, MidiEvent, Value};

    fn make_empty_vm() -> ChimeraVM {
        // Create a dummy strand so VM doesn't halt
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Drop,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            }, // Infinite loop
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.phase = crate::vm::nova::Phase::Corporeal;
        vm
    }

    #[test]
    fn test_bang_propagation() {
        let mut vm = make_empty_vm();

        // Setup Grid: * at (5,5)
        vm.grid[5][5] = Value::Str("*".to_string());

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        vm.step();

        // Neighbors should have signal
        assert!(vm.signal_grid[5][4] > 0, "West neighbor failed");
        assert!(vm.signal_grid[5][6] > 0, "East neighbor failed");
        assert!(vm.signal_grid[4][5] > 0, "North neighbor failed");
        assert!(vm.signal_grid[6][5] > 0, "South neighbor failed");

        // Original should be 0 (unless back-propagated by something, which shouldn't happen here)
        // Wait, process_signals creates next_signals from scratch.
        // So original signal dissipates if not re-triggered.
        assert_eq!(vm.signal_grid[5][5], 0, "Center signal should dissipate");
    }

    #[test]
    fn test_directional_propagation() {
        let mut vm = make_empty_vm();

        // Setup Grid: > at (5,5)
        vm.grid[5][5] = Value::Str(">".to_string());

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        vm.step();

        // East should have signal
        assert!(vm.signal_grid[5][6] > 0, "East neighbor failed");

        // West should NOT
        assert_eq!(vm.signal_grid[5][4], 0, "West neighbor shouldn't fire");
    }

    #[test]
    fn test_triggered_execution() {
        let mut vm = make_empty_vm();

        // Setup: * at (5,5), "add" at (5,6)
        vm.grid[5][5] = Value::Str("*".to_string());
        vm.grid[5][6] = Value::Str("add".to_string());

        // Stack: 10, 20
        vm.stack.push(Value::Int(10));
        vm.stack.push(Value::Int(20));

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        // Bang propagates to (5,6) next signal grid.
        // DNA executes Push(0).
        vm.step();

        // Stack should be 10, 20, 0 (dummy push)
        assert_eq!(vm.stack.len(), 3);

        // Step AGAIN
        // Signal trigger Add. Add(20, 0) -> 20. Stack: [10, 20].
        // DNA execute Drop. Stack: [10].
        vm.step();

        // Stack should be 10
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(10));
    }

    #[test]
    fn test_arithmetic_add() {
        let mut vm = make_empty_vm();
        // A: Add. North (10) + East (20) -> South (30 -> 'u')
        vm.grid[5][5] = Value::Str("A".to_string());
        vm.grid[4][5] = Value::Int(10);
        vm.grid[5][6] = Value::Int(20);
        vm.signal_grid[5][5] = 1;
        vm.step();

        // Result at South (6,5)
        // 30 in Base36 is 'u'
        match &vm.grid[6][5] {
            Value::Str(s) => assert_eq!(s, "u"),
            _ => panic!("Expected string result 'u', got {:?}", vm.grid[6][5]),
        }
    }

    #[test]
    fn test_logic_and() {
        let mut vm = make_empty_vm();
        // &: And. North (0b1100 = 12) & East (0b1010 = 10) -> South (0b1000 = 8)
        vm.grid[5][5] = Value::Str("&".to_string());
        vm.grid[4][5] = Value::Int(12);
        vm.grid[5][6] = Value::Int(10);
        vm.signal_grid[5][5] = 1;
        vm.step();

        match &vm.grid[6][5] {
            Value::Str(s) => assert_eq!(s, "8"),
            _ => panic!("Expected string result '8', got {:?}", vm.grid[6][5]),
        }
    }

    #[test]
    fn test_execution_increment() {
        let mut vm = make_empty_vm();
        // I: Increment. North (5) -> South (6)
        vm.grid[5][5] = Value::Str("I".to_string());
        vm.grid[4][5] = Value::Int(5);
        vm.grid[5][6] = Value::Int(35); // Max 35
        vm.signal_grid[5][5] = 1;
        vm.step();

        match &vm.grid[6][5] {
            Value::Str(s) => assert_eq!(s, "6"),
            _ => panic!("Expected string result '6', got {:?}", vm.grid[6][5]),
        }
    }

    #[test]
    fn test_directional_north() {
        let mut vm = make_empty_vm();
        // N: Read North, Write South
        vm.grid[5][5] = Value::Str("N".to_string());
        vm.grid[4][5] = Value::Str("X".to_string()); // Value at North
        vm.signal_grid[5][5] = 1;
        vm.step();

        // Note: Nova signals normalize chars to lowercase/base36 when writing.
        match &vm.grid[6][5] {
            Value::Str(s) => assert_eq!(s, "x"),
            _ => panic!("Expected string result 'x', got {:?}", vm.grid[6][5]),
        }
    }

    #[test]
    fn test_midi_note() {
        let mut vm = make_empty_vm();
        // : : Midi Note
        // N: Note 0 (Base C3)
        // E: Velocity 35 (Max)
        // W: Channel 0
        // S: Duration 1
        vm.grid[5][5] = Value::Str(":".to_string());
        vm.grid[4][5] = Value::Int(0); // Note 0 (+48 = 48)
        vm.grid[5][6] = Value::Int(35); // Velocity Max
        vm.grid[5][4] = Value::Int(0); // Channel 0
        vm.grid[6][5] = Value::Int(1); // Duration 1
        vm.signal_grid[5][5] = 1;
        vm.step();

        // Check MIDI events
        assert_eq!(vm.midi_messages.len(), 1);
        match &vm.midi_messages[0] {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
                duration,
            } => {
                assert_eq!(*channel, 0);
                assert_eq!(*note, 48);
                assert_eq!(*velocity, 127);
                assert_eq!(*duration, 1);
            }
            _ => panic!("Expected NoteOn event"),
        }
    }

    #[test]
    fn test_exotic_sum() {
        let mut vm = make_empty_vm();
        // Σ: Sum N+E+W -> S
        // N=1, E=2, W=3 -> Sum=6
        vm.grid[5][5] = Value::Str("Σ".to_string());
        vm.grid[4][5] = Value::Int(1);
        vm.grid[5][6] = Value::Int(2);
        vm.grid[5][4] = Value::Int(3);
        vm.signal_grid[5][5] = 1;
        vm.step();

        match &vm.grid[6][5] {
            Value::Str(s) => assert_eq!(s, "6"),
            _ => panic!("Expected string result '6', got {:?}", vm.grid[6][5]),
        }
    }
}
