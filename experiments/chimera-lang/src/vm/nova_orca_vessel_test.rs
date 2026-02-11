#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix, Strand};

    fn make_empty_dna() -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }], // Strand 0 empty
            },
        }
    }

    #[test]
    fn test_orca_vessel_spawn() {
        let mut vm = ChimeraVM::new(make_empty_dna());

        // Setup Grid for 'V' operator
        // . 1 .  (North: Type 1 = Chloroplast)
        // . V 0  (Center: V, East: Strand 0)
        // . * .  (South: Bang to trigger)

        // Coordinates
        let cy = 8;
        let cx = 8;

        vm.grid[cy-1][cx] = Value::Int(1); // Type 1
        vm.grid[cy][cx] = Value::Str("V".to_string());
        vm.grid[cy][cx+1] = Value::Int(0); // Strand 0
        vm.grid[cy+1][cx] = Value::Str("*".to_string()); // Bang

        // Initial state: 0 organelles
        assert_eq!(vm.organelles.len(), 0);

        // Step 1: Bang fires signal to V
        vm.step();

        // Step 2: V receives signal and executes Spawn
        // Logic: process_signals runs in step().
        // * generates signal for neighbors (including V).
        // V sees signal > 0. Reads North(1) and East(0).
        // Pushes Push(0), Push(1), Spawn to executions.
        // Execution phase runs Spawn.

        // Check if organelle spawned
        // Note: process_signals might run before or after signal propagation depending on implementation details.
        // Let's run a few steps to be sure.
        for _ in 0..3 {
            vm.step();
        }

        assert!(!vm.organelles.is_empty(), "Organelle should have been spawned");
        let org = &vm.organelles[0];
        assert_eq!(org.kind, crate::vm::nova::OrganelleType::Chloroplast);
        assert_eq!(org.ip.0, 0); // Executing Strand 0
    }
}
