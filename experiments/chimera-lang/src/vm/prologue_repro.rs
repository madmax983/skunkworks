#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_collect_scatter_runes() {
        let mut vm = setup_vm();

        // Setup Collect [ at 5,5
        // Inputs via delayed_signals to bypass Source logic
        vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(1)); // N
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(2)); // W
        vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(3)); // E
        vm.prologue_state.delayed_signals[6][5] = Some(Value::Int(4)); // S

        vm.grid[5][5] = Value::Str("[".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][5];
        if let Some(Value::Junction(JunctionType::Any, vals)) = sig {
            assert!(vals.contains(&Value::Int(1)));
            assert!(vals.contains(&Value::Int(2)));
            assert!(vals.contains(&Value::Int(3)));
            assert!(vals.contains(&Value::Int(4)));
        } else {
            panic!("Collect [ rune failed to produce Junction. Got: {:?}", sig);
        }

        // Setup Scatter ] at 8,5
        // Input West (8,4)
        let junction = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(10), Value::Int(20), Value::Int(30)],
        );
        vm.prologue_state.delayed_signals[8][4] = Some(junction);

        vm.grid[8][5] = Value::Str("]".to_string());

        exec_prologue_tick(&mut vm);

        // Check outputs
        // N (7,5) -> Item 0
        let n_sig = &vm.prologue_state.signal_grid[7][5];
        // E (8,6) -> Item 1
        let e_sig = &vm.prologue_state.signal_grid[8][6];
        // S (9,5) -> Item 2
        let s_sig = &vm.prologue_state.signal_grid[9][5];

        assert_eq!(n_sig, &Some(Value::Int(10)));
        assert_eq!(e_sig, &Some(Value::Int(20)));
        assert_eq!(s_sig, &Some(Value::Int(30)));
    }

    #[test]
    fn test_list_ops_runes() {
        let mut vm = setup_vm();

        // U (Head) at 5,5
        // West: [1, 2, 3]
        let list = Value::Junction(JunctionType::Any, vec![Value::Int(1), Value::Int(2)]);

        vm.prologue_state.delayed_signals[5][4] = Some(list.clone());
        vm.grid[5][5] = Value::Str("U".to_string());

        exec_prologue_tick(&mut vm);

        let head = &vm.prologue_state.signal_grid[5][5]; // Output Self
        assert_eq!(head, &Some(Value::Int(1)));

        // V (Tail) at 8,5
        vm.prologue_state.delayed_signals[8][4] = Some(list.clone());
        vm.grid[8][5] = Value::Str("V".to_string());

        exec_prologue_tick(&mut vm);

        let tail = &vm.prologue_state.signal_grid[8][5]; // Output Self
        if let Some(Value::Junction(_, vals)) = tail {
            assert_eq!(vals.len(), 1);
            assert_eq!(vals[0], Value::Int(2));
        } else {
            panic!("Tail V rune failed. Got: {:?}", tail);
        }
    }
}
