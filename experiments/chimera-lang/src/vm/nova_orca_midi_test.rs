#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, MidiEvent, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_orca_midi_note() {
        let mut vm = make_vm();
        // Layout:
        // . N .
        // C : V
        // . D .
        // N=0 (C3), V=35 (Max), C=0 (Ch1), D=4 (Quarter)

        vm.grid[0][1] = Value::Str("0".to_string());
        vm.grid[1][0] = Value::Str("0".to_string());
        vm.grid[1][1] = Value::Str(":".to_string());
        vm.grid[1][2] = Value::Str("z".to_string()); // 'z' is 35 in base 36
        vm.grid[2][1] = Value::Str("4".to_string());

        // Bang!
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Signal processing buffers into SignalContext, then writes to VM at end of update.
        // We modified process_signals to write to midi_messages at step 3.75.
        // So checking vm.midi_messages should work.

        assert_eq!(vm.midi_messages.len(), 1);
        match &vm.midi_messages[0] {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
                duration,
            } => {
                assert_eq!(*channel, 0);
                assert_eq!(*note, 48); // 0 + 48 = C3
                assert_eq!(*velocity, 127); // 'z' (35) -> 1.0 -> 127
                assert_eq!(*duration, 4);
            }
            _ => panic!("Expected NoteOn"),
        }
    }

    #[test]
    fn test_orca_midi_cc() {
        let mut vm = make_vm();
        // Layout:
        // . K .
        // C ; V
        // . . .
        // K=1 (Mod Wheel), V=10, C=1 (Ch2)

        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[1][0] = Value::Str("1".to_string());
        vm.grid[1][1] = Value::Str(";".to_string());
        vm.grid[1][2] = Value::Str("a".to_string()); // 'a' is 10

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.midi_messages.len(), 1);
        match &vm.midi_messages[0] {
            MidiEvent::ControlChange {
                channel,
                controller,
                value,
            } => {
                assert_eq!(*channel, 1);
                assert_eq!(*controller, 1);
                // Value: 10/35 * 127 approx 36
                assert_eq!(*value, 36);
            }
            _ => panic!("Expected ControlChange"),
        }
    }

    #[test]
    fn test_orca_random() {
        let mut vm = make_vm();
        // Layout:
        // . 0 .  (Min)
        // * ? 9  (Max)
        // . . .
        // Result written to South (2,1)

        vm.grid[0][1] = Value::Str("0".to_string());
        vm.grid[1][0] = Value::Str("*".to_string());
        vm.grid[1][1] = Value::Str("?".to_string());
        vm.grid[1][2] = Value::Str("9".to_string());

        vm.signal_grid[1][1] = 1; // Signal directly on ?

        process_signals(&mut vm);

        // Check if something was written to South
        match &vm.grid[2][1] {
            Value::Str(s) => {
                let v = s.chars().next().unwrap();
                assert!(v >= '0' && v <= '9');
            }
            _ => panic!("Expected string output at (2,1)"),
        }
    }
}
