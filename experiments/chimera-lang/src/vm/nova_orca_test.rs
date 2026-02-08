#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_orca_add() {
        let mut vm = make_vm();
        // Layout:
        // . 1 .
        // * A 2
        // . . .

        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[1][0] = Value::Str("*".to_string());
        vm.grid[1][1] = Value::Str("A".to_string());
        vm.grid[1][2] = Value::Str("2".to_string());

        // Initial signal at (1,0) - The Bang
        vm.signal_grid[1][0] = 1;

        // Step 1: Bang propagates to A
        process_signals(&mut vm);

        // Check propagation: A at (1,1) should have signal in next step?
        // process_signals handles propagation AND execution in one go?
        // Based on current implementation, it does propagation then execution.
        // Bang at (1,0) propagates to (1,1) (A).
        // Then execution phase runs A.
        // A reads North(1) and East(2), writes South(3).

        // Let's verify result at (2,1)
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "3"),
            Value::Int(n) => assert_eq!(*n, 3), // Depending on implementation
            _ => panic!("Expected result 3, got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_base36() {
        let mut vm = make_vm();
        // Layout:
        // . a .  (10)
        // * A 1  (1)
        // . . .
        // Output: 11 -> 'b'

        vm.grid[0][1] = Value::Str("a".to_string());
        vm.grid[1][0] = Value::Str("*".to_string());
        vm.grid[1][1] = Value::Str("A".to_string());
        vm.grid[1][2] = Value::Str("1".to_string());

        vm.signal_grid[1][0] = 1;

        process_signals(&mut vm);

        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "b"),
            Value::Int(n) => assert_eq!(*n, 11),
            _ => panic!("Expected result 'b' (11), got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_clock() {
        let mut vm = make_vm();
        // Layout:
        // . 4 . (Modulo)
        // * C .
        // . . .

        vm.grid[0][1] = Value::Str("4".to_string());
        vm.grid[1][0] = Value::Str("*".to_string());
        vm.grid[1][1] = Value::Str("C".to_string());

        vm.signal_grid[1][0] = 1;
        vm.tick_counter = 5; // 5 % 4 = 1

        process_signals(&mut vm);

        // Result at South (2,1) should be 1
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected result 1, got {:?}", vm.grid[2][1]),
        }
    }
}
