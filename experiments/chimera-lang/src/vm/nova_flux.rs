use super::{ChimeraVM, Organelle, Value};
use crate::vm::nova::OrganelleType;
use rand::Rng;

pub fn process_flux(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    let mut to_spawn = Vec::new();

    for y in 0..size {
        for x in 0..size {
            let entropy = vm.entropy_grid[y][x];
            if entropy > 80 {
                let mut rng = rand::thread_rng();
                // 1% chance to spawn Wisp if high entropy
                if rng.gen_bool(0.01) {
                    if vm.organelles.len() < super::MAX_ORGANELLES {
                        to_spawn.push((y, x));
                    }
                }
            }
        }
    }

    for (y, x) in to_spawn {
        vm.organelle_id_counter += 1;
        let wisp = Organelle {
            stack: Vec::new(),
            ip: (0, 0),
            context_loc: (y, x),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Wisp,
            direction: (0, 0),
            ttl: Some(50), // Limited life
            name: "Flux Wisp".to_string(),
            traits: vec!["Chaotic".to_string(), "Ephemeral".to_string()],
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: 0,
        };
        vm.organelles.push(wisp);
        vm.output.push(format!("FLUX: Wisp manifested at {},{}", x, y));
    }
}

pub fn tick_wisp(vm: &mut ChimeraVM, organelle: &mut Organelle) -> bool {
    let (cy, cx) = organelle.context_loc;

    // Increase local entropy
    vm.entropy_grid[cy][cx] = vm.entropy_grid[cy][cx].saturating_add(5).min(100);

    // Random Move
    let mut rng = rand::thread_rng();
    let dy = rng.gen_range(-1..=1);
    let dx = rng.gen_range(-1..=1);

    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        organelle.context_loc = (ny, nx);
    }

    // Random Effect (Glitch)
    if rng.gen_bool(0.1) {
        let (gy, gx) = organelle.context_loc;
        if matches!(vm.grid[gy][gx], Value::Int(0)) {
             vm.grid[gy][gx] = Value::Int(rng.gen_range(0..100));
        } else {
             // Mutate existing value
             if let Value::Int(n) = &mut vm.grid[gy][gx] {
                 *n = n.wrapping_add(rng.gen_range(-10..=10));
             }
        }
        vm.output.push(format!("WISP: Glitch at {},{}", gx, gy));
    }

    !organelle.halted
}
