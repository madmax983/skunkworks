#[cfg(test)]
mod tests {
    use chimera_lang::vm::ChromaCell;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::exec_prologue_tick;

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_red_amplification() {
        // Red '+' doubles the result
        let mut vm = make_vm();

        // Setup: 5 + 5 -> Red '+' -> ?
        // We use delayed_signals to ensure they persist past prepare_signals phase
        vm.grid[5][4] = Value::Int(5);
        vm.grid[5][6] = Value::Int(5);
        vm.grid[5][5] = Value::Str("+".to_string());

        // Paint it Red
        vm.chroma_grid[5][5] = ChromaCell {
            fg: Some((255, 0, 0)),
            char: None,
        };

        // Source inputs
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(5));
        vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(5));

        exec_prologue_tick(&mut vm);

        // Result at South (6,5) should be (5+5)*2 = 20
        match &vm.prologue_state.signal_grid[6][5] {
            Some(Value::Int(n)) => assert_eq!(*n, 20, "Expected Red Amplification (20), got {}", n),
            _ => panic!("Expected signal at output, got {:?}", vm.prologue_state.signal_grid[6][5]),
        }
    }

    #[test]
    fn test_green_crossover() {
        // Green '+' performs genetic/string crossover
        let mut vm = make_vm();

        // "AAAA" + "BBBB" -> Green '+' -> "AABB" (roughly)
        vm.grid[5][4] = Value::Str("AAAA".to_string());
        vm.grid[5][6] = Value::Str("BBBB".to_string());
        vm.grid[5][5] = Value::Str("+".to_string());

        vm.chroma_grid[5][5] = ChromaCell {
            fg: Some((0, 255, 0)),
            char: None,
        };

        vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("AAAA".to_string()));
        vm.prologue_state.delayed_signals[5][6] = Some(Value::Str("BBBB".to_string()));

        exec_prologue_tick(&mut vm);

        match &vm.prologue_state.signal_grid[6][5] {
            Some(Value::Str(s)) => {
                assert_eq!(s, "AABB");
            },
            _ => panic!("Expected string signal, got {:?}", vm.prologue_state.signal_grid[6][5]),
        }
    }

    #[test]
    fn test_blue_logic() {
        // Blue '+' acts as AND gate (Logical Conjunction)
        let mut vm = make_vm();

        // 1 + 1 -> Blue '+' -> 1 (AND)
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][6] = Value::Int(1);
        vm.grid[5][5] = Value::Str("+".to_string());

        vm.chroma_grid[5][5] = ChromaCell {
            fg: Some((0, 0, 255)),
            char: None,
        };

        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(1));
        vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(1));

        exec_prologue_tick(&mut vm);

        match &vm.prologue_state.signal_grid[6][5] {
            Some(Value::Int(n)) => assert_eq!(*n, 1, "Expected Blue Logic AND (1), got {}", n),
            _ => panic!("Expected signal, got {:?}", vm.prologue_state.signal_grid[6][5]),
        }
    }
}
