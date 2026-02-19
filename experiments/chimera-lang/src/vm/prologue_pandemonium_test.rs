#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::prologue::exec_prologue_tick;

    #[test]
    fn test_chaos_source() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Place Chaos Source ¿ at (5,5)
        // It should emit to North (4,5)
        vm.grid[5][5] = Value::Str("¿".to_string());

        exec_prologue_tick(&mut vm);

        // Check if signal exists at (4,5)
        assert!(vm.prologue_state.signal_grid[4][5].is_some());
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[4][5] {
            assert!(*v >= 0 && *v < 100);
        } else {
            panic!("Chaos Source did not emit Int");
        }
    }

    #[test]
    fn test_noise_wire() {
         let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 42 -> ! -> ≈ -> ?
        // ! reads from West (5,4)
        vm.grid[5][4] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("≈".to_string()); // South of !
        vm.grid[7][5] = Value::Str("?".to_string()); // South of ≈

        exec_prologue_tick(&mut vm);

        // Check wire signal
        assert!(vm.prologue_state.signal_grid[6][5].is_some());
        // Check sink received something
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,7 received"));
    }

    #[test]
    fn test_chaos_sink() {
         let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 42 -> ! -> ~ -> ¡
        // ! reads from West (5,4)
        vm.grid[5][4] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string()); // South of !
        vm.grid[7][5] = Value::Str("¡".to_string()); // South of ~

        exec_prologue_tick(&mut vm);

        // Check sink triggered
        let output = vm.output.join("\n");
        assert!(output.contains("PANDEMONIUM:"));
    }
}
