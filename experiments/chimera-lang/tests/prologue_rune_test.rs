#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::vm::prologue::exec_prologue_tick;

    #[test]
    fn test_prologue_basic_circuit() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: 42 -> ! -> ~ -> ?
        vm.grid[4][5] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        exec_prologue_tick(&mut vm);

        // Check if signal propagated to wire
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(*v, 42);
        } else {
            panic!("Wire at (6,5) did not carry signal 42");
        }

        // Check output
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,7 received Int(42)"));
    }

    #[test]
    fn test_prologue_xor_gate() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // XOR Gate at (5,5)
        // Inputs at (5,4) (West) and (5,6) (East)
        // We simulate signals directly by placing sources feeding into wires
        // Or simpler: Manually inject signals into prologue_state.signal_grid
        // But let's build the circuit to test propagation fully.

        // Case 1: 0 XOR 0 -> 0 (No Signal)
        // (5,5) = +
        vm.grid[5][5] = Value::Str("+".to_string());
        exec_prologue_tick(&mut vm);
        assert!(vm.prologue_state.signal_grid[5][5].is_none());

        // Case 2: 1 XOR 0 -> 1
        // West Input: 1 -> ! -> ~ -> +
        // (5,2)=1, (5,3)=!, (5,4)=~
        vm.grid[5][2] = Value::Int(1);
        vm.grid[5][3] = Value::Str("!".to_string()); // ! reads North (4,3). Wait.
        // ! reads North (y-1).
        // Let's put value at (4,3).
        vm.grid[4][3] = Value::Int(1);

        // This setup is tricky because '!' is a Source.
        // Let's just manually set signal_grid to simulate input arriving at West/East neighbor wires.
        // But exec_prologue_tick clears signal_grid at start!
        // So we MUST use the grid components.

        // Setup XOR at (5,5)
        vm.grid[5][5] = Value::Str("+".to_string());

        // West Input: Source at (5,4)? No, Source is '!'.
        // Let's use Source '!' at (5,4). It reads from (4,4).
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[4][4] = Value::Int(1); // Input A

        // East Input: Source '!' at (5,6). Reads from (4,6).
        vm.grid[5][6] = Value::Str("!".to_string());
        vm.grid[4][6] = Value::Int(0); // Input B (Empty/Zero)

        exec_prologue_tick(&mut vm);
        // (5,4) should have signal 1.
        // (5,6) should have no signal (0 is empty).
        // (5,5) should have signal 1 (1 XOR 0 = 1).
        assert!(vm.prologue_state.signal_grid[5][4].is_some());
        assert!(vm.prologue_state.signal_grid[5][6].is_none());
        assert!(vm.prologue_state.signal_grid[5][5].is_some());

        // Case 3: 1 XOR 1 -> 0
        vm.grid[4][6] = Value::Int(1); // Set B to 1
        exec_prologue_tick(&mut vm);
        assert!(vm.prologue_state.signal_grid[5][4].is_some());
        assert!(vm.prologue_state.signal_grid[5][6].is_some());
        assert!(vm.prologue_state.signal_grid[5][5].is_none(), "1 XOR 1 should be 0 (None)");
    }

    #[test]
    fn test_prologue_splitter() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Splitter at (5,5)
        // Input from North: ! at (4,5) reading (3,5)
        vm.grid[5][5] = Value::Str("*".to_string());
        vm.grid[4][5] = Value::Str("!".to_string());
        vm.grid[3][5] = Value::Int(99);

        // Output Wires at (5,4) and (5,6)
        vm.grid[5][4] = Value::Str("~".to_string());
        vm.grid[5][6] = Value::Str("~".to_string());

        exec_prologue_tick(&mut vm);

        // ! emits at (4,5)
        // * at (5,5) reads North (4,5), gets 99.
        // * becomes active.
        // ~ at (5,4) reads East (5,5), gets 99.
        // ~ at (5,6) reads West (5,5), gets 99.

        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][4] {
            assert_eq!(*v, 99);
        } else {
            panic!("West wire failed to receive split signal");
        }

        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][6] {
            assert_eq!(*v, 99);
        } else {
            panic!("East wire failed to receive split signal");
        }
    }

    #[test]
    fn test_prologue_delay() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Delay at (5,5)
        // Input from North: ! at (4,5) reading (3,5)
        vm.grid[5][5] = Value::Str("#".to_string());
        vm.grid[4][5] = Value::Str("!".to_string());
        vm.grid[3][5] = Value::Int(77);

        // Output Sink at (6,5)
        vm.grid[6][5] = Value::Str("?".to_string());

        // Tick 1:
        // ! emits 77 at (4,5).
        // # reads 77 from (4,5). Stores in delayed_signals.
        // # does NOT emit to (5,5) in this tick.
        // ? reads (5,5), sees nothing.
        exec_prologue_tick(&mut vm);

        assert!(vm.prologue_state.signal_grid[5][5].is_none(), "Delay should not emit on first tick");
        assert!(vm.prologue_state.delayed_signals[5][5].is_some(), "Delay should store signal");

        // Tick 2:
        // delayed_signals copied to signal_grid at start. (5,5) has 77.
        // ! emits 77 again (source is constant).
        // # reads 77 again. Stores for Tick 3.
        // ? reads (5,5), sees 77.

        // Clear output before tick 2
        vm.output.clear();
        exec_prologue_tick(&mut vm);

        assert!(vm.prologue_state.signal_grid[5][5].is_some(), "Delay should emit on second tick");
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,6 received Int(77)"));
    }

    #[test]
    fn test_prologue_gene_trigger() {
        // Setup DNA with a named strand "test_strand"
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(999)] },
            Gene { op: OpCode::Print, args: vec![] },
        ];
        let strand = Strand { genes };

        let dna = Dna { helix: Helix { strands: vec![strand, Strand { genes: vec![] }] } };
        // We add a dummy strand at 1 so "test_strand" can be 0 or 1.
        // Wait, interrupt works by index.

        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Register the strand name. Let's point to index 0.
        vm.dictionary.insert("test_strand".to_string(), 0);

        // Setup Circuit: "test_strand" -> ! -> ~ -> ?
        // (4,5) value
        // (5,5) !
        // (6,5) ~
        // (7,5) ?
        vm.grid[4][5] = Value::Str("test_strand".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        // Move IP away from (0,0) to prove jump happens.
        vm.ip = (1, 0);

        exec_prologue_tick(&mut vm);

        // Sink reads "test_strand". Triggers interrupt(0).
        // IP should become (0,0). Call stack should have (1,0).

        assert_eq!(vm.ip, (0, 0));
        assert_eq!(vm.call_stack.len(), 1);
        assert_eq!(vm.call_stack[0], (1, 0));
    }
}
