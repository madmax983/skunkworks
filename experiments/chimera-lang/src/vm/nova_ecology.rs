#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::vm::nova::{Organelle, OrganelleType};
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use strum::IntoEnumIterator;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn spawn_random_ecology(vm: &mut ChimeraVM, count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
            break;
        }

        let x = rng.gen_range(0..crate::vm::GRID_SIZE);
        let y = rng.gen_range(0..crate::vm::GRID_SIZE);

        // Generate random DNA
        let mut genes = Vec::new();
        let len = rng.gen_range(5..20);

        let opcodes: Vec<OpCode> = OpCode::iter().collect();

        for _ in 0..len {
            let op = opcodes[rng.gen_range(0..opcodes.len())].clone();
            genes.push(crate::ast::Gene { op, args: vec![] });
        }

        let strand = crate::ast::Strand { genes };

        vm.dna.helix.strands.push(strand.clone());
        let s_idx = vm.dna.helix.strands.len() - 1;

        // Add Telomere
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        vm.organelle_id_counter += 1;

        let kind = if rng.gen_bool(0.01) {
            OrganelleType::MadScientist
        } else {
            OrganelleType::Worker
        };

        let mut traits = vec!["Wild".to_string()];
        if rng.gen_bool(0.05) { traits.push("Viral".to_string()); }
        if rng.gen_bool(0.05) { traits.push("Radioactive".to_string()); }
        if rng.gen_bool(0.05) { traits.push("Scavenger".to_string()); }

        let organelle = Organelle {
            stack: Vec::new(),
            ip: (s_idx, 0),
            context_loc: (y, x),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind,
            direction: (rng.gen_range(-1..=1), rng.gen_range(-1..=1)),
            ttl: None,
            name: format!("Eco-{}", vm.organelle_id_counter),
            traits,
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: rng.gen(),
            energy: 100,
            experience: 0,
            stage: 0,
        };
        vm.organelles.push(organelle);
    }
}

#[cfg(feature = "nova")]
pub fn process_ecology_tick(vm: &mut ChimeraVM) {
    let mut interactions: std::collections::HashMap<(usize, usize), Vec<usize>> = std::collections::HashMap::new();

    // 1. Metabolism & Map Positions
    for (i, org) in vm.organelles.iter_mut().enumerate() {
        if org.halted { continue; }

        // Metabolism
        org.energy = org.energy.saturating_sub(1);
        if org.energy <= 0 {
            org.halted = true;
            // Drop food on death
            let (y, x) = org.context_loc;
            if org.traits.contains(&"Radioactive".to_string()) {
                vm.grid[y][x] = Value::Int(-50); // Toxic Waste
            } else {
                vm.grid[y][x] = Value::Int(25); // Food
            }
            continue;
        }

        // Check for food / poison
        let (y, x) = org.context_loc;
        if let Value::Int(n) = vm.grid[y][x] {
            if n > 0 {
                org.energy = org.energy.saturating_add(n);
                org.experience = org.experience.saturating_add(n); // Gain XP from eating
                vm.grid[y][x] = Value::Int(0);
            } else if n < 0 {
                // Toxic Waste
                if org.traits.contains(&"Scavenger".to_string()) {
                    org.energy = org.energy.saturating_add(n.abs());
                    org.experience = org.experience.saturating_add(n.abs()); // Gain XP from scavenging
                    vm.grid[y][x] = Value::Int(0); // Cleaned up
                } else {
                    org.energy = org.energy.saturating_sub(n.abs()); // Radiation Damage
                }
            }
        }

        interactions.entry((y, x)).or_default().push(i);
    }

    // 2. Interactions (Predation/Mating)
    for (_loc, indices) in interactions {
        if indices.len() > 1 {
            // Battle royale: Biggest energy wins
            let mut best_idx = 0;
            let mut max_energy = -1;

            for &idx in &indices {
                // Skip already dead in this frame (though interactions usually mutual exclusive)
                if vm.organelles[idx].energy > max_energy {
                    max_energy = vm.organelles[idx].energy;
                    best_idx = idx;
                }
            }

            // Winner eats losers
            // We need to collect viral transfers first to avoid borrow issues
            let mut viral_transfers = Vec::new();

            for &idx in &indices {
                if idx != best_idx {
                    // Viral Check
                    if vm.organelles[idx].traits.contains(&"Viral".to_string()) {
                        let loser_strand = vm.organelles[idx].ip.0;
                        let winner_strand = vm.organelles[best_idx].ip.0;
                        viral_transfers.push((loser_strand, winner_strand));
                    }

                    let food = vm.organelles[idx].energy / 2;
                    vm.organelles[best_idx].energy = vm.organelles[best_idx].energy.saturating_add(food);
                    vm.organelles[idx].halted = true;
                    vm.organelles[idx].energy = 0;
                }
            }

            // Execute Viral Transfers
            for (l_idx, w_idx) in viral_transfers {
                if l_idx < vm.dna.helix.strands.len() && w_idx < vm.dna.helix.strands.len() {
                    if !vm.dna.helix.strands[l_idx].genes.is_empty() {
                        let mut rng = rand::thread_rng();
                        let gene_idx = rng.gen_range(0..vm.dna.helix.strands[l_idx].genes.len());
                        let gene = vm.dna.helix.strands[l_idx].genes[gene_idx].clone();

                        // Append to winner's strand
                        vm.dna.helix.strands[w_idx].genes.push(gene);
                        vm.output.push(format!("VIRAL: Gene transfer {} -> {}", l_idx, w_idx));
                    }
                }
            }
        }
    }

    // 3. Cleanup Dead
    vm.organelles.retain(|o| !o.halted);
}

#[cfg(feature = "nova")]
pub fn spawn_food(vm: &mut ChimeraVM) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..crate::vm::GRID_SIZE);
    let y = rng.gen_range(0..crate::vm::GRID_SIZE);

    // Food is Value::Int(energy)
    // Only if empty or overwritable
    if matches!(vm.grid[y][x], Value::Int(0)) {
        vm.grid[y][x] = Value::Int(50);
    }
}

#[cfg(feature = "nova")]
pub fn tick_ecology(vm: &mut ChimeraVM) {
    // 1. Spawn Food periodically
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.2) {
        spawn_food(vm);
    }

    // 2. Run Life Simulation
    process_ecology_tick(vm);
}

#[cfg(feature = "nova")]
pub fn cambrian_explosion(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.organelles.clear();
    vm.output.push("CAMBRIAN EXPLOSION: Mass Extinction & Rapid Speciation".to_string());

    // Spawn 50% of capacity
    spawn_random_ecology(vm, crate::vm::MAX_ORGANELLES / 2);

    // Also fill grid with food/energy to support them
    for y in 0..crate::vm::GRID_SIZE {
        for x in 0..crate::vm::GRID_SIZE {
            if rand::thread_rng().gen_bool(0.3) {
                vm.grid[y][x] = Value::Int(50);
            }
        }
    }

    None
}
