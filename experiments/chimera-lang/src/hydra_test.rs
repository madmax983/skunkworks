#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_hydra_pump() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.grid[5][5] = Value::Str("@".to_string());

        // Step
        vm.step();

        // Moisture should be added (Pump adds 50 per tick, then decays)
        assert!(
            vm.moisture_grid[5][5] >= 40,
            "Pump should add moisture (approx 50)"
        );
    }

    #[test]
    fn test_hydra_flow() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);

        // Set initial moisture manually
        vm.moisture_grid[5][5] = 500;
        // Fan right at 5,5 (pushes East with strength 5)
        vm.grid[5][5] = Value::Str(">".to_string());

        vm.step();

        // Wind should be advected
        // Source 5,5 -> Wind 5 sets target 5,10.
        // Wind moves to 5,10 (decayed to 4). Source becomes 0.
        assert_eq!(
            vm.wind_grid[5][5],
            (0, 0),
            "Wind should move away from source"
        );
        assert_eq!(vm.wind_grid[5][10], (0, 4), "Wind should arrive at target");

        // Moisture should move East (dx=5 because simulation uses current wind before decay update)
        // Source 5,5 -> Target 5,10
        assert!(
            vm.moisture_grid[5][10] > 0,
            "Moisture should advect East to 5,10"
        );
        // Some stays behind due to splitting
        assert!(vm.moisture_grid[5][5] > 0, "Some moisture remains");
    }

    #[test]
    fn test_hydra_wall() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);

        // Moisture moving East towards a wall
        vm.moisture_grid[5][5] = 500;
        vm.wind_grid[5][5] = (0, 1); // Move 1 right

        // Wall at 5,6
        vm.grid[5][6] = Value::Str("#".to_string());

        vm.step();

        // Should NOT be at 5,6 (Wall)
        assert_eq!(vm.moisture_grid[5][6], 0, "Wall should block moisture");

        // Should be at 5,5 (Reflected/Stayed)
        assert!(
            vm.moisture_grid[5][5] > 0,
            "Moisture should be reflected/stay"
        );
    }
}
