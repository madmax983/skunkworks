#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_jumper() {
        let mut vm = make_vm();
        // Layout:
        // . . .
        // 7 J .
        // . . .
        // Jumper (J) at (1,1). Input West (7). Output East.

        vm.grid[1][0] = Value::Str("7".to_string());
        vm.grid[1][1] = Value::Str("J".to_string());

        // Signal J to activate? Or is J passive/always active?
        // Orca J usually requires a bang? Or acts if it has input?
        // In my plan, I decided to check for signal or just inputs.
        // Most Orca operators need a Bang or adjacent operator triggering them.
        // But some are passive (like movements).
        // Let's assume Jumper requires a signal for now, or we implement it to always run if input exists?
        // Standard Orca: Operators run if they receive a bang OR are "Primary" (uppercase letters usually run if banged).
        // BUT movement operators often run every tick if they have inputs? No.
        // Let's bang it to be sure.
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check East (1,2)
        match &vm.grid[1][2] {
            Value::Str(s) => assert_eq!(s, "7", "Jumper failed to move '7' to East"),
            _ => panic!("Expected '7' at (1,2), got {:?}", vm.grid[1][2]),
        }
    }

    #[test]
    fn test_warp() {
        let mut vm = make_vm();
        // Layout:
        // . A .
        // . ( .
        // . B .
        // Warp '(' at (1,1). North 'A', South 'B'.
        // Expect: North 'B', South 'A'.

        vm.grid[0][1] = Value::Str("A".to_string());
        vm.grid[1][1] = Value::Str("(".to_string());
        vm.grid[2][1] = Value::Str("B".to_string());

        // Bang it
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check North (0,1)
        match &vm.grid[0][1] {
            Value::Str(s) => assert_eq!(s, "B", "Warp failed to move 'B' to North"),
            _ => panic!("Expected 'B' at (0,1), got {:?}", vm.grid[0][1]),
        }

        // Check South (2,1)
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "A", "Warp failed to move 'A' to South"),
            _ => panic!("Expected 'A' at (2,1), got {:?}", vm.grid[2][1]),
        }
    }
}
