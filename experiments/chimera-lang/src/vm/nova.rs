#![cfg(feature = "nova")]
use super::ChimeraVM;
use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::Value;
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a "time-travel" snapshot of the VM state.
///
/// Used by the `Sporulate` and `Germinate` opcodes to save and restore the entire simulation state.
#[cfg(feature = "nova")]
#[derive(Clone)]
pub struct Spore {
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize),
    pub output: Vec<String>,
    pub halted: bool,
    pub energy: i64,
    pub grid: Vec<Vec<Value>>,
    pub chaos_mode: bool,
    pub recursion_depth: usize,
    pub context_loc: (usize, usize),
    pub epigenome: HashSet<(usize, usize)>,
    pub telomeres: Vec<i64>,
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    pub waste_grid: Vec<Vec<i64>>,
    pub light_grid: Vec<Vec<i64>>,
    pub call_stack: Vec<(usize, usize)>,
    pub input_buffer: VecDeque<char>,
    pub receptors: HashMap<char, usize>,
    pub entangled_pairs: HashMap<usize, usize>,
    pub portals: HashMap<(usize, usize), (usize, usize)>,
    pub membranes: Vec<Vec<u8>>,
    pub sonar_target: Option<(usize, usize)>,
    pub symbiotes: Vec<(usize, usize)>,
    pub ether: HashMap<i64, VecDeque<Value>>,
    pub reflex_map: HashMap<i64, usize>,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq)]
pub enum OrganelleType {
    Worker,       // Standard execution
    Chloroplast,  // Generates energy from light
    Mitochondria, // Reduces metabolic cost / generates base energy
    Lysosome,     // Consumes waste to produce energy
    Ribosome,     // Executes grid instructions
}

/// An independent execution unit spawned by the main strand.
#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct Organelle {
    pub stack: Vec<Value>,
    pub ip: (usize, usize),
    pub context_loc: (usize, usize),
    pub call_stack: Vec<(usize, usize)>,
    pub recursion_depth: usize,
    pub halted: bool,
    pub kind: OrganelleType,
    pub direction: (i8, i8),
}

#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_hormones(vm: &mut ChimeraVM) {
    let mut buffer = [[[0i64; 3]; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            for c in 0..3 {
                let mut sum = (vm.hormone_grid[y][x][c] as i128) * 4;
                let mut count = 4;

                for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let mut blocked = false;
                    if let Some(mask) = get_direction_mask(dy, dx) {
                        if (vm.membranes[y][x] & mask) != 0 {
                            blocked = true;
                        }
                    }

                    if !blocked {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            sum += vm.hormone_grid[ny][nx][c] as i128;
                            count += 1;
                        }
                    }
                }

                buffer[y][x][c] = (sum / count) as i64;
            }
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.hormone_grid[y][x] = buffer[y][x];
        }
    }
}

#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_waste(vm: &mut ChimeraVM) {
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let mut sum = (vm.waste_grid[y][x] as i128) * 4;
            let mut count = 4;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut blocked = false;
                if let Some(mask) = get_direction_mask(dy, dx) {
                    if (vm.membranes[y][x] & mask) != 0 {
                        blocked = true;
                    }
                }

                if !blocked {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        sum += vm.waste_grid[ny][nx] as i128;
                        count += 1;
                    }
                }
            }

            buffer[y][x] = (sum / count) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.waste_grid[y][x] = buffer[y][x];
        }
    }
}

#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_light(vm: &mut ChimeraVM) {
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let mut sum = (vm.light_grid[y][x] as i128) * 4;
            let mut count = 4;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut blocked = false;
                if let Some(mask) = get_direction_mask(dy, dx) {
                    if (vm.membranes[y][x] & mask) != 0 {
                        blocked = true;
                    }
                }

                if !blocked {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        sum += vm.light_grid[ny][nx] as i128;
                        count += 1;
                    }
                }
            }

            // Blur and strong decay (50%)
            buffer[y][x] = ((sum / count) / 2) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.light_grid[y][x] = buffer[y][x];
        }
    }
}
#[cfg(feature = "nova")]
pub fn exec_nova_op(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        #[cfg(feature = "nova")]
        OpCode::Simulate => {
            // stack: ticks, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let ticks_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Int(ticks), Value::Int(s_idx)) = (ticks_val, s_val) {
                    let idx = s_idx as usize;
                    if idx < vm.dna.helix.strands.len() && ticks > 0 {
                        if vm.recursion_depth > 100 {
                            vm.output
                                .push("Error: Recursion limit exceeded".to_string());
                            return None;
                        }

                        // Cap ticks to prevent DoS
                        let safe_ticks = ticks.min(1000);

                        // Fork VM
                        // Cloning `vm` clones everything, which provides an accurate snapshot.
                        let mut sim_vm = vm.clone();

                        // Setup simulation context
                        sim_vm.ip = (idx, 0);
                        sim_vm.output.clear(); // Silence output
                        sim_vm.halted = false;

                        // Run simulation loop
                        for _ in 0..safe_ticks {
                            sim_vm.step();
                            if sim_vm.halted {
                                break;
                            }
                        }

                        // Collect Results
                        // 1. Top of stack (or 0 if empty)
                        let top_val = sim_vm.stack.last().cloned().unwrap_or(Value::Int(0));
                        // 2. Final Energy
                        let energy = sim_vm.energy;
                        // 3. Status (1 = Alive, 0 = Halted/Dead)
                        let status = if sim_vm.halted { 0 } else { 1 };

                        // Push results to original VM stack
                        vm.stack.push(top_val);
                        vm.stack.push(Value::Int(energy));
                        vm.stack.push(Value::Int(status));

                        // Deduct Energy Cost: Base cost + duration cost
                        let cost = safe_ticks.saturating_add(50);
                        vm.energy = vm.energy.saturating_sub(cost);

                        vm.output.push(format!(
                            "SIMULATE: Ran strand {} for {} ticks. Status: {}",
                            idx, safe_ticks, status
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid args for simulate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for simulate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for simulate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Spawn => {
            // stack: type, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let type_val = vm.stack.pop().unwrap();
                let idx_val = vm.stack.pop().unwrap();

                if let (Value::Int(t), Value::Int(idx)) = (type_val, idx_val) {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let (kind, direction) = match t {
                            1 => (OrganelleType::Chloroplast, (0, 0)),
                            2 => (OrganelleType::Mitochondria, (0, 0)),
                            3 => (OrganelleType::Lysosome, (0, 0)),
                            4 => (OrganelleType::Ribosome, (0, 1)), // Default East
                            _ => (OrganelleType::Worker, (0, 0)),
                        };

                        let organelle = Organelle {
                            stack: Vec::new(),
                            ip: (s_idx, 0),
                            context_loc: vm.context_loc,
                            call_stack: Vec::new(),
                            recursion_depth: 0,
                            halted: false,
                            kind: kind.clone(),
                            direction,
                        };
                        vm.organelles.push(organelle);
                        vm.energy = vm.energy.saturating_sub(20);
                        vm.output.push(format!(
                            "SPAWN: Created {:?} Organelle executing strand {}",
                            kind, s_idx
                        ));
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for spawn".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for spawn".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for spawn".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Sporulate => {
            // Create snapshot
            let spore = Spore {
                dna: vm.dna.clone(),
                stack: vm.stack.clone(),
                ip: vm.ip,
                output: vm.output.clone(),
                halted: vm.halted,
                energy: vm.energy,
                grid: vm.grid.clone(),
                chaos_mode: vm.chaos_mode,
                recursion_depth: vm.recursion_depth,
                context_loc: vm.context_loc,
                epigenome: vm.epigenome.clone(),
                telomeres: vm.telomeres.clone(),
                hormone_grid: vm.hormone_grid.clone(),
                waste_grid: vm.waste_grid.clone(),
                light_grid: vm.light_grid.clone(),
                call_stack: vm.call_stack.clone(),
                input_buffer: vm.input_buffer.clone(),
                receptors: vm.receptors.clone(),
                entangled_pairs: vm.entangled_pairs.clone(),
                portals: vm.portals.clone(),
                membranes: vm.membranes.clone(),
                sonar_target: vm.sonar_target,
                symbiotes: vm.symbiotes.clone(),
                ether: vm.ether.clone(),
                reflex_map: vm.reflex_map.clone(),
                #[cfg(feature = "cortex")]
                synapse_map: vm.synapse_map.clone(),
                #[cfg(feature = "cortex")]
                activation_levels: vm.activation_levels.clone(),
            };

            let id = vm.spores.len();
            vm.spores.push(spore);
            vm.stack.push(Value::Int(id as i64));
            vm.energy = vm.energy.saturating_sub(50); // High cost for time travel
            vm.output.push(format!("SPORULATE: Created Spore {}", id));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Germinate => {
            // stack: spore_id
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(id) = val {
                    let idx = id as usize;
                    if idx < vm.spores.len() {
                        let spore = &vm.spores[idx];
                        // Restore state
                        vm.dna = spore.dna.clone();
                        vm.stack = spore.stack.clone();
                        vm.ip = spore.ip;
                        vm.output = spore.output.clone();
                        vm.halted = spore.halted;
                        vm.energy = spore.energy;
                        vm.grid = spore.grid.clone();
                        vm.chaos_mode = spore.chaos_mode;
                        vm.recursion_depth = spore.recursion_depth;
                        vm.context_loc = spore.context_loc;
                        vm.epigenome = spore.epigenome.clone();
                        vm.telomeres = spore.telomeres.clone();
                        vm.hormone_grid = spore.hormone_grid.clone();
                        vm.waste_grid = spore.waste_grid.clone();
                        vm.light_grid = spore.light_grid.clone();
                        vm.call_stack = spore.call_stack.clone();
                        vm.input_buffer = spore.input_buffer.clone();
                        vm.receptors = spore.receptors.clone();
                        vm.entangled_pairs = spore.entangled_pairs.clone();
                        vm.portals = spore.portals.clone();
                        vm.membranes = spore.membranes.clone();
                        vm.sonar_target = spore.sonar_target;
                        vm.symbiotes = spore.symbiotes.clone();
                        vm.ether = spore.ether.clone();
                        vm.reflex_map = spore.reflex_map.clone();
                        #[cfg(feature = "cortex")]
                        {
                            vm.synapse_map = spore.synapse_map.clone();
                            vm.activation_levels = spore.activation_levels.clone();
                        }

                        vm.output.push(format!("GERMINATE: Restored Spore {}", idx));
                    } else {
                        vm.output
                            .push("Error: Spore index out of bounds".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for germinate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for germinate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Incubate => {
            // stack: len, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let len_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(len)) = (x_val, y_val, len_val) {
                    if len > 0 {
                        let mut genes = Vec::new();
                        let mut valid = true;
                        let max_len = len as usize;

                        // Read sequence from grid
                        let mut sequence = Vec::new();
                        for k in 0..max_len {
                            if let Some((ny, nx)) = vm.normalize_coords(y, x + k as i64) {
                                sequence.push(vm.grid[ny][nx].clone());
                            } else {
                                valid = false;
                                vm.output
                                    .push("Error: Incubate range out of bounds".to_string());
                                break;
                            }
                        }

                        if valid {
                            let mut k = 0;
                            while k < sequence.len() {
                                match &sequence[k] {
                                    Value::Junction(_, _) => k += 1, // Ignore junctions
                                    Value::Int(n) => {
                                        // Treated as push(n)
                                        genes.push(crate::ast::Gene {
                                            op: OpCode::Push,
                                            args: vec![crate::ast::Nucleotide::Number(*n)],
                                        });
                                        k += 1;
                                    }
                                    Value::Str(s) => {
                                        let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                        let mut args = Vec::new();
                                        match op {
                                            OpCode::Push
                                            | OpCode::Jump
                                            | OpCode::Brz
                                            | OpCode::Call
                                            | OpCode::Bind
                                            | OpCode::Unbind
                                            | OpCode::Decohere
                                            | OpCode::Telomerase
                                            | OpCode::Mitosis
                                            | OpCode::Apoptosis
                                            | OpCode::Cas9Cut
                                            | OpCode::Ligase
                                            | OpCode::Entangle => {
                                                #[cfg(feature = "cortex")]
                                                {
                                                    // Gate is handled by cortex feature, but we can't easily conditionally match inside match arm list
                                                    // So we'll just check if it's one of these.
                                                    // Actually, this match arm structure is tricky with cfg flags.
                                                    // Let's use if statements or just be permissive.
                                                }
                                                // 1 Arg
                                                if k + 1 < sequence.len() {
                                                    match &sequence[k + 1] {
                                                        Value::Int(n) => args.push(
                                                            crate::ast::Nucleotide::Number(*n),
                                                        ),
                                                        Value::Str(ss) => args.push(
                                                            crate::ast::Nucleotide::String(
                                                                ss.clone(),
                                                            ),
                                                        ),
                                                        Value::Junction(_, _) => {}
                                                    }
                                                    k += 1; // Consume arg
                                                }
                                            }
                                            #[cfg(feature = "cortex")]
                                            OpCode::Gate
                                            | OpCode::Link
                                            | OpCode::Sever
                                            | OpCode::Spark => {
                                                if k + 1 < sequence.len() {
                                                    match &sequence[k + 1] {
                                                        Value::Int(n) => args.push(
                                                            crate::ast::Nucleotide::Number(*n),
                                                        ),
                                                        Value::Str(ss) => args.push(
                                                            crate::ast::Nucleotide::String(
                                                                ss.clone(),
                                                            ),
                                                        ),
                                                    }
                                                    k += 1; // Consume arg
                                                }
                                            }
                                            _ => {}
                                        }

                                        genes.push(crate::ast::Gene { op, args });
                                        k += 1;
                                    }
                                }
                            }

                            vm.dna.helix.strands.push(crate::ast::Strand { genes });
                            vm.telomeres.push(50);
                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }
                            vm.energy = vm.energy.saturating_sub(20); // Cost
                            vm.output.push(format!(
                                "INCUBATE: Created new strand {} from grid",
                                vm.dna.helix.strands.len() - 1
                            ));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid length for incubate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for incubate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for incubate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Methylate => {
            if vm.stack.len() >= 2 {
                let gene_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();
                if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                    vm.epigenome.insert((s_idx as usize, g_idx as usize));
                    vm.output.push(format!("METHYLATED: {}:{}", s_idx, g_idx));
                } else {
                    vm.output
                        .push("Error: Invalid args for methylate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for methylate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Demethylate => {
            if vm.stack.len() >= 2 {
                let gene_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();
                if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                    vm.epigenome.remove(&(s_idx as usize, g_idx as usize));
                    vm.output.push(format!("DEMETHYLATED: {}:{}", s_idx, g_idx));
                } else {
                    vm.output
                        .push("Error: Invalid args for demethylate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for demethylate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Telomerase => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(amount) => {
                        if amount > 0 {
                            let idx = vm.ip.0;
                            if idx < vm.telomeres.len() {
                                vm.telomeres[idx] = vm.telomeres[idx].saturating_add(amount);
                                vm.energy = vm.energy.saturating_sub(25); // High cost
                                vm.output.push(format!(
                                    "TELOMERASE: Extended strand {} by {}",
                                    idx, amount
                                ));
                            }
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Invalid arg for telomerase".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for telomerase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::TLen => {
            let idx = vm.ip.0;
            if idx < vm.telomeres.len() {
                vm.stack.push(Value::Int(vm.telomeres[idx]));
            } else {
                vm.stack.push(Value::Int(0));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Recombine => {
            // stack: split_point, strand_b, strand_a (bottom)
            if vm.stack.len() >= 3 {
                let split_val = vm.stack.pop().unwrap();
                let strand_b_val = vm.stack.pop().unwrap();
                let strand_a_val = vm.stack.pop().unwrap();

                match (strand_a_val, strand_b_val, split_val) {
                    (Value::Int(sa), Value::Int(sb), Value::Int(split)) => {
                        let helix_len = vm.dna.helix.strands.len();
                        let sa_idx = sa as usize;
                        let sb_idx = sb as usize;
                        let split_idx = split as usize;

                        if sa >= 0
                            && sb >= 0
                            && split >= 0
                            && sa_idx < helix_len
                            && sb_idx < helix_len
                        {
                            // We need to check split bounds for both strands
                            let len_a = vm.dna.helix.strands[sa_idx].genes.len();
                            let len_b = vm.dna.helix.strands[sb_idx].genes.len();

                            if split_idx <= len_a && split_idx <= len_b {
                                // Perform recombination
                                // We need to borrow strands mutably.
                                // Since they are in the same Vec, we need split_at_mut or similar trickery,
                                // or just use indices if we can modify the Vec safely.
                                // We can't get two mutable references to the same Vec at different indices directly.
                                // So we'll use `split_at_mut` if they are different indices, or just do nothing if same.

                                if sa_idx == sb_idx {
                                    // Recombining same strand with itself at same point is a no-op.
                                    vm.output.push(
                                        "Warning: Recombining strand with itself".to_string(),
                                    );
                                } else {
                                    // Ensure ordered access to avoid panic
                                    let (lower, upper) = if sa_idx < sb_idx {
                                        (sa_idx, sb_idx)
                                    } else {
                                        (sb_idx, sa_idx)
                                    };

                                    let (first_slice, second_slice) =
                                        vm.dna.helix.strands.split_at_mut(upper);
                                    let strand_low = &mut first_slice[lower];
                                    let strand_high = &mut second_slice[0]; // relative index 0 is absolute 'upper'

                                    // Identify which is A and B
                                    let (strand_a, strand_b) = if sa_idx < sb_idx {
                                        (strand_low, strand_high)
                                    } else {
                                        (strand_high, strand_low)
                                    };

                                    let mut tail_a = strand_a.genes.split_off(split_idx);
                                    let mut tail_b = strand_b.genes.split_off(split_idx);

                                    strand_a.genes.append(&mut tail_b);
                                    strand_b.genes.append(&mut tail_a);

                                    vm.output.push(format!(
                                        "RECOMBINATION: Swapped tails of strand {} and {} at {}",
                                        sa, sb, split
                                    ));
                                }
                            } else {
                                vm.output
                                    .push("Error: Split point out of bounds".to_string());
                            }
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for recombine".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for recombine".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SIndex => {
            vm.stack.push(Value::Int(vm.ip.0 as i64));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::CrisprScan => {
            // stack: guide_idx, target_idx (bottom)
            if vm.stack.len() >= 2 {
                let guide_val = vm.stack.pop().unwrap();
                let target_val = vm.stack.pop().unwrap();

                if let (Value::Int(g_idx), Value::Int(t_idx)) = (guide_val, target_val) {
                    let g_idx = g_idx as usize;
                    let t_idx = t_idx as usize;
                    let helix_len = vm.dna.helix.strands.len();

                    if g_idx < helix_len && t_idx < helix_len {
                        let guide_strand = &vm.dna.helix.strands[g_idx];
                        let target_strand = &vm.dna.helix.strands[t_idx];

                        // We need to match sequence of gene names
                        // Use op.to_string() for comparison
                        let guide_names: Vec<String> = guide_strand
                            .genes
                            .iter()
                            .map(|g| g.op.to_string())
                            .collect();
                        let target_names: Vec<String> = target_strand
                            .genes
                            .iter()
                            .map(|g| g.op.to_string())
                            .collect();

                        let mut found_idx: i64 = -1;

                        if !guide_names.is_empty() && guide_names.len() <= target_names.len() {
                            for i in 0..=(target_names.len() - guide_names.len()) {
                                if target_names[i..i + guide_names.len()] == guide_names[..] {
                                    found_idx = i as i64;
                                    break;
                                }
                            }
                        } else if guide_names.is_empty() {
                            // Explicitly handle empty guide to ensure -1 (though init value handles it)
                            found_idx = -1;
                        }

                        vm.stack.push(Value::Int(found_idx));
                        vm.energy = vm.energy.saturating_sub(5);
                        vm.output.push(format!(
                            "CRISPR_SCAN: Scanned strand {} for pattern from {} -> {}",
                            t_idx, g_idx, found_idx
                        ));
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for crispr_scan".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for crispr_scan".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for crispr_scan".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Cas9Cut => {
            // stack: cut_index, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let cut_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();

                if let (Value::Int(cut), Value::Int(s_idx)) = (cut_val, strand_val) {
                    let s_idx = s_idx as usize;
                    let cut_idx = cut as usize;
                    let helix_len = vm.dna.helix.strands.len();

                    if s_idx < helix_len {
                        let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                        if cut >= 0 && cut_idx <= strand_len {
                            // Perform split
                            // We need to mutate the strand.
                            let strand = &mut vm.dna.helix.strands[s_idx];
                            let tail_genes = strand.genes.split_off(cut_idx);

                            // Create new strand
                            vm.dna
                                .helix
                                .strands
                                .push(crate::ast::Strand { genes: tail_genes });
                            vm.telomeres.push(50);

                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }

                            let new_strand_idx = vm.dna.helix.strands.len() - 1;

                            vm.stack.push(Value::Int(new_strand_idx as i64));
                            vm.energy = vm.energy.saturating_sub(10);

                            vm.output.push(format!(
                                "CAS9_CUT: Cut strand {} at {}, created strand {}",
                                s_idx, cut_idx, new_strand_idx
                            ));

                            // Handling Self-Modification Safety:
                            // If we cut the strand we are currently executing,
                            // and the cut point is BEFORE or AT our current IP, execution context moves.
                            if s_idx == vm.ip.0 && vm.ip.1 >= cut_idx {
                                let new_gene_idx = vm.ip.1 - cut_idx;
                                // We jump to the next instruction in the NEW strand
                                return Some((new_strand_idx, new_gene_idx + 1));
                            }
                        } else {
                            vm.output.push("Error: Cut index out of bounds".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for cas9_cut".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for cas9_cut".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for cas9_cut".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Ligase => {
            // stack: donor_idx, recipient_idx (bottom)
            if vm.stack.len() >= 2 {
                let donor_val = vm.stack.pop().unwrap();
                let recipient_val = vm.stack.pop().unwrap();

                if let (Value::Int(d_idx), Value::Int(r_idx)) = (donor_val, recipient_val) {
                    let d_idx = d_idx as usize;
                    let r_idx = r_idx as usize;
                    let helix_len = vm.dna.helix.strands.len();

                    if d_idx < helix_len && r_idx < helix_len {
                        if d_idx == r_idx {
                            vm.output
                                .push("Warning: Ligase on same strand is no-op".to_string());
                        } else {
                            // We need to move genes from donor to recipient.
                            let (lower, upper) = if d_idx < r_idx {
                                (d_idx, r_idx)
                            } else {
                                (r_idx, d_idx)
                            };

                            let (first_slice, second_slice) =
                                vm.dna.helix.strands.split_at_mut(upper);
                            let strand_low = &mut first_slice[lower];
                            let strand_high = &mut second_slice[0];

                            let (strand_d, strand_r) = if d_idx < r_idx {
                                (strand_low, strand_high)
                            } else {
                                (strand_high, strand_low)
                            };

                            strand_r.genes.append(&mut strand_d.genes);
                            // donor genes are now empty.

                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("LIGASE: Appended strand {} to {}", d_idx, r_idx));
                        }
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for ligase".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for ligase".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ligase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Mitosis => {
            // stack: strand_idx (target to clone)
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            // Clone the strand
                            let new_strand = vm.dna.helix.strands[s_idx].clone();
                            vm.dna.helix.strands.push(new_strand);
                            vm.telomeres.push(50); // Default life

                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }

                            // Inherit epigenetics
                            // We need to find all keys (s_idx, g_idx) and insert (new_idx, g_idx)
                            let new_s_idx = vm.dna.helix.strands.len() - 1;
                            let genes_to_methylate: Vec<usize> = vm
                                .epigenome
                                .iter()
                                .filter(|(s, _)| *s == s_idx)
                                .map(|(_, g)| *g)
                                .collect();

                            for g_idx in genes_to_methylate {
                                vm.epigenome.insert((new_s_idx, g_idx));
                            }

                            vm.energy = vm.energy.saturating_sub(30); // Cost
                            vm.output
                                .push(format!("MITOSIS: Cloned strand {} to {}", s_idx, new_s_idx));
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds for mitosis".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for mitosis".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for mitosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Apoptosis => {
            // stack: strand_idx
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            vm.dna.helix.strands[s_idx].genes.clear();

                            // Remove associated epigenetics
                            vm.epigenome.retain(|(s, _)| *s != s_idx);

                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("APOPTOSIS: Cleared strand {}", s_idx));
                        } else {
                            vm.output.push(
                                "Error: Strand index out of bounds for apoptosis".to_string(),
                            );
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for apoptosis".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for apoptosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Integrase => {
            // stack: arg, name, gene_idx, strand_idx (bottom)
            if vm.stack.len() >= 4 {
                let arg_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();
                let gene_idx_val = vm.stack.pop().unwrap();
                let strand_idx_val = vm.stack.pop().unwrap();

                match (strand_idx_val, gene_idx_val, name_val, arg_val) {
                    (Value::Int(s), Value::Int(g), Value::Str(name), arg) => {
                        let s_idx = s as usize;
                        let g_idx = g as usize;
                        let helix_len = vm.dna.helix.strands.len();

                        if s >= 0 && s_idx < helix_len {
                            let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                            if g >= 0 && g_idx <= strand_len {
                                // Create Gene
                                let new_gene = crate::ast::Gene {
                                    op: name.parse().unwrap_or(OpCode::Unknown(name.clone())),
                                    args: match arg {
                                        Value::Int(n) => {
                                            vec![crate::ast::Nucleotide::Number(n)]
                                        }
                                        Value::Str(s) => {
                                            vec![crate::ast::Nucleotide::String(s)]
                                        }
                                        Value::Junction(_, _) => vec![],
                                    },
                                };

                                // Insert
                                vm.dna.helix.strands[s_idx].genes.insert(g_idx, new_gene);

                                // Update Epigenome: Shift all markers at (s_idx, k >= g_idx) to k+1
                                let mut new_markers = Vec::new();
                                let mut to_remove = Vec::new();
                                for &(ms, mg) in vm.epigenome.iter() {
                                    if ms == s_idx && mg >= g_idx {
                                        to_remove.push((ms, mg));
                                        new_markers.push((ms, mg + 1));
                                    }
                                }
                                for marker in to_remove {
                                    vm.epigenome.remove(&marker);
                                }
                                for marker in new_markers {
                                    vm.epigenome.insert(marker);
                                }

                                vm.energy = vm.energy.saturating_sub(20);
                                vm.output
                                    .push(format!("INTEGRASE: Inserted {} at {}:{}", name, s, g));

                                // Update IP if we inserted before or at current execution
                                if vm.ip.0 == s_idx && vm.ip.1 >= g_idx {
                                    vm.ip.1 += 1;
                                }
                                // Default None means step() will increment IP +1.
                                // If we shifted IP +1 here, total is +2.
                                // This skips the inserted gene (if at g_idx) and the current gene (now at g_idx+1).
                                // Correct.
                            } else {
                                vm.output.push(
                                    "Error: Gene index out of bounds for integrase".to_string(),
                                );
                            }
                        } else {
                            vm.output.push(
                                "Error: Strand index out of bounds for integrase".to_string(),
                            );
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for integrase".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for integrase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Excision => {
            // stack: gene_idx, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let gene_idx_val = vm.stack.pop().unwrap();
                let strand_idx_val = vm.stack.pop().unwrap();

                match (strand_idx_val, gene_idx_val) {
                    (Value::Int(s), Value::Int(g)) => {
                        let s_idx = s as usize;
                        let g_idx = g as usize;
                        let helix_len = vm.dna.helix.strands.len();

                        if s >= 0 && s_idx < helix_len {
                            let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                            if g >= 0 && g_idx < strand_len {
                                // Remove
                                vm.dna.helix.strands[s_idx].genes.remove(g_idx);

                                // Update Epigenome: Remove marker at g_idx, shift k > g_idx to k-1
                                vm.epigenome.remove(&(s_idx, g_idx));
                                let mut new_markers = Vec::new();
                                let mut to_remove = Vec::new();
                                for &(ms, mg) in vm.epigenome.iter() {
                                    if ms == s_idx && mg > g_idx {
                                        to_remove.push((ms, mg));
                                        new_markers.push((ms, mg - 1));
                                    }
                                }
                                for marker in to_remove {
                                    vm.epigenome.remove(&marker);
                                }
                                for marker in new_markers {
                                    vm.epigenome.insert(marker);
                                }

                                vm.energy = vm.energy.saturating_sub(15);
                                vm.output.push(format!("EXCISION: Removed {}:{}", s, g));

                                // Update IP
                                if vm.ip.0 == s_idx {
                                    if g_idx < vm.ip.1 {
                                        // Removed before current. Shift IP left.
                                        vm.ip.1 -= 1;
                                        return None; // step() increments +1. Net 0 change (but content moved left, so we execute next).
                                    } else if g_idx == vm.ip.1 {
                                        // Removed current.
                                        // Next gene slid into current slot.
                                        // We want to execute it.
                                        // So return IP as is, prevent step() increment.
                                        return Some(vm.ip);
                                    }
                                }
                            } else {
                                vm.output.push(
                                    "Error: Gene index out of bounds for excision".to_string(),
                                );
                            }
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds for excision".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for excision".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for excision".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Secrete => {
            // stack: channel, amount (top)
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
        #[cfg(feature = "nova")]
        OpCode::Detect => {
            // stack: channel
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
        #[cfg(feature = "nova")]
        OpCode::Absorb => {
            // stack: channel, amount (top)
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
        #[cfg(feature = "nova")]
        OpCode::Migrate => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;

                    let mut blocked = false;
                    if let Some(mask) = get_direction_mask(dy, dx) {
                        if (vm.membranes[cy][cx] & mask) != 0 {
                            blocked = true;
                        }
                    }

                    if !blocked {
                        if let Some((mut new_y, mut new_x)) =
                            vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                        {
                            // Check for portal
                            if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
                                vm.output.push(format!(
                                    "PORTAL: Teleported from {},{} to {},{}",
                                    new_x, new_y, px, py
                                ));
                                new_y = py;
                                new_x = px;
                            }

                            vm.context_loc = (new_y, new_x);
                            vm.energy = vm.energy.saturating_sub(5);
                            vm.output
                                .push(format!("MIGRATE: moved to {},{}", new_x, new_y));
                        } else {
                            // Hit boundary
                            vm.energy = vm.energy.saturating_sub(2);
                            vm.output.push("MIGRATE: Blocked by boundary".to_string());
                            vm.trigger_reflex(0); // Collision Event
                        }
                    } else {
                        // Blocked by membrane
                        vm.energy = vm.energy.saturating_sub(2);
                        vm.output.push("MIGRATE: Blocked by membrane".to_string());
                        vm.trigger_reflex(0); // Collision Event
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for migrate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for migrate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Detox => {
            // stack: radius (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(r) = val {
                    let (cy, cx) = vm.context_loc;
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                    for (tx, ty) in coords {
                        vm.waste_grid[ty][tx] = 0;
                    }
                    vm.energy = vm.energy.saturating_sub((r * r + 1).clamp(5, 50)); // Cost proportional to area
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
        #[cfg(feature = "nova")]
        OpCode::WRead => {
            let (cy, cx) = vm.context_loc;
            let waste = vm.waste_grid[cy][cx];
            vm.stack.push(Value::Int(waste));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Call => {
            if let Some(Nucleotide::Number(idx)) = args.first() {
                let strand_idx = *idx as usize;
                if strand_idx < vm.dna.helix.strands.len() {
                    // Push return address (current strand, next gene)
                    // ip points to Call instruction. Step loop will increment it.
                    // But if we jump, step loop sets ip to target.
                    // So we need to push (ip.0, ip.1 + 1).
                    vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));
                    return Some((strand_idx, 0));
                } else {
                    vm.output
                        .push("Error: Invalid strand index for call".to_string());
                }
            } else {
                vm.output.push("Error: Invalid arg for call".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Ret => {
            if let Some(ret_addr) = vm.call_stack.pop() {
                return Some(ret_addr);
            } else {
                vm.output
                    .push("Warning: Return with empty stack".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Bind => {
            // stack: strand_idx (top), char_code
            if vm.stack.len() >= 2 {
                let s_val = vm.stack.pop().unwrap();
                let c_val = vm.stack.pop().unwrap();
                if let (Value::Int(s), Value::Int(c)) = (s_val, c_val) {
                    let strand_idx = s as usize;
                    let key = (c as u8) as char;
                    if strand_idx < vm.dna.helix.strands.len() {
                        vm.receptors.insert(key, strand_idx);
                        vm.output
                            .push(format!("BIND: '{}' -> Strand {}", key, strand_idx));
                    } else {
                        vm.output
                            .push("Error: Invalid strand index for bind".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for bind".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for bind".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Unbind => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(c) = val {
                    let key = (c as u8) as char;
                    vm.receptors.remove(&key);
                    vm.output.push(format!("UNBIND: '{}'", key));
                } else {
                    vm.output
                        .push("Error: Type mismatch for unbind".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for unbind".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Entangle => {
            if vm.stack.len() >= 2 {
                let s_val2 = vm.stack.pop().unwrap();
                let s_val1 = vm.stack.pop().unwrap();
                if let (Value::Int(s1), Value::Int(s2)) = (s_val1, s_val2) {
                    let idx1 = s1 as usize;
                    let idx2 = s2 as usize;
                    let len = vm.dna.helix.strands.len();
                    if idx1 < len && idx2 < len {
                        if idx1 != idx2 {
                            // Remove existing links
                            if let Some(old) = vm.entangled_pairs.remove(&idx1) {
                                vm.entangled_pairs.remove(&old);
                            }
                            if let Some(old) = vm.entangled_pairs.remove(&idx2) {
                                vm.entangled_pairs.remove(&old);
                            }

                            // Add new links
                            vm.entangled_pairs.insert(idx1, idx2);
                            vm.entangled_pairs.insert(idx2, idx1);
                            vm.output.push(format!("ENTANGLE: {} <-> {}", idx1, idx2));
                        } else {
                            vm.output
                                .push("Warning: Cannot entangle strand with itself".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for entangle".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for entangle".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for entangle".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Decohere => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(s) = val {
                    let idx = s as usize;
                    if let Some(partner) = vm.entangled_pairs.remove(&idx) {
                        vm.entangled_pairs.remove(&partner);
                        vm.output
                            .push(format!("DECOHERE: Broken link {} <-> {}", idx, partner));
                    } else {
                        vm.output
                            .push(format!("DECOHERE: No link found for {}", idx));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for decohere".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for decohere".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Reflex => {
            // stack: event_id, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let strand_val = vm.stack.pop().unwrap();
                let event_val = vm.stack.pop().unwrap();

                if let (Value::Int(event), Value::Int(strand)) = (event_val, strand_val) {
                    let s_idx = strand as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        vm.reflex_map.insert(event, s_idx);
                        vm.output.push(format!(
                            "REFLEX: Registered handler for Event {} -> Strand {}",
                            event, s_idx
                        ));
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for reflex".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for reflex".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for reflex".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Conjugate => {
            // stack: direction (0=R, 1=D, 2=L, 3=U), y, x, strand_idx (bottom)
            if vm.stack.len() >= 4 {
                let dir_val = vm.stack.pop().unwrap();
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Int(s), Value::Int(y), Value::Int(x), Value::Int(dir)) =
                    (s_val, y_val, x_val, dir_val)
                {
                    let s_idx = s as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];
                        let mut curr_x = x;
                        let mut curr_y = y;
                        let (dx, dy) = match dir.rem_euclid(4) {
                            0 => (1, 0),
                            1 => (0, 1),
                            2 => (-1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        let mut success_count = 0;
                        let mut cells_to_write = Vec::new();

                        for gene in &strand.genes {
                            cells_to_write.push(Value::Str(gene.op.to_string()));
                            for arg in &gene.args {
                                match arg {
                                    crate::ast::Nucleotide::Number(n) => {
                                        cells_to_write.push(Value::Int(*n));
                                    }
                                    crate::ast::Nucleotide::String(s) => {
                                        cells_to_write.push(Value::Str(s.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }

                        for val in cells_to_write {
                            if let Some((ny, nx)) = vm.normalize_coords(curr_y, curr_x) {
                                vm.grid[ny][nx] = val;
                                success_count += 1;

                                // Advance
                                if let Some((next_y, next_x)) =
                                    vm.normalize_coords(ny as i64 + dy, nx as i64 + dx)
                                {
                                    curr_y = next_y as i64;
                                    curr_x = next_x as i64;
                                } else {
                                    // Hit wall, stop writing
                                    break;
                                }
                            } else {
                                break; // Start out of bounds
                            }
                        }

                        vm.energy -= success_count; // Cost 1 per cell
                        vm.output.push(format!(
                            "CONJUGATE: Wrote {} cells from strand {} at {},{}",
                            success_count, s_idx, x, y
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid strand index for conjugate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for conjugate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for conjugate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Gravitate => {
            // stack: radius
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(r) = val {
                    if r > 0 {
                        let (cy, cx) = vm.context_loc;
                        // Get coordinates within radius
                        // Note: get_circular_coords uses Euclidean distance on Plane.
                        // For Torus, we should use toroidal distance, but for simplicity we keep it local.
                        // However, valid coordinates are returned.
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

                        // Calculate distances and sort
                        let mut coords_with_dist: Vec<((usize, usize), i64)> = coords
                            .into_iter()
                            .map(|(x, y)| {
                                let dx = x as i64 - cx as i64;
                                let dy = y as i64 - cy as i64;
                                // Squared distance is sufficient for sorting
                                ((x, y), dx * dx + dy * dy)
                            })
                            .collect();

                        // Sort by distance (ascending)
                        coords_with_dist.sort_by_key(|&(_, d)| d);

                        let mut moved_count = 0;

                        for ((tx, ty), dist_sq) in coords_with_dist {
                            if dist_sq == 0 {
                                continue; // Skip center
                            }

                            // If empty, skip
                            if matches!(vm.grid[ty][tx], Value::Int(0)) {
                                continue;
                            }

                            // Calculate target (one step closer to center)
                            let dx = cx as i64 - tx as i64;
                            let dy = cy as i64 - ty as i64;

                            let sx = if dx > 0 {
                                1
                            } else if dx < 0 {
                                -1
                            } else {
                                0
                            };
                            let sy = if dy > 0 {
                                1
                            } else if dy < 0 {
                                -1
                            } else {
                                0
                            };

                            // Use normalize_coords to find valid target
                            if let Some((target_y, target_x)) =
                                vm.normalize_coords(ty as i64 + sy, tx as i64 + sx)
                            {
                                // Check if target is empty
                                if matches!(vm.grid[target_y][target_x], Value::Int(0)) {
                                    // Move
                                    vm.grid[target_y][target_x] = vm.grid[ty][tx].clone();
                                    vm.grid[ty][tx] = Value::Int(0);
                                    moved_count += 1;
                                }
                            }
                        }

                        vm.energy = vm.energy.saturating_sub(moved_count + 5); // Base cost + variable
                        vm.output.push(format!(
                            "GRAVITATE: Pulled {} items towards {},{}",
                            moved_count, cx, cy
                        ));
                    } else {
                        // Negative or zero radius is no-op
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for gravitate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for gravitate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Lumine => {
            // stack: intensity, radius (bottom)
            if vm.stack.len() >= 2 {
                let intensity_val = vm.stack.pop().unwrap();
                let radius_val = vm.stack.pop().unwrap();
                if let (Value::Int(r), Value::Int(intensity)) = (radius_val, intensity_val) {
                    if r > 0 && intensity > 0 {
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        let count = coords.len();
                        for (tx, ty) in coords {
                            vm.light_grid[ty][tx] = vm.light_grid[ty][tx].saturating_add(intensity);
                        }
                        vm.energy = vm.energy.saturating_sub((count / 2) as i64);
                        vm.output.push(format!(
                            "LUMINE: Emitted {} light at {},{} r={}",
                            intensity, cx, cy, r
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for lumine".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for lumine".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SenseLight => {
            let (cy, cx) = vm.context_loc;
            let intensity = vm.light_grid[cy][cx];
            vm.stack.push(Value::Int(intensity));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Dream => {
            // stack: ticks, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let ticks_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Int(ticks), Value::Int(s_idx)) = (ticks_val, s_val) {
                    let idx = s_idx as usize;
                    if idx < vm.dna.helix.strands.len() && ticks > 0 {
                        // Cap ticks
                        let safe_ticks = ticks.min(1000);

                        // Clone VM
                        let mut dream_vm = vm.clone();

                        // Force a mutation
                        dream_vm.mutate();

                        // Run simulation
                        dream_vm.ip = (idx, 0);
                        dream_vm.output.clear();
                        dream_vm.halted = false;

                        for _ in 0..safe_ticks {
                            dream_vm.step();
                            if dream_vm.halted {
                                break;
                            }
                        }

                        // Evaluate
                        let success = dream_vm.energy > vm.energy;

                        if success {
                            // Adopt DNA
                            vm.dna = dream_vm.dna;
                            vm.stack.push(Value::Int(1)); // Success
                            vm.output.push("DREAM: Mutation accepted".to_string());
                        } else {
                            vm.stack.push(Value::Int(0)); // Failure
                            vm.output.push("DREAM: Mutation discarded".to_string());
                        }

                        // Pay Cost (Base 50 + ticks/2)
                        let cost = 50 + (safe_ticks / 2);
                        vm.energy = vm.energy.saturating_sub(cost);
                    } else {
                        vm.output.push("Error: Invalid args for dream".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for dream".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for dream".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Chemotaxis => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(c) = val {
                    let (cy, cx) = vm.context_loc;
                    let channel_idx = (c.unsigned_abs() as usize) % 3;

                    let mut max_intensity = -1;
                    let mut best_dy = 0;
                    let mut best_dx = 0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if let Some((ny, nx)) =
                                vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
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
        #[cfg(feature = "nova")]
        OpCode::Identity => {
            let id = match &vm.active_organelle_kind {
                None => -1, // Nucleus
                Some(OrganelleType::Worker) => 0,
                Some(OrganelleType::Chloroplast) => 1,
                Some(OrganelleType::Mitochondria) => 2,
                Some(OrganelleType::Lysosome) => 3,
                Some(OrganelleType::Ribosome) => 4,
            };
            vm.stack.push(Value::Int(id));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Differentiate => {
            if let Some(val) = vm.stack.pop() {
                if vm.active_organelle_kind.is_some() {
                    if let Value::Int(t) = val {
                        let new_kind = match t {
                            1 => Some(OrganelleType::Chloroplast),
                            2 => Some(OrganelleType::Mitochondria),
                            3 => Some(OrganelleType::Lysosome),
                            4 => Some(OrganelleType::Ribosome),
                            _ => Some(OrganelleType::Worker), // 0 or others fallback to Worker
                        };

                        if let Some(kind) = new_kind {
                            vm.signal_differentiation = Some(kind.clone());
                            vm.energy = vm.energy.saturating_sub(50); // High cost to re-specialize
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
        #[cfg(feature = "nova")]
        OpCode::Shape => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(t) = val {
                    let new_topology = match t {
                        0 => Some(super::Topology::Plane),
                        1 => Some(super::Topology::Torus),
                        2 => Some(super::Topology::CylinderH),
                        3 => Some(super::Topology::CylinderV),
                        4 => Some(super::Topology::Klein),
                        5 => Some(super::Topology::Mobius),
                        _ => None,
                    };

                    if let Some(topo) = new_topology {
                        vm.topology = topo;
                        vm.output
                            .push(format!("SHAPE: Changed topology to {:?}", topo));
                        vm.energy = vm.energy.saturating_sub(100); // Massive energy cost to reshape reality
                    } else {
                        vm.output
                            .push(format!("Error: Invalid topology index {}", t));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for shape".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for shape".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Rift => {
            // stack: y2, x2, y1, x1 (bottom)
            if vm.stack.len() >= 4 {
                let x2_val = vm.stack.pop().unwrap();
                let y2_val = vm.stack.pop().unwrap();
                let x1_val = vm.stack.pop().unwrap();
                let y1_val = vm.stack.pop().unwrap();

                if let (Value::Int(x1), Value::Int(y1), Value::Int(x2), Value::Int(y2)) =
                    (x1_val, y1_val, x2_val, y2_val)
                {
                    if (0..16).contains(&x1)
                        && (0..16).contains(&y1)
                        && (0..16).contains(&x2)
                        && (0..16).contains(&y2)
                    {
                        vm.portals
                            .insert((y1 as usize, x1 as usize), (y2 as usize, x2 as usize));
                        vm.energy = vm.energy.saturating_sub(50);
                        vm.output.push(format!(
                            "RIFT: Opened portal from {},{} to {},{}",
                            x1, y1, x2, y2
                        ));
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for rift".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for rift".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for rift".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Seal => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if (0..16).contains(&x) && (0..16).contains(&y) {
                        if vm.portals.remove(&(y as usize, x as usize)).is_some() {
                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("SEAL: Closed portal at {},{}", x, y));
                        } else {
                            vm.output
                                .push(format!("SEAL: No portal found at {},{}", x, y));
                        }
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for seal".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for seal".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for seal".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Sonar => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    let mut found = false;

                    for d in 1..=16 {
                        if let Some((ny, nx)) =
                            vm.normalize_coords(cy as i64 + dy * d, cx as i64 + dx * d)
                        {
                            if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                                vm.stack.push(Value::Int(d));
                                vm.stack.push(vm.grid[ny][nx].clone());
                                vm.sonar_target = Some((ny, nx));
                                found = true;
                                break;
                            }
                        } else {
                            // Hit wall - stop scanning
                            break;
                        }
                    }

                    if !found {
                        vm.stack.push(Value::Int(16));
                        vm.stack.push(Value::Int(0));
                    }

                    vm.energy = vm.energy.saturating_sub(2);
                } else {
                    vm.output.push("Error: Type mismatch for sonar".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sonar".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Broadcast => {
            // stack: channel, value (top)
            if vm.stack.len() >= 2 {
                let value = vm.stack.pop().unwrap();
                let channel_val = vm.stack.pop().unwrap();
                if let Value::Int(channel) = channel_val {
                    let queue = vm.ether.entry(channel).or_default();
                    if queue.len() < 100 {
                        queue.push_back(value);
                        vm.energy = vm.energy.saturating_sub(1);
                        vm.output
                            .push(format!("BROADCAST: Sent to channel {}", channel));
                    } else {
                        vm.output
                            .push(format!("BROADCAST: Channel {} full", channel));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for broadcast".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for broadcast".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Tune => {
            // stack: channel
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(channel) = val {
                    let mut value = Value::Int(0);
                    if let Some(queue) = vm.ether.get_mut(&channel) {
                        if let Some(v) = queue.pop_front() {
                            value = v;
                            vm.output
                                .push(format!("TUNE: Received from channel {}", channel));
                        }
                    }
                    vm.stack.push(value);
                    vm.energy = vm.energy.saturating_sub(1);
                } else {
                    vm.output.push("Error: Type mismatch for tune".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for tune".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Membrane => {
            // stack: mask (1=N, 2=S, 4=E, 8=W)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(mask_val) = val {
                    let mask = mask_val as u8;
                    let (cy, cx) = vm.context_loc;

                    // Toggle bits
                    vm.membranes[cy][cx] ^= mask;

                    // Handle reciprocity
                    // North
                    if (mask & 1) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 - 1, cx as i64) {
                            vm.membranes[ny][nx] ^= 2;
                        }
                    }
                    // South
                    if (mask & 2) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + 1, cx as i64) {
                            vm.membranes[ny][nx] ^= 1;
                        }
                    }
                    // East
                    if (mask & 4) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 + 1) {
                            vm.membranes[ny][nx] ^= 8;
                        }
                    }
                    // West
                    if (mask & 8) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 - 1) {
                            vm.membranes[ny][nx] ^= 4;
                        }
                    }

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("MEMBRANE: Toggled mask {} at {},{}", mask, cx, cy));
                } else {
                    vm.output
                        .push("Error: Type mismatch for membrane".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for membrane".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Osmosis => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    if let Some((mut new_y, mut new_x)) =
                        vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                    {
                        // Check for portal
                        if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
                            vm.output.push(format!(
                                "PORTAL: Teleported from {},{} to {},{}",
                                new_x, new_y, px, py
                            ));
                            new_y = py;
                            new_x = px;
                        }

                        vm.context_loc = (new_y, new_x);
                        vm.energy = vm.energy.saturating_sub(20); // High cost
                        vm.output
                            .push(format!("OSMOSIS: Moved to {},{}", new_x, new_y));
                    } else {
                        vm.energy = vm.energy.saturating_sub(5);
                        vm.output.push("OSMOSIS: Blocked by boundary".to_string());
                        vm.trigger_reflex(0); // Collision Event
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for osmosis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for osmosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Symbiosis => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                        // Find organelle at (ny, nx)
                        let mut found_idx = None;
                        for (i, org) in vm.organelles.iter().enumerate() {
                            if org.context_loc == (ny, nx) {
                                found_idx = Some(i);
                                break;
                            }
                        }

                        if let Some(idx) = found_idx {
                            let organelle = vm.organelles.remove(idx);

                            // Absorb IP
                            vm.symbiotes.push(organelle.ip);

                            // Absorb Stack
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
                } else {
                    vm.output
                        .push("Error: Type mismatch for symbiosis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for symbiosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Lysis => {
            // Eject the last symbiote
            if let Some(sip) = vm.symbiotes.pop() {
                let (cy, cx) = vm.context_loc;

                // Create organelle
                let organelle = Organelle {
                    stack: Vec::new(),
                    ip: sip,
                    context_loc: (cy, cx),
                    call_stack: Vec::new(),
                    recursion_depth: 0,
                    halted: false,
                    kind: OrganelleType::Worker, // Default
                    direction: (0, 0),
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
        _ => None,
    }
}

#[cfg(feature = "nova")]
pub fn get_direction_mask(dy: i64, dx: i64) -> Option<u8> {
    match (dy, dx) {
        (-1, 0) => Some(1), // N
        (1, 0) => Some(2),  // S
        (0, 1) => Some(4),  // E
        (0, -1) => Some(8), // W
        _ => None,
    }
}
