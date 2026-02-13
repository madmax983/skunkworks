#[cfg(all(feature = "nova", feature = "elektra"))]
use crate::vm::{nova::Organelle, ChimeraVM};
#[cfg(all(feature = "nova", feature = "elektra"))]
use rand::Rng;

#[cfg(all(feature = "nova", feature = "elektra"))]
pub fn process_dynamo(vm: &mut ChimeraVM, organelle: &mut Organelle) {
    let (cy, cx) = vm.context_loc;

    // 1. Induction: Harvest Energy from Current (Magnetic Flux)
    // Dynamo thrives in high-current environments.
    // Ensure coordinates are within bounds (should be guaranteed by context_loc)
    if cy < crate::vm::GRID_SIZE && cx < crate::vm::GRID_SIZE {
        let current = vm.current_grid[cy][cx];
        if current > 0.1 {
            // Efficiency: 5 Energy per unit of current
            let gain = (current * 5.0) as i64;
            if gain > 0 {
                vm.energy = vm.energy.saturating_add(gain);
                vm.output
                    .push(format!("DYNAMO: Inducted {} Energy from {:.2}A", gain, current));
            }
        }

        // 2. Discharge: Release Excess Energy as Voltage
        // If Energy > 200, Dynamo becomes unstable and discharges.
        if vm.energy > 200 {
            let excess = vm.energy - 200;
            let discharge_amount = excess.min(50); // Cap discharge

            // Convert Energy -> Voltage (1:1 ratio for now)
            let voltage = discharge_amount as f32;
            vm.voltage_grid[cy][cx] += voltage;
            vm.energy -= discharge_amount;

            vm.output.push(format!(
                "DYNAMO: Discharged {:.1}V due to overcharge",
                voltage
            ));
        }

        // 3. Electromotility: Random movement if powered
        // Dynamo jitters if there is voltage nearby
        let voltage = vm.voltage_grid[cy][cx];
        if voltage > 5.0 {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.2) {
                let dy = rng.gen_range(-1..=1);
                let dx = rng.gen_range(-1..=1);
                organelle.direction = (dy, dx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{
        nova::{Organelle, OrganelleType},
        ChimeraVM,
    };
    use crate::ast::{Dna, Helix, Strand};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_dynamo_induction() {
        let mut vm = make_empty_vm();

        // Setup Dynamo
        let (cy, cx) = (8, 8);
        vm.context_loc = (cy, cx);

        let mut organelle = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (cy, cx),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Dynamo,
            direction: (0, 0),
            ttl: None,
            name: "Test Dynamo".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
        };

        // Inject current
        vm.current_grid[cy][cx] = 2.0;
        vm.energy = 50;

        // Process
        super::process_dynamo(&mut vm, &mut organelle);

        // Check energy gain
        // Gain = 2.0 * 5.0 = 10
        // Final = 50 + 10 = 60
        assert_eq!(vm.energy, 60);

        // Check output log
        assert!(vm.output.iter().any(|s| s.contains("DYNAMO: Inducted")));
    }

    #[test]
    fn test_dynamo_discharge() {
        let mut vm = make_empty_vm();

        // Setup Dynamo
        let (cy, cx) = (8, 8);
        vm.context_loc = (cy, cx);

        let mut organelle = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (cy, cx),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Dynamo,
            direction: (0, 0),
            ttl: None,
            name: "Overcharged Dynamo".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0,
        };

        // Overcharge
        vm.energy = 300;

        // Process
        super::process_dynamo(&mut vm, &mut organelle);

        // Check discharge
        // Excess = 300 - 200 = 100
        // Cap = 50
        // Voltage += 50
        // Energy -= 50 = 250

        assert_eq!(vm.energy, 250);
        assert_eq!(vm.voltage_grid[cy][cx], 50.0);
        assert!(vm.output.iter().any(|s| s.contains("DYNAMO: Discharged")));
    }
}
