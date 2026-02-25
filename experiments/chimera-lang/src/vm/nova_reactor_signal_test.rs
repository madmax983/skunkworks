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
        let mut vm = ChimeraVM::new(dna);
        vm.orca_mode = true;
        vm
    }

    #[test]
    fn test_reactor_stable_fusion() {
        let mut vm = make_empty_vm();

        // Setup:
        // . 1 ⚛ 2 .
        // Positions: (5,4)=1, (5,5)=⚛, (5,6)=2
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][5] = Value::Str("⚛".to_string());
        vm.grid[5][6] = Value::Int(2);

        // Signal the reactor
        vm.signal_grid[5][5] = 1;

        process_signals(&mut vm);

        // Expect:
        // South (6,5) = 1+2 = 3 (val_to_char(3) = '3')
        // North (4,5) = 3%10 = 3 (val_to_char(3) = '3')

        assert_eq!(vm.grid[6][5], Value::Str("3".to_string()));
        assert_eq!(vm.grid[4][5], Value::Str("3".to_string()));

        // Reactor should remain
        assert_eq!(vm.grid[5][5], Value::Str("⚛".to_string()));
    }

    #[test]
    fn test_reactor_meltdown() {
        let mut vm = make_empty_vm();

        // Setup:
        // . 50 ⚛ 50 .
        // Sum 100 > 99
        vm.grid[5][4] = Value::Int(50);
        vm.grid[5][5] = Value::Str("⚛".to_string());
        vm.grid[5][6] = Value::Int(50);

        // Signal the reactor
        vm.signal_grid[5][5] = 1;

        process_signals(&mut vm);

        // Expect:
        // Reactor becomes Slag (#)
        assert_eq!(vm.grid[5][5], Value::Str("#".to_string()));

        // Expect Bang signals in neighbors?
        // exec_reactor_rune sets ctx.next_signals
        // process_signals applies ctx.next_signals to vm.signal_grid at end
        // So checking vm.signal_grid[5][4] (West) should be > 0 (it was 0 before, assuming 50 doesn't emit)

        // Note: 50 is 'Y' in base 36 (34) wait.
        // val_to_char(50): 50 % 36 = 14 -> 'e' (10='a', 11='b', 12='c', 13='d', 14='e')
        // So 50 is just a value.

        // Check signal propagation (Bang adds 10)
        assert!(vm.signal_grid[5][4] >= 10); // West
        assert!(vm.signal_grid[5][6] >= 10); // East
        assert!(vm.signal_grid[4][5] >= 10); // North
        assert!(vm.signal_grid[6][5] >= 10); // South
    }

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.orca_mode = true;
        vm
    }
}
