use super::{ChimeraVM, Value, MAX_ORGANELLES, MAX_STRANDS};
use crate::opcode::OpCode;
use crate::vm::nova::{Organelle, OrganelleType};
use crate::vm::nova_bestiary;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn organelle_type_from_int(t: i64) -> (OrganelleType, (i8, i8)) {
    match t {
        1 => (OrganelleType::Chloroplast, (0, 0)),
        2 => (OrganelleType::Mitochondria, (0, 0)),
        3 => (OrganelleType::Lysosome, (0, 0)),
        4 => (OrganelleType::Ribosome, (0, 1)), // Default East
        5 => (OrganelleType::Void, (0, 0)),
        6 => (OrganelleType::Alchemist, (0, 0)),
        10 => (OrganelleType::MadScientist, (0, 0)),
        11 => (OrganelleType::Phage, (0, 1)), // Default East
        12 => (OrganelleType::Savant, (0, 0)),
        13 => (OrganelleType::Metazoan, (0, 0)),
        _ => (OrganelleType::Worker, (0, 0)),
    }
}

pub fn exec_spawn(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: type, strand_idx (bottom)
    let t = vm.pop_int("spawn")?;
    let idx = vm.pop_int("spawn")?;

    let s_idx = idx as usize;

    if s_idx >= vm.dna.helix.strands.len() {
        vm.output
            .push("Error: Strand index out of bounds for spawn".to_string());
        return None;
    }

    if vm.organelles.len() >= MAX_ORGANELLES {
        vm.output
            .push("Error: Organelle limit exceeded".to_string());
        return None;
    }

    let (kind, direction) = organelle_type_from_int(t);

    let strand = &vm.dna.helix.strands[s_idx];
    let mut hasher = DefaultHasher::new();
    strand.hash(&mut hasher);
    let genome_id = hasher.finish();
    let traits = nova_bestiary::analyze_traits(strand);
    let name = nova_bestiary::generate_name(genome_id, &traits);

    vm.organelle_id_counter += 1;
    let organelle = Organelle {
        stack: Vec::new(),
        ip: (s_idx, 0),
        context_loc: vm.context_loc,
        call_stack: Vec::new(),
        recursion_depth: 0,
        halted: false,
        kind: kind.clone(),
        direction,
        ttl: None,
        name,
        traits,
        id: vm.organelle_id_counter,
        tissue_id: None,
        genome_id,
        energy: 50,
        experience: 0,
        stage: 0,
    };
    vm.organelles.push(organelle);
    vm.energy = vm.energy.saturating_sub(20);
    vm.output.push(format!(
        "SPAWN: Created {:?} Organelle executing strand {}",
        kind, s_idx
    ));

    None
}

pub fn exec_identity(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let id = match &vm.active_organelle_kind {
        None => -1, // Nucleus
        Some(OrganelleType::Worker) => 0,
        Some(OrganelleType::Chloroplast) => 1,
        Some(OrganelleType::Mitochondria) => 2,
        Some(OrganelleType::Lysosome) => 3,
        Some(OrganelleType::Ribosome) => 4,
        Some(OrganelleType::Void) => 5,
        Some(OrganelleType::Alchemist) => 6,
        Some(OrganelleType::Seed) => 7,
        Some(OrganelleType::Choir) => 8,
        Some(OrganelleType::Wisp) => 9,
        Some(OrganelleType::MadScientist) => 10,
        Some(OrganelleType::Phage) => 11,
        Some(OrganelleType::Savant) => 12,
        Some(OrganelleType::Metazoan) => 13,
    };
    vm.stack.push(Value::Int(id));
    None
}

pub fn exec_differentiate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if vm.active_organelle_kind.is_some() {
            if let Value::Int(t) = val {
                let new_kind = match t {
                    1 => Some(OrganelleType::Chloroplast),
                    2 => Some(OrganelleType::Mitochondria),
                    3 => Some(OrganelleType::Lysosome),
                    4 => Some(OrganelleType::Ribosome),
                    5 => Some(OrganelleType::Void),
                    6 => Some(OrganelleType::Alchemist),
                    7 => Some(OrganelleType::Seed),
                    8 => Some(OrganelleType::Choir),
                    9 => Some(OrganelleType::Wisp),
                    10 => Some(OrganelleType::MadScientist),
                    11 => Some(OrganelleType::Phage),
                    12 => Some(OrganelleType::Savant),
                    13 => Some(OrganelleType::Metazoan),
                    _ => Some(OrganelleType::Worker),
                };

                if let Some(kind) = new_kind {
                    vm.signal_differentiation = Some(kind.clone());
                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output
                        .push(format!("DIFFERENTIATE: Requesting change to {:?}", kind));
                }
            } else {
                vm.output
                    .push("Error: Type mismatch for differentiate".to_string());
            }
        } else {
            vm.output
                .push("Error: Nucleus cannot differentiate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for differentiate".to_string());
    }
    None
}

pub fn exec_symbiosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let dx = vm.pop_int("symbiosis")?;
    let dy = vm.pop_int("symbiosis")?;

    let (cy, cx) = vm.context_loc;
    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        let mut found_idx = None;
        for (i, org) in vm.organelles.iter().enumerate() {
            if org.context_loc == (ny, nx) {
                found_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = found_idx {
            let organelle = vm.organelles.remove(idx);
            vm.symbiotes.push(organelle.ip);
            vm.stack.extend(organelle.stack);
            vm.energy = vm.energy.saturating_sub(20);
            vm.output
                .push(format!("SYMBIOSIS: Absorbed organelle at {},{}", nx, ny));
        } else {
            vm.output.push("SYMBIOSIS: No organelle found".to_string());
        }
    } else {
        vm.output
            .push("Error: Coordinates out of bounds for symbiosis".to_string());
    }
    None
}

pub fn exec_lysis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(sip) = vm.symbiotes.pop() {
        if vm.organelles.len() >= MAX_ORGANELLES {
            vm.symbiotes.push(sip);
            vm.output
                .push("Error: Organelle limit exceeded".to_string());
            return None;
        }

        let (cy, cx) = vm.context_loc;
        vm.organelle_id_counter += 1;
        let organelle = Organelle {
            stack: Vec::new(),
            ip: sip,
            context_loc: (cy, cx),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Symbiote Spawn".to_string(),
            traits: vec!["Ejected".to_string()],
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: 0,
            energy: 25,
            experience: 0,
            stage: 0,
        };
        vm.organelles.push(organelle);
        vm.energy = vm.energy.saturating_sub(10);
        vm.output
            .push(format!("LYSIS: Ejected symbiote to {},{}", cx, cy));
    } else {
        vm.output.push("LYSIS: No symbiotes to eject".to_string());
    }
    None
}

pub fn exec_secrete(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let amount_val = vm.stack.pop().unwrap();
        let channel_val = vm.stack.pop().unwrap();
        if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
            if a > 0 {
                let (cy, cx) = vm.context_loc;
                let channel_idx = (c.unsigned_abs() as usize) % 3;
                vm.hormone_grid[cy][cx][channel_idx] =
                    vm.hormone_grid[cy][cx][channel_idx].saturating_add(a);
                vm.output.push(format!(
                    "SECRETE: Added {} to channel {} at {},{}",
                    a, c, cx, cy
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for secrete".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for secrete".to_string());
    }
    None
}

pub fn exec_detect(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(c) = val {
            let (cy, cx) = vm.context_loc;
            let channel_idx = (c.unsigned_abs() as usize) % 3;
            let intensity = vm.hormone_grid[cy][cx][channel_idx];
            vm.stack.push(Value::Int(intensity));
        } else {
            vm.output
                .push("Error: Type mismatch for detect".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for detect".to_string());
    }
    None
}

pub fn exec_absorb(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let amount_val = vm.stack.pop().unwrap();
        let channel_val = vm.stack.pop().unwrap();
        if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
            let (cy, cx) = vm.context_loc;
            let channel_idx = (c.unsigned_abs() as usize) % 3;
            let intensity = &mut vm.hormone_grid[cy][cx][channel_idx];
            let absorbed = if *intensity >= a {
                *intensity = intensity.saturating_sub(a);
                a
            } else {
                let v = *intensity;
                *intensity = 0;
                v
            };
            vm.stack.push(Value::Int(absorbed));
            vm.output.push(format!(
                "ABSORB: Consumed {} from channel {} at {},{}",
                absorbed, c, cx, cy
            ));
        } else {
            vm.output
                .push("Error: Type mismatch for absorb".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for absorb".to_string());
    }
    None
}

pub(crate) fn exec_detox(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            let (cy, cx) = vm.context_loc;
            crate::vm::iterate_circle(
                #[cfg(feature = "nova")]
                vm.topology,
                cx as i64,
                cy as i64,
                r,
                |tx, ty| {
                    vm.waste_grid[ty][tx] = 0;
                },
            );
            let r_sq = (r as i128).saturating_mul(r as i128);
            let cost = (r_sq + 1).clamp(5, 50) as i64;
            vm.energy = vm.energy.saturating_sub(cost);
            vm.output
                .push(format!("DETOX: Cleansed radius {} at {},{}", r, cx, cy));
        } else {
            vm.output.push("Error: Type mismatch for detox".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for detox".to_string());
    }
    None
}

pub fn exec_chemotaxis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(c) = val {
            let (cy, cx) = vm.context_loc;
            let channel_idx = (c.unsigned_abs() as usize) % 3;

            let mut max_intensity = -1;
            let mut best_dy = 0;
            let mut best_dx = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                        let intensity = vm.hormone_grid[ny][nx][channel_idx];
                        if intensity > max_intensity {
                            max_intensity = intensity;
                            best_dy = dy;
                            best_dx = dx;
                        }
                    }
                }
            }

            vm.stack.push(Value::Int(best_dy));
            vm.stack.push(Value::Int(best_dx));
            vm.energy = vm.energy.saturating_sub(5);
        } else {
            vm.output
                .push("Error: Type mismatch for chemotaxis".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for chemotaxis".to_string());
    }
    None
}

pub(crate) fn exec_irradiate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let r = vm.pop_int("irradiate")?;
    let amount = vm.pop_int("irradiate")?;

    if r > 0 && amount > 0 {
        let (cy, cx) = vm.context_loc;
        crate::vm::iterate_circle(
            #[cfg(feature = "nova")]
            vm.topology,
            cx as i64,
            cy as i64,
            r,
            |tx, ty| {
                vm.mutagen_grid[ty][tx] = vm.mutagen_grid[ty][tx].saturating_add(amount);
            },
        );
        let r_sq = (r as i128).saturating_mul(r as i128);
        let cost = (r_sq + 1).clamp(5, 50) as i64 + amount / 10;
        vm.energy = vm.energy.saturating_sub(cost);
        vm.output.push(format!(
            "IRRADIATE: Added {} mutagen at {},{} r={}",
            amount, cx, cy, r
        ));
    }
    None
}

pub fn exec_sense_mutagen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.mutagen_grid[cy][cx];
    vm.stack.push(Value::Int(level));
    None
}

pub fn exec_devour(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.mutagen_grid[cy][cx];
    if level > 0 {
        vm.mutagen_grid[cy][cx] = 0;
        let energy_gain = level / 2;
        vm.energy = vm.energy.saturating_add(energy_gain);
        vm.stack.push(Value::Int(energy_gain));
        vm.output.push(format!(
            "DEVOUR: Consumed {} mutagen, gained {} energy",
            level, energy_gain
        ));
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

pub fn exec_spirit(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(msg) = val {
            vm.spirit_message = Some(msg);
        } else {
            vm.stack.push(val);
            vm.spirit_message = None;
        }
    } else {
        vm.spirit_message = None;
    }
    vm.spirit_request = true;
    None
}

pub fn exec_match(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let target_val = vm.stack.pop().unwrap();
        let pattern_val = vm.stack.pop().unwrap();

        if let (Value::Str(p), Value::Str(t)) = (pattern_val, target_val) {
            // Re-implement glob_match here or make it public in nova.rs?
            // glob_match is small, I can copy it here or make it a utility.
            // I'll copy it for now to decouple.
            let is_match = glob_match(&p, &t);
            vm.stack.push(Value::Int(if is_match { 1 } else { 0 }));
            vm.energy = vm.energy.saturating_sub(5);
        } else {
            vm.output.push("Error: Type mismatch for match".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for match".to_string());
    }
    None
}

fn glob_match(pattern: &str, target: &str) -> bool {
    if let Some((p_head, p_tail)) = pattern.split_once('*') {
        if !target.starts_with(p_head) {
            return false;
        }
        let t_rest = &target[p_head.len()..];
        if p_tail.is_empty() {
            return true;
        }

        for i in 0..=t_rest.len() {
            if t_rest.is_char_boundary(i) && glob_match(p_tail, &t_rest[i..]) {
                return true;
            }
        }
        false
    } else {
        pattern == target
    }
}

pub fn exec_bury(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    if vm.graveyard.len() >= crate::vm::MAX_GRAVEYARD_SIZE {
                        vm.output.push("BURY: Graveyard limit reached".to_string());
                        return None;
                    }

                    let strand = vm.dna.helix.strands[s_idx].clone();
                    vm.graveyard.push(strand);

                    vm.dna.helix.strands[s_idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != s_idx);

                    vm.cladistics.kill_strand(s_idx, vm.tick_counter);

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("BURY: Buried strand {} in graveyard", s_idx));
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for bury".to_string());
                }
            }
            _ => vm.output.push("Error: Type mismatch for bury".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for bury".to_string());
    }
    None
}

pub fn exec_exhume(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(strand) = vm.graveyard.pop() {
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        let new_idx = vm.dna.helix.strands.len() - 1;

        vm.cladistics.register_strand(
            new_idx,
            Some(vm.ip.0),
            vm.tick_counter,
            "Exhume".to_string(),
        );

        vm.stack.push(Value::Int(new_idx as i64));
        vm.energy = vm.energy.saturating_sub(25);
        vm.output
            .push(format!("EXHUME: Resurrected strand as {}", new_idx));
    } else {
        vm.stack.push(Value::Int(-1));
        vm.output.push("EXHUME: Graveyard empty".to_string());
    }
    None
}

pub fn exec_seance(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(strand) = vm.graveyard.last() {
        let ghost_strand = strand.clone();
        crate::vm::nova_simulation::execute_ephemeral_strand(vm, &ghost_strand);
        vm.energy = vm.energy.saturating_sub(15);
        vm.output.push("SEANCE: Communed with the dead".to_string());
    } else {
        vm.output.push("SEANCE: Graveyard empty".to_string());
    }
    None
}

pub fn exec_mourn(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let count = vm.graveyard.len() as i64;
    let energy_gain = count * 2;
    vm.energy = vm.energy.saturating_add(energy_gain);
    vm.stack.push(Value::Int(energy_gain));
    vm.output.push(format!(
        "MOURN: Gained {} energy from {} ancestors",
        energy_gain, count
    ));
    None
}

pub fn exec_reincarnate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    let mut new_strand = vm.dna.helix.strands[s_idx].clone();
                    let mut rng = rand::thread_rng();

                    let enzymes = [
                        OpCode::Push,
                        OpCode::Add,
                        OpCode::Sub,
                        OpCode::Mul,
                        OpCode::Div,
                        OpCode::Dup,
                        OpCode::Print,
                        OpCode::Swap,
                        OpCode::Drop,
                        OpCode::Jump,
                        OpCode::Brz,
                        OpCode::Photosynthesize,
                        OpCode::Consume,
                        OpCode::GRead,
                        OpCode::GWrite,
                        OpCode::Genome,
                        OpCode::Meme,
                        OpCode::Poly,
                    ];

                    for gene in &mut new_strand.genes {
                        if rng.gen_bool(0.1) {
                            let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                            gene.op = new_op;
                        }
                    }

                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for reincarnate".to_string());
                        return None;
                    }
                    vm.dna.helix.strands.push(new_strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    vm.cladistics.register_strand(
                        new_idx,
                        Some(s_idx),
                        vm.tick_counter,
                        "Reincarnate".to_string(),
                    );

                    vm.dna.helix.strands[s_idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != s_idx);

                    vm.cladistics.kill_strand(s_idx, vm.tick_counter);

                    vm.stack.push(Value::Int(new_idx as i64));

                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output.push(format!(
                        "REINCARNATE: Strand {} reborn as {}",
                        s_idx, new_idx
                    ));
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for reincarnate".to_string());
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for reincarnate".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for reincarnate".to_string());
    }
    None
}
