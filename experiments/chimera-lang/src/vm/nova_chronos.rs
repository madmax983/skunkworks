#![cfg(feature = "nova")]

use super::nova::Phase;
#[cfg(feature = "oracle")]
use super::oracle;
use super::{ChimeraVM, ChromaCell, Value};
use crate::ast::Dna;
use crate::opcode::OpCode;
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a "time-travel" snapshot of the VM state.
#[derive(Clone)]
pub struct Spore {
    pub phase: Phase,
    pub chirality: crate::vm::Chirality,
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
    pub mutagen_grid: Vec<Vec<i64>>,
    pub light_grid: Vec<Vec<i64>>,
    pub call_stack: Vec<(usize, usize)>,
    pub input_buffer: VecDeque<char>,
    pub receptors: HashMap<char, usize>,
    pub entangled_pairs: HashMap<usize, usize>,
    pub portals: HashMap<(usize, usize), (usize, usize)>,
    pub membranes: Vec<Vec<u8>>,
    pub chroma_grid: Vec<Vec<ChromaCell>>,
    pub sonar_target: Option<(usize, usize)>,
    pub symbiotes: Vec<(usize, usize)>,
    pub ether: HashMap<i64, VecDeque<Value>>,
    pub reflexes: HashMap<i64, usize>,
    pub remap_table: HashMap<OpCode, OpCode>,
    pub direction: isize,
    pub mycelium: HashMap<(usize, usize), Vec<(usize, usize)>>,
    pub immune_system: HashSet<u64>,
    pub dictionary: HashMap<String, usize>,
    pub gravity_grid: Vec<Vec<i64>>,
    pub wind_grid: Vec<Vec<(i8, i8)>>,
    pub moisture_grid: Vec<Vec<i64>>,
    pub entropy_grid: Vec<Vec<i64>>,
    pub relativity_mode: bool,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
}

pub fn exec_sporulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.spores.len() >= crate::vm::MAX_SPORES {
        vm.output.push("Error: Spore limit exceeded".to_string());
        return None;
    }

    let spore = create_spore(vm);
    let id = vm.spores.len();
    vm.spores.push(spore);
    vm.stack.push(Value::Int(id as i64));
    vm.energy = vm.energy.saturating_sub(50);
    vm.output.push(format!("SPORULATE: Created Spore {}", id));
    None
}

pub fn exec_time_loop(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(loop_id) = val {
            if vm.spores.len() >= crate::vm::MAX_SPORES {
                vm.output.push("Error: Spore limit exceeded".to_string());
                return None;
            }

            let spore = create_spore(vm);
            let idx = vm.spores.len();
            vm.spores.push(spore);
            vm.paradox_loops.insert(loop_id, idx);
            vm.energy = vm.energy.saturating_sub(50);
            vm.output.push(format!(
                "TIME_LOOP: Anchor {} set at index {}",
                loop_id, idx
            ));
        } else {
            vm.output
                .push("Error: Type mismatch for time_loop".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for time_loop".to_string());
    }
    None
}

fn create_spore(vm: &ChimeraVM) -> Spore {
    Spore {
        phase: vm.phase,
        chirality: vm.chirality,
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
        mutagen_grid: vm.mutagen_grid.clone(),
        light_grid: vm.light_grid.clone(),
        call_stack: vm.call_stack.clone(),
        input_buffer: vm.input_buffer.clone(),
        receptors: vm.receptors.clone(),
        entangled_pairs: vm.entangled_pairs.clone(),
        portals: vm.portals.clone(),
        membranes: vm.membranes.clone(),
        chroma_grid: vm.chroma_grid.clone(),
        sonar_target: vm.sonar_target,
        symbiotes: vm.symbiotes.clone(),
        ether: vm.ether.clone(),
        reflexes: vm.reflexes.clone(),
        remap_table: vm.remap_table.clone(),
        direction: vm.direction,
        mycelium: vm.mycelium.clone(),
        immune_system: vm.immune_system.clone(),
        dictionary: vm.dictionary.clone(),
        gravity_grid: vm.gravity_grid.clone(),
        wind_grid: vm.wind_grid.clone(),
        moisture_grid: vm.moisture_grid.clone(),
        entropy_grid: vm.entropy_grid.clone(),
        relativity_mode: vm.relativity_mode,
        #[cfg(feature = "cortex")]
        synapse_map: vm.synapse_map.clone(),
        #[cfg(feature = "cortex")]
        activation_levels: vm.activation_levels.clone(),
    }
}

fn restore_state(vm: &mut ChimeraVM, spore: &Spore) {
    vm.phase = spore.phase;
    vm.chirality = spore.chirality;
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
    vm.mutagen_grid = spore.mutagen_grid.clone();
    vm.light_grid = spore.light_grid.clone();
    vm.call_stack = spore.call_stack.clone();
    vm.input_buffer = spore.input_buffer.clone();
    vm.receptors = spore.receptors.clone();
    vm.entangled_pairs = spore.entangled_pairs.clone();
    vm.portals = spore.portals.clone();
    vm.membranes = spore.membranes.clone();
    vm.chroma_grid = spore.chroma_grid.clone();
    vm.sonar_target = spore.sonar_target;
    vm.symbiotes = spore.symbiotes.clone();
    vm.ether = spore.ether.clone();
    vm.reflexes = spore.reflexes.clone();
    vm.remap_table = spore.remap_table.clone();
    vm.direction = spore.direction;
    vm.mycelium = spore.mycelium.clone();
    vm.immune_system = spore.immune_system.clone();
    vm.dictionary = spore.dictionary.clone();
    vm.gravity_grid = spore.gravity_grid.clone();
    vm.wind_grid = spore.wind_grid.clone();
    vm.moisture_grid = spore.moisture_grid.clone();
    vm.entropy_grid = spore.entropy_grid.clone();
    vm.relativity_mode = spore.relativity_mode;
    #[cfg(feature = "cortex")]
    {
        vm.synapse_map = spore.synapse_map.clone();
        vm.activation_levels = spore.activation_levels.clone();
    }
}

pub fn exec_germinate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(id) = val {
            let idx = id as usize;
            if idx < vm.spores.len() {
                // Cloning the spore so we can mutate vm
                let spore = vm.spores[idx].clone();
                restore_state(vm, &spore);
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

pub fn exec_paradox(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let val = vm.stack.pop().unwrap();
        let id_val = vm.stack.pop().unwrap();

        if let Value::Int(loop_id) = id_val {
            if let Some(&idx) = vm.paradox_loops.get(&loop_id) {
                if idx < vm.spores.len() {
                    // Clone spore to restore
                    let spore = vm.spores[idx].clone();

                    // Check integrity cost
                    // Cost increases with stack depth of value being sent back?
                    // For now, flat cost + slight recursive cost
                    let entropy_cost = 5.0 + (val.depth() as f64);

                    if vm.chronos_integrity <= entropy_cost {
                        vm.output
                            .push("PARADOX FAILURE: Chronos Integrity Critical".to_string());
                        vm.halted = true; // Timeline collapse
                        return None;
                    }

                    restore_state(vm, &spore);

                    // Apply Paradox Effects
                    vm.stack.push(val); // Push the future knowledge
                    vm.chronos_integrity -= entropy_cost;

                    vm.output.push(format!(
                        "PARADOX: Timeline Rewound to Anchor {} (Integrity: {:.1}%)",
                        loop_id, vm.chronos_integrity
                    ));
                } else {
                    vm.output
                        .push("Error: Invalid Spore Index in Paradox Map".to_string());
                }
            } else {
                vm.output
                    .push(format!("Error: Unknown Time Loop ID {}", loop_id));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for paradox ID".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for paradox".to_string());
    }
    None
}

pub fn exec_time_warp(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let factor_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();

        if let (Value::Int(r), Value::Int(f)) = (radius_val, factor_val) {
            if r > 0 {
                let factor = f.clamp(0, 10) as u8;
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let count = coords.len();

                for (tx, ty) in coords {
                    vm.time_grid[ty][tx] = factor;
                }

                let cost_multiplier = if factor == 0 { 2 } else { factor as i64 };
                vm.energy = vm
                    .energy
                    .saturating_sub((count as i64 * cost_multiplier) / 2);
                vm.output.push(format!(
                    "TIME_WARP: Set time factor {} at {},{} r={}",
                    factor, cx, cy, r
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for time_warp".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for time_warp".to_string());
    }
    None
}

pub fn exec_chronos(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let factor = vm.time_grid[cy][cx];
    vm.stack.push(Value::Int(factor as i64));
    None
}

pub fn exec_chronostasis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(ticks) = val {
            if ticks > 0 {
                vm.chronostasis_timer = ticks as usize;
                let cost = 50 + ticks;
                vm.energy = vm.energy.saturating_sub(cost);
                vm.output
                    .push(format!("CHRONOSTASIS: Time frozen for {} ticks", ticks));
            } else {
                vm.output
                    .push("Error: Invalid ticks for chronostasis".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for chronostasis".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for chronostasis".to_string());
    }
    None
}

pub fn exec_retrograde(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(ticks) = val {
            if ticks > 0 {
                let history_len = vm.grid_history.len();
                let t_idx = ticks as usize;
                if t_idx < history_len {
                    // History is VecDeque, index 0 is oldest?
                    // VM implementation pushes back. So last is newest.
                    // To get 'ticks' ago: len - 1 - ticks.
                    let idx = history_len.saturating_sub(1).saturating_sub(t_idx);

                    let past_grid = vm.grid_history[idx].clone();
                    vm.grid = past_grid;

                    vm.energy = vm.energy.saturating_sub(ticks * 2);
                    vm.output
                        .push(format!("RETROGRADE: Reverted grid to {} ticks ago", ticks));
                } else {
                    vm.output
                        .push("RETROGRADE: Ticks exceed history depth".to_string());
                }
            } else {
                vm.output
                    .push("Error: Invalid ticks for retrograde".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for retrograde".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for retrograde".to_string());
    }
    None
}

#[cfg(feature = "oracle")]
pub fn exec_divergence(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: query, count (top)
    if vm.stack.len() >= 2 {
        let count_val = vm.stack.pop().unwrap();
        let query_val = vm.stack.pop().unwrap();

        if let Value::Int(count) = count_val {
            let limit = count.clamp(1, 50); // Safety limit

            // Check recursion depth to prevent infinite divergence
            if vm.recursion_depth > 5 {
                vm.output
                    .push("DIVERGENCE: Recursion limit reached".to_string());
                vm.stack.push(Value::Int(0));
                return None;
            }

            for i in 0..limit {
                // Fork timeline
                let mut sim_vm = vm.clone();
                sim_vm.recursion_depth += 1;
                sim_vm.halted = false;
                // Silence output for simulations
                // sim_vm.output.clear();

                // Apply chaos/mutation to diverge the timeline
                sim_vm.mutate();

                // Run for a short burst to allow consequences to unfold
                for _ in 0..10 {
                    sim_vm.step();
                }

                let mut solutions = Vec::new();

                oracle::solve(
                    &[query_val.clone()],
                    HashMap::new(),
                    &sim_vm.knowledge_base,
                    &sim_vm,
                    &mut solutions,
                    0,
                );

                if !solutions.is_empty() {
                    // Success! Collapse wavefunction to this timeline.
                    // Preserve original output log + divergence success msg
                    let mut original_output = vm.output.clone();
                    original_output.push(format!("DIVERGENCE: Timeline #{} Shifted! Success.", i));

                    *vm = sim_vm;
                    vm.output = original_output;
                    vm.recursion_depth -= 1; // Fix depth
                    vm.stack.push(Value::Int(1));
                    return None;
                }
            }

            // Failure
            vm.stack.push(Value::Int(0));
            vm.output
                .push("DIVERGENCE: All timelines failed.".to_string());
        } else {
            vm.output
                .push("Error: Type mismatch for divergence".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for divergence".to_string());
    }
    None
}
