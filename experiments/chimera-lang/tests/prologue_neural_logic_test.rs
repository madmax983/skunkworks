#[cfg(feature = "biophysics")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_neural_synapse() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup: 100 -> ! -> • -> $ -> (Output at 7,7)
        // Coords:
        // 7,4: 100
        // 7,5: !
        // 7,6: •
        // 7,7: $
        // 7,8: (Target) - Wait, $ writes South.
        // Let's use:
        // 100 ! • $
        //         .

        vm.grid[7][4] = Value::Int(100);
        vm.grid[7][5] = Value::Str("!".to_string());
        vm.grid[7][6] = Value::Str("•".to_string());
        vm.grid[7][7] = Value::Str("$".to_string());

        // Init Synapse weight to 50%
        vm.prologue_state.registers.insert((7, 6), Value::Int(50));

        // Tick 1: ! emits 100 to 7,5
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Tick 2: 100 moves to 7,6 (Synapse) -> becomes 50 -> moves to 7,7 ($)
        // Wait, propagation happens in loop within one tick.
        // So 100 -> ! -> •(50) -> $ should happen in one tick if iteration suffices.

        // Let's check intermediate state if needed, or result.
        // $ writes to South (8,7).

        // Run tick
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Check grid[8][7]
        if let Value::Int(v) = vm.grid[8][7] {
            assert_eq!(v, 50, "Synapse should attenuate signal to 50%");
        } else {
            panic!("Expected Int(50), got {:?}", vm.grid[8][7]);
        }
    }

    #[test]
    fn test_neural_learning() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup:
        // 100 ! ° •
        // ° is at 7,6. • is at 7,7.
        // 100 -> ! -> °

        vm.grid[7][4] = Value::Int(100);
        vm.grid[7][5] = Value::Str("!".to_string());
        vm.grid[7][6] = Value::Str("°".to_string());
        vm.grid[7][7] = Value::Str("•".to_string());

        // Default weight is implicitly 100 if not set, or we can set it.
        vm.prologue_state.registers.insert((7, 7), Value::Int(100));

        // Tick 1
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Check weight of • at 7,7
        if let Some(Value::Int(w)) = vm.prologue_state.registers.get(&(7, 7)) {
            assert_eq!(*w, 110, "Learning should increase weight by 10");
        } else {
            panic!("Weight register missing");
        }
    }

    #[cfg(feature = "elektra")]
    #[test]
    fn test_varistor() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup: 20 -> ! -> ⇝
        // 20 at 7,4
        // ! at 7,5
        // ⇝ at 7,6

        vm.grid[7][4] = Value::Int(20);
        vm.grid[7][5] = Value::Str("!".to_string());
        vm.grid[7][6] = Value::Str("⇝".to_string());

        // Tick
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Check resistance at 7,6
        // Logic: Resistance = 100 - Signal = 100 - 20 = 80.
        let r = vm.resistance_grid[7][6];
        assert_eq!(r, 80.0, "Resistance should be 80.0");
    }
}
