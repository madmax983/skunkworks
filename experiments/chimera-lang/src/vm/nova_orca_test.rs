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

    #[test]
    fn test_orca_io() {
        let mut vm = make_vm();
        // Layout:
        // . 5 .
        // * N .
        // . . .
        // N should read North (5) and write South (5)

        vm.grid[0][1] = Value::Str("5".to_string());
        vm.grid[1][0] = Value::Str("*".to_string());
        vm.grid[1][1] = Value::Str("N".to_string());
        vm.signal_grid[1][0] = 1;

        process_signals(&mut vm);

        // N should stay put
        match &vm.grid[1][1] {
            Value::Str(s) => assert_eq!(s, "N"),
            _ => panic!("N moved or disappeared"),
        }

        // South (2,1) should be 5
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "5"),
            _ => panic!("Expected result 5 at South, got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_teleport() {
        let mut vm = make_vm();
        // Layout:
        // . V .  (Value to write: V='7')
        // X T Y  (X=3, Y=3) -> Write '7' to (3,3)
        // . . .

        vm.grid[0][1] = Value::Str("7".to_string());
        vm.grid[1][0] = Value::Str("3".to_string()); // West (X)
        vm.grid[1][1] = Value::Str("T".to_string());
        vm.grid[1][2] = Value::Str("3".to_string()); // East (Y)

        // Signal T directly
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        match &vm.grid[3][3] {
            Value::Str(s) => assert_eq!(s, "7"),
            _ => panic!("Expected result 7 at (3,3), got {:?}", vm.grid[3][3]),
        }
    }

    #[test]
    fn test_orca_laser() {
        let mut vm = make_vm();
        // Layout:
        // . 1 .  (Direction: 1 = East)
        // 4 L .  (Length: 4)
        // . . .
        // Should fire beam East for 4 cells.

        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[1][0] = Value::Str("4".to_string());
        vm.grid[1][1] = Value::Str("L".to_string());

        // Signal L
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check signal trace to the East
        // (1,2), (1,3), (1,4), (1,5) should have signal
        assert!(vm.signal_grid[1][2] > 0, "Beam missing at dist 1");
        assert!(vm.signal_grid[1][3] > 0, "Beam missing at dist 2");
        assert!(vm.signal_grid[1][4] > 0, "Beam missing at dist 3");
        assert!(vm.signal_grid[1][5] > 0, "Beam missing at dist 4");
        assert_eq!(vm.signal_grid[1][6], 0, "Beam went too far");
    }

    #[test]
    fn test_orca_query() {
        let mut vm = make_vm();
        // Layout:
        // . 3 .  (Direction: 3 = West)
        // B Q B  (West of Q is 'B'. East of Q is Target 'B')
        // . . .
        // Q at (1,1).
        // North Input (0,1) is '3'.
        // East Input (1,2) is 'B'.
        // West Neighbor (1,0) is 'B'.
        // Expected: Match -> Write '1' to South (2,1).

        vm.grid[0][1] = Value::Str("3".to_string());
        vm.grid[1][0] = Value::Str("B".to_string());
        vm.grid[1][1] = Value::Str("Q".to_string());
        vm.grid[1][2] = Value::Str("B".to_string());

        // Signal Q
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected result 1, got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_modulo() {
        let mut vm = make_vm();
        vm.grid[0][1] = Value::Str("5".to_string());
        vm.grid[1][2] = Value::Str("2".to_string());
        vm.grid[1][1] = Value::Str("%".to_string());
        vm.signal_grid[1][1] = 1;
        process_signals(&mut vm);
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected result 1, got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_logic() {
        let mut vm = make_vm();
        vm.grid[0][1] = Value::Str("5".to_string());
        vm.grid[1][2] = Value::Str("3".to_string());
        vm.grid[1][1] = Value::Str("&".to_string());
        vm.signal_grid[1][1] = 1;
        process_signals(&mut vm);
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected result 1, got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_ether() {
        let mut vm = make_vm();
        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[1][2] = Value::Str("A".to_string());
        vm.grid[1][1] = Value::Str("[".to_string());
        vm.signal_grid[1][1] = 1;
        process_signals(&mut vm);

        let queue = vm.ether.get(&1).expect("Expected queue for channel 1");
        assert_eq!(queue.len(), 1);
        match &queue[0] {
            Value::Str(s) => assert_eq!(s, "a"),
            _ => panic!("Expected 'a' in ether"),
        }

        vm.grid[1][1] = Value::Str("]".to_string());
        vm.grid[2][1] = Value::Str(".".to_string());
        vm.signal_grid[1][1] = 1;
        process_signals(&mut vm);

        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "a"),
            _ => panic!("Expected result 'a', got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_unzip() {
        let mut vm = make_vm();
        // Setup DNA: Strand 0 has [push(10)]
        // Gene 0: push(10) -> "push"
        use crate::ast::{Gene, Nucleotide, Strand};
        use crate::opcode::OpCode;
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };
        vm.dna.helix.strands.push(strand);

        // Layout:
        // W U E
        // W=Strand=0
        // E=Gene=0
        // U at (1,1)

        vm.grid[1][0] = Value::Str("0".to_string()); // Strand 0
        vm.grid[1][1] = Value::Str("U".to_string());
        vm.grid[1][2] = Value::Str("0".to_string()); // Gene 0

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check output
        // (2,1) = 'p'
        // (3,1) = 'u'
        // (4,1) = 's'
        // (5,1) = 'h'

        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "p"),
            _ => panic!("Expected 'p' at (2,1), got {:?}", vm.grid[2][1]),
        }
        match &vm.grid[3][1] {
            Value::Str(s) => assert_eq!(s, "u"),
            _ => panic!("Expected 'u' at (3,1), got {:?}", vm.grid[3][1]),
        }
        match &vm.grid[4][1] {
            Value::Str(s) => assert_eq!(s, "s"),
            _ => panic!("Expected 's' at (4,1), got {:?}", vm.grid[4][1]),
        }
        match &vm.grid[5][1] {
            Value::Str(s) => assert_eq!(s, "h"),
            _ => panic!("Expected 'h' at (5,1), got {:?}", vm.grid[5][1]),
        }
    }
}
