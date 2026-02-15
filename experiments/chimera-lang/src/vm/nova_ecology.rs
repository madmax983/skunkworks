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
        let organelle = Organelle {
            stack: Vec::new(),
            ip: (s_idx, 0),
            context_loc: (y, x),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (rng.gen_range(-1..=1), rng.gen_range(-1..=1)),
            ttl: None,
            name: format!("Eco-{}", vm.organelle_id_counter),
            traits: vec!["Wild".to_string()],
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: rng.gen(),
        };
        vm.organelles.push(organelle);
    }
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

    // 2. Cull dead organelles
    // Already handled in process_organelles? No, process_organelles ticks them.
    // If they halt, they stay until removed?
    // process_organelles: "if keep { next_organelles.push(organelle); }"
    // So yes, halted ones are dropped.
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
