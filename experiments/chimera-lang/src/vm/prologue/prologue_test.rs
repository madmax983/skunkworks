pub mod catalyst_test;

#[cfg(test)]
mod tests {
    use crate::ast::Dna;
    use crate::ast::Helix;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    #[test]
    fn test_prologue_circuit() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        vm.grid[5][4] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        exec_prologue_tick(&mut vm);

        assert!(vm.prologue_state.signal_grid[6][5].is_some());
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(*v, 42);
        } else {
            panic!("Wire did not carry signal 42");
        }

        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,7 received Int(42)"));
    }
}
