#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
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
        vm.grid[5][3] = Value::Int(10);
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
        vm.grid[5][3] = sup;
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
        // 5,3: 42
        // 5,4: ! (Value Src)
        // 5,5: {
        // 4,4: 1
        // 4,5: ! (Channel Src, emits to 4,5 which is North of {)

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());

        vm.grid[4][4] = Value::Int(1);
        vm.grid[4][5] = Value::Str("!".to_string());

        vm.grid[5][5] = Value::Str("{".to_string());

        // Setup Receiver: 1 (Channel) -> }
        // 8,7: 1
        // 8,8: ! (Channel Src, emits to 8,8 which is North of })
        // 9,8: }

        vm.grid[8][7] = Value::Int(1);
        vm.grid[8][8] = Value::Str("!".to_string());
        vm.grid[9][8] = Value::Str("}".to_string());

        // Tick 1: Teleport
        exec_prologue_tick(&mut vm);

        // Check storage
        assert!(vm.prologue_state.teleport_channels.contains_key(&1));
        assert_eq!(
            vm.prologue_state.teleport_channels.get(&1),
            Some(&Value::Int(42))
        );

        // Tick 2: Receive (should happen in same tick if order permits)
        assert_eq!(vm.prologue_state.signal_grid[9][8], Some(Value::Int(42)));
    }
}
