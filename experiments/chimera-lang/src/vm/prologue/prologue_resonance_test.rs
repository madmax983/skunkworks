#[cfg(test)]
mod tests {
    use crate::ast::Dna;
    use crate::ast::Helix;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    #[test]
    fn test_resonance_note() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: 60 -> ! -> ~ -> ♪
        // ! at 5,5 reads 5,4 (60) -> Emits 60 at 5,5
        // ~ at 6,5 reads 5,5 (60) -> Emits 60 at 6,5
        // ♪ at 7,5 reads 7,4... Wait.
        // ♪ Reads WEST.
        // So circuit needs to be horizontal.

        // 60 -> ! -> ~ -> ♪
        // 60 at 5,4
        // ! at 5,5 (Reads 60)
        // ~ at 5,6 (Reads 60 from West)
        // ♪ at 5,7 (Reads 60 from West)

        vm.grid[5][4] = Value::Int(60);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("~".to_string());
        vm.grid[5][7] = Value::Str("♪".to_string());

        exec_prologue_tick(&mut vm);

        // Check output
        let output = vm.output.join("\n");
        assert!(
            output.contains("RESONANCE: Note 60"),
            "Output was: {}",
            output
        );
    }

    #[test]
    fn test_resonance_chord() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit:
        //      1 (Type)
        //      !
        // 60 ! ♫ (Chord)

        // Type:
        // 1 at 4,4
        // ! at 4,5 (Reads 1 from 4,4). Emits 1 at 4,5.
        // ♫ at 5,5 (Reads North 4,5).

        // Root:
        // 60 at 5,3
        // ! at 5,4 (Reads 60 from 5,3). Emits 60 at 5,4.
        // ♫ at 5,5 (Reads West 5,4).

        vm.grid[4][4] = Value::Int(1);
        vm.grid[4][5] = Value::Str("!".to_string());

        vm.grid[5][3] = Value::Int(60);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("♫".to_string());

        exec_prologue_tick(&mut vm);

        let output = vm.output.join("\n");
        assert!(
            output.contains("RESONANCE: Chord 60 Type 1"),
            "Output was: {}",
            output
        );
    }

    #[test]
    fn test_resonance_drum() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // 1 -> ! -> 🥁
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("🥁".to_string());

        exec_prologue_tick(&mut vm);

        let output = vm.output.join("\n");
        assert!(output.contains("RESONANCE: Drum"), "Output was: {}", output);
    }
}
