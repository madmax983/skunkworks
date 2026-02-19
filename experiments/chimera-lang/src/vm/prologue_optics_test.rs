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
    fn test_rune_mirror_back() {
        let mut vm = setup_vm();
        // Setup: 42 -> ! -> \
        // 5,3: 42 (Value)
        // 5,4: ! (Source, reads West 5,3)
        // 5,5: \ (Mirror, reads West 5,4, should reflect South)

        // \ reflects:
        // Input West -> Output South
        // Input North -> Output East
        // Input East -> Output North
        // Input South -> Output West

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("\\".to_string());

        exec_prologue_tick(&mut vm);

        // Mirror Output Logic:
        // If 5,5 receives signal from West (5,4), it outputs to South (6,5).
        // Check 6,5
        assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(42)));
        // Ensure no leakage to East (5,6)
        assert!(vm.prologue_state.signal_grid[5][6].is_none());
    }

    #[test]
    fn test_rune_mirror_forward() {
        let mut vm = setup_vm();
        // Setup: 42 -> ! -> /
        // 5,3: 42
        // 5,4: !
        // 5,5: / (Mirror, reads West, reflects North)

        // / reflects:
        // Input West -> Output North
        // Input North -> Output West
        // Input East -> Output South
        // Input South -> Output East

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("/".to_string());

        exec_prologue_tick(&mut vm);

        // Input West -> Output North (4,5)
        assert_eq!(vm.prologue_state.signal_grid[4][5], Some(Value::Int(42)));
        // Ensure no leakage to South (6,5)
        assert!(vm.prologue_state.signal_grid[6][5].is_none());
    }

    #[test]
    fn test_rune_beam_horizontal() {
        let mut vm = setup_vm();
        // Setup: 42 -> ! -> -
        // 5,3: 42
        // 5,4: !
        // 5,5: - (Beam, reads West, passes East)

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("-".to_string());

        exec_prologue_tick(&mut vm);

        // Input West -> Output East (5,6)
        assert_eq!(vm.prologue_state.signal_grid[5][6], Some(Value::Int(42)));

        // Verify it BLOCKS North input
        // Setup: 99 -> ! -> (North of -)
        // 4,4: 99
        // 4,5: ! (Reads 4,4 -> Emits to 4,5)
        // 5,5: - (Already there)
        // 6,5: ? (South of -)

        // Reset grid for part 2
        let mut vm2 = setup_vm();
        vm2.grid[4][4] = Value::Int(99);
        vm2.grid[4][5] = Value::Str("!".to_string());
        vm2.grid[5][5] = Value::Str("-".to_string());

        exec_prologue_tick(&mut vm2);

        // Signal from North (4,5) should NOT pass to South (6,5) or East/West
        assert!(vm2.prologue_state.signal_grid[6][5].is_none());
        assert!(vm2.prologue_state.signal_grid[5][6].is_none());
    }
}
