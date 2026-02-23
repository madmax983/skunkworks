#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

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
    fn test_prologue_agent_ghosting() {
        // Reproduction of the "Ghost Agent" bug using Hunters.

        let mut vm = setup_vm();

        // Setup:
        // (5, 5) H (Hunter A)
        // (5, 6) @ (Seeker B)
        // (5, 7) ~ (Wire)
        // (5, 8) ! (Source)

        vm.grid[5][5] = Value::Str("H".to_string());
        vm.grid[5][6] = Value::Str("@".to_string());
        vm.grid[5][7] = Value::Str("~".to_string());
        vm.grid[5][8] = Value::Str("!".to_string());

        exec_prologue_tick(&mut vm);

        // B (@) should move to (5, 7) (Wire).
        assert_eq!(
            vm.grid[5][7],
            Value::Str("@".to_string()),
            "Seeker B should move to wire/signal"
        );

        // A (H) should move to (5, 6).
        assert_eq!(
            vm.grid[5][6],
            Value::Str("H".to_string()),
            "Hunter A should move to prey"
        );

        // Old pos (5, 5) should be empty
        assert_eq!(vm.grid[5][5], Value::Int(0), "Old pos should be empty");
    }
}
