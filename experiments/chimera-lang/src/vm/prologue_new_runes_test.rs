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
    fn test_rune_jumper() {
        let mut vm = setup_vm();
        // Setup: 42 -> ! -> J -> ?
        // 5,3: 42 (Value)
        // 5,4: ! (Source, reads West 5,3, becomes signal at 5,4)
        // 5,5: J (Jumper, reads West 5,4, writes East 5,6)

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("J".to_string());

        exec_prologue_tick(&mut vm);

        // J should output to East (5,6)
        assert!(vm.prologue_state.signal_grid[5][6].is_some());
        assert_eq!(vm.prologue_state.signal_grid[5][6], Some(Value::Int(42)));
    }

    #[test]
    fn test_rune_warp() {
        let mut vm = setup_vm();
        // Setup: 1 -> ! -> (
        // Operands: A (North of (), 4,5), B (South of (), 6,5)
        // 5,3: 1 (Trigger Signal)
        // 5,4: !
        // 5,5: (
        // 4,5: 10
        // 6,5: 20

        vm.grid[5][3] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());

        vm.grid[4][5] = Value::Int(10);
        vm.grid[6][5] = Value::Int(20);
        vm.grid[5][5] = Value::Str("(".to_string());

        exec_prologue_tick(&mut vm);

        // Check Grid Swap
        assert_eq!(vm.grid[4][5], Value::Int(20));
        assert_eq!(vm.grid[6][5], Value::Int(10));
    }

    #[test]
    fn test_rune_clock() {
        let mut vm = setup_vm();
        // Setup: 3 -> ! -> C
        // 5,3: 3 (Modulus)
        // 5,4: !
        // 5,5: C

        vm.grid[5][3] = Value::Int(3);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("C".to_string());

        // Fix: C is also a Critter. Set it to static state so it doesn't move away.
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str("C:100:.:0".to_string()));

        vm.tick_counter = 10;
        exec_prologue_tick(&mut vm);
        // 10 % 3 = 1. Output at Self (5,5)
        assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(1)));

        vm.tick_counter = 11;
        exec_prologue_tick(&mut vm);
        // 11 % 3 = 2
        assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(2)));
    }

    #[test]
    fn test_rune_directional() {
        let mut vm = setup_vm();
        // Setup: 99 -> ! -> E
        // 5,3: 99
        // 5,4: !
        // 5,5: E (East Emitter)

        vm.grid[5][3] = Value::Int(99);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("E".to_string());

        exec_prologue_tick(&mut vm);

        // E reads West (5,4) and writes East (5,6)
        assert_eq!(vm.prologue_state.signal_grid[5][6], Some(Value::Int(99)));
    }
}
