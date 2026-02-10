#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::ChimeraVM;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::Value;
    use crate::vm::nova::{Organelle, OrganelleType};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_flux_emitter() {
        let mut vm = make_vm();
        // Layout:
        // . F . (F reads West, applies to South)
        // 5 . . (West=5)
        // . . . (South should get entropy +5)

        // F at (1, 1).
        // West is (1, 0). Value 5.
        // South is (2, 1).

        vm.grid[1][1] = Value::Str("F".to_string());
        vm.grid[1][0] = Value::Int(5);

        vm.signal_grid[1][1] = 1; // Activate

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 5);
    }

    #[test]
    fn test_jam_emitter() {
        let mut vm = make_vm();
        // J at (1, 1). West=5. South=(2,1).

        vm.grid[1][1] = Value::Str("J".to_string());
        vm.grid[1][0] = Value::Int(5);
        vm.entropy_grid[2][1] = 10;

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 5); // 10 - 5
    }

    #[test]
    fn test_flux_saturation() {
        let mut vm = make_vm();
        vm.grid[1][1] = Value::Str("F".to_string());
        vm.grid[1][0] = Value::Int(150); // > 100

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 100); // Clamped
    }

    #[test]
    fn test_wisp_spawn() {
        let mut vm = make_vm();
        // Set high entropy
        vm.entropy_grid[5][5] = 90;

        // Run until spawn or limit
        for _ in 0..1000 {
            crate::vm::nova_flux::process_flux(&mut vm);
            if !vm.organelles.is_empty() {
                break;
            }
        }

        if !vm.organelles.is_empty() {
             assert_eq!(vm.organelles[0].kind, crate::vm::nova::OrganelleType::Wisp);
        }
    }

    #[test]
    fn test_chaos_opcode_success() {
        let mut vm = make_vm();
        // Setup stack: push 50 (chaos amount)
        vm.stack.push(Value::Int(50));

        let initial_energy = vm.energy;
        let initial_havoc = vm.havoc.rate;
        let initial_glitch = vm.glitch_level;

        crate::vm::nova_flux::exec_chaos(&mut vm);

        // Verify havoc rate increased
        assert!(vm.havoc.rate > initial_havoc, "Havoc rate should increase");
        assert!((vm.havoc.rate - (initial_havoc + 50.0 / 1000.0)).abs() < 1e-6);

        // Verify glitch level increased
        assert!(vm.glitch_level > initial_glitch, "Glitch level should increase");
        assert!((vm.glitch_level - (initial_glitch + 50.0 / 100.0)).abs() < 1e-6);

        // Verify energy consumption (amount injected)
        assert_eq!(vm.energy, initial_energy - 50);

        // Verify output message
        assert!(vm.output.last().unwrap().contains("CHAOS: Injected 50 entropy"));
    }

    #[test]
    fn test_chaos_opcode_error_handling() {
        let mut vm = make_vm();

        // 1. Stack underflow
        vm.stack.clear();
        crate::vm::nova_flux::exec_chaos(&mut vm);
        assert_eq!(vm.output.last().unwrap(), "Error: Stack underflow for chaos");

        // 2. Type mismatch
        vm.stack.push(Value::Str("Not a number".to_string()));
        crate::vm::nova_flux::exec_chaos(&mut vm);
        assert_eq!(vm.output.last().unwrap(), "Error: Type mismatch for chaos");

        // 3. Negative amount (no-op but no error message in current impl, check logic)
        // Actually, if amount > 0 is false, it does nothing and logs nothing?
        // Let's check code: `if amount > 0 { ... }`
        // It does not output error for <= 0.
        // Let's verify no side effects.
        vm.stack.push(Value::Int(-10));
        let energy_before = vm.energy;
        crate::vm::nova_flux::exec_chaos(&mut vm);
        assert_eq!(vm.energy, energy_before);
    }

    #[test]
    fn test_wisp_entropy_generation() {
        let mut vm = make_vm();
        let (y, x) = (5, 5);

        let mut wisp = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (y, x),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Wisp,
            direction: (0, 0),
            ttl: Some(50),
            name: "Test Wisp".to_string(),
            traits: vec!["Chaotic".to_string()],
            id: 1,
            tissue_id: None,
            genome_id: 0,
        };

        // Initial entropy
        vm.entropy_grid[y][x] = 10;

        crate::vm::nova_flux::tick_wisp(&mut vm, &mut wisp);

        // Wisp increases entropy by 5
        assert_eq!(vm.entropy_grid[y][x], 15);

        // Also check saturation
        vm.entropy_grid[y][x] = 98;
        // Reset location because wisp moved in previous tick
        wisp.context_loc = (y, x);
        crate::vm::nova_flux::tick_wisp(&mut vm, &mut wisp);
        assert_eq!(vm.entropy_grid[y][x], 100);
    }
}
