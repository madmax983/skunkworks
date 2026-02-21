#[cfg(all(test, feature = "elektra"))]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::elektra::update_circuit;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_prologue_conductivity() {
        let mut vm = make_empty_vm();

        // Setup Circuit: ⚡ ~ ~ ≡
        // ⚡ at (5, 5)
        // ~ at (5, 6)
        // ~ at (5, 7)
        // ≡ at (5, 8)

        vm.grid[5][5] = Value::Str("⚡".to_string());
        vm.grid[5][6] = Value::Str("~".to_string());
        vm.grid[5][7] = Value::Str("~".to_string());
        vm.grid[5][8] = Value::Str("≡".to_string());

        // Step 1: Prologue Tick (Scans runes, sets Sources/Sinks)
        exec_prologue_tick(&mut vm);

        // Check if Source was set
        assert_eq!(vm.resistance_grid[5][5], -1.0);
        assert_eq!(vm.voltage_grid[5][5], 100.0);

        // Check if Sink was set
        assert_eq!(vm.resistance_grid[5][8], -2.0);
        assert_eq!(vm.voltage_grid[5][8], 0.0);

        // Step 2: Circuit Tick (Solves Voltage)
        update_circuit(&mut vm);

        // Check Voltage Propagation
        let v_mid = vm.voltage_grid[5][6];
        println!("Voltage at 5,6: {}", v_mid);
        assert!(v_mid > 10.0, "Wire should conduct voltage (got {})", v_mid);
        assert!(v_mid < 100.0, "Voltage should drop across distance");
    }

    #[test]
    fn test_electrophoresis() {
        let mut vm = make_empty_vm();

        // Setup Gradient: Source at (5, 0), Ground at (5, 10)
        vm.voltage_grid[5][0] = 100.0;
        vm.resistance_grid[5][0] = -1.0;

        vm.voltage_grid[5][10] = 0.0;
        vm.resistance_grid[5][10] = -2.0;

        // Fill with wire to ensure gradient
        for x in 1..10 {
            vm.grid[5][x] = Value::Str("~".to_string());
        }

        // Solve circuit
        update_circuit(&mut vm);

        // Place Organelle at (5, 2)
        // Voltage should be decreasing as x increases.
        // So Electrophoresis should move towards +x (lower voltage).
        vm.context_loc = (5, 2);

        // Execute Electrophoresis
        // We can manually call the exec function if exposed, or run a gene.
        // Let's run a gene.
        vm.execute_gene_inner(OpCode::Electrophoresis, &[]);

        // Check position
        assert_eq!(vm.context_loc, (5, 3), "Should move towards lower voltage");
    }

    #[test]
    fn test_modulate() {
        let mut vm = make_empty_vm();
        vm.context_loc = (5, 5);

        // Push resistance 500.0
        vm.stack.push(Value::Int(500));

        vm.execute_gene_inner(OpCode::Modulate, &[]);

        assert_eq!(vm.resistance_grid[5][5], 500.0);
    }
}
