#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_quantum_superposition() {
        let mut vm = setup_vm();
        // Setup: 10 -> ! -> q
        vm.grid[4][4] = Value::Int(10);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("q".to_string());

        exec_prologue_tick(&mut vm);

        // Check if q outputted superposition
        let output = &vm.prologue_state.signal_grid[5][5];
        match output {
            Some(Value::Superposition(states)) => {
                assert_eq!(states.len(), 2);
                // Check if it contains 10 and 11
                let has_10 = states.iter().any(|(v, _)| *v == Value::Int(10));
                let has_11 = states.iter().any(|(v, _)| *v == Value::Int(11));
                assert!(has_10 && has_11);
            }
            _ => panic!("Expected Superposition, got {:?}", output),
        }
    }

    #[test]
    fn test_quantum_measurement() {
        let mut vm = setup_vm();
        // Setup: Superposition -> ! -> m
        let sup = Value::Superposition(vec![(Value::Int(1), 1.0)]); // 100% prob of 1
        vm.grid[4][4] = sup;
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("m".to_string());

        exec_prologue_tick(&mut vm);

        // Check if m outputted collapsed value
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Int(1)));
    }

    #[test]
    fn test_teleportation() {
        let mut vm = setup_vm();
        // Setup Sender: 42 -> ! -> { <- 1 (Channel)
        // 4,4: 42
        // 5,4: !
        // 5,5: {
        // 4,5: 1 (Channel)
        // 5,5 needs North signal. So 4,5 needs to be signal source.
        // 3,5: 1
        // 4,5: !

        vm.grid[4][4] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());

        vm.grid[3][5] = Value::Int(1);
        vm.grid[4][5] = Value::Str("!".to_string());

        vm.grid[5][5] = Value::Str("{".to_string());

        // Setup Receiver: 1 (Channel) -> }
        // 7,8: 1
        // 8,8: !
        // 9,8: }

        vm.grid[7][8] = Value::Int(1);
        vm.grid[8][8] = Value::Str("!".to_string());
        vm.grid[9][8] = Value::Str("}".to_string());

        // Tick 1: Teleport
        exec_prologue_tick(&mut vm);

        // Check storage
        assert!(vm.prologue_state.teleport_channels.contains_key(&1));
        assert_eq!(vm.prologue_state.teleport_channels.get(&1), Some(&Value::Int(42)));

        // Tick 2: Receive
        // Note: Receiver logic executes in same tick if order permits, but here we check across ticks to be safe.
        // Actually, if we just ran tick 1, signals propagated.
        // Did } receive in Tick 1?
        // } is at 9,8. It needs North signal from 8,8.
        // 8,8 is !. It reads 7,8 (Int 1).
        // ! emits to signal grid at start of tick.
        // So } should read it in Tick 1.
        // And if teleport_channels was updated in same tick (by { at 5,5), } might read it if processed after?
        // Current logic iterates propagation.
        // But `teleport_channels` is updated instantly in `apply_teleport_runes`.
        // So yes, it should work in one tick if iteration order hits { then }.
        // Or multiple iterations of propagation.
        // Let's check result of Tick 1.

        assert_eq!(vm.prologue_state.signal_grid[9][8], Some(Value::Int(42)));
    }
}
