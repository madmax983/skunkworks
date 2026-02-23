use super::{ChimeraVM, Value, MAX_STRANDS};
use crate::vm::nova_genetics;
use crate::{ChimeraParser, Rule};
use pest::Parser;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn exec_quantum_jump(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s_idx = vm.ip.0;
    if let Some(&partner_idx) = vm.entangled_pairs.get(&s_idx) {
        if partner_idx < vm.dna.helix.strands.len() {
            let gene_idx = vm.ip.1;
            let p_len = vm.dna.helix.strands[partner_idx].genes.len();
            let target_gene = if gene_idx < p_len {
                gene_idx
            } else {
                p_len.saturating_sub(1)
            };

            vm.energy = vm.energy.saturating_sub(10);
            vm.output
                .push(format!("QUANTUM_JUMP: {} -> {}", s_idx, partner_idx));
            return Some((partner_idx, target_gene));
        }
    } else {
        vm.output
            .push("QUANTUM_JUMP: No entangled partner".to_string());
    }
    None
}

pub fn exec_entangle(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s_val2 = vm.stack.pop().unwrap();
        let s_val1 = vm.stack.pop().unwrap();
        if let (Value::Int(s1), Value::Int(s2)) = (s_val1, s_val2) {
            let idx1 = s1 as usize;
            let idx2 = s2 as usize;
            let len = vm.dna.helix.strands.len();
            if idx1 < len && idx2 < len {
                if idx1 != idx2 {
                    if let Some(old) = vm.entangled_pairs.remove(&idx1) {
                        vm.entangled_pairs.remove(&old);
                    }
                    if let Some(old) = vm.entangled_pairs.remove(&idx2) {
                        vm.entangled_pairs.remove(&old);
                    }

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

pub fn exec_decohere(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

pub fn exec_superpose(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let b = vm.stack.pop().unwrap();
        let a = vm.stack.pop().unwrap();

        let depth_a = a.depth();
        let depth_b = b.depth();
        if depth_a.max(depth_b) + 1 > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Superpose depth limit exceeded".to_string());
        } else {
            vm.stack
                .push(Value::Superposition(vec![(a, 0.5), (b, 0.5)]));
            vm.energy = vm.energy.saturating_sub(10);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for superpose".to_string());
    }
    None
}

pub fn exec_collapse(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Superposition(states) => {
                let mut rng = rand::thread_rng();
                let r: f64 = rng.gen();
                let mut sum = 0.0;
                let mut collapsed = states[0].0.clone();

                for (v, p) in states {
                    sum += p;
                    if r <= sum {
                        collapsed = v;
                        break;
                    }
                }
                vm.stack.push(collapsed);
                vm.energy = vm.energy.saturating_sub(5);
            }
            other => {
                vm.stack.push(other);
            }
        }
    } else {
        vm.output
            .push("Error: Stack underflow for collapse".to_string());
    }
    None
}

pub fn exec_observe(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Superposition(states) => {
                let mut rng = rand::thread_rng();
                let r: f64 = rng.gen();
                let mut sum = 0.0;
                let mut collapsed = states[0].0.clone();

                for (v, p) in states {
                    sum += p;
                    if r <= sum {
                        collapsed = v;
                        break;
                    }
                }
                vm.stack.push(collapsed.clone());
                vm.energy = vm.energy.saturating_sub(5);
                vm.output.push(format!("OBSERVED: {}", collapsed));
            }
            other => {
                vm.stack.push(other.clone());
                vm.output.push(format!("OBSERVED: {}", other));
            }
        }
    } else {
        vm.output
            .push("Error: Stack underflow for observe".to_string());
    }
    None
}

pub fn exec_horcrux(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_idx), Value::Int(y), Value::Int(x)) = (s_val, y_val, x_val) {
            let idx = s_idx as usize;
            if idx < vm.dna.helix.strands.len() {
                if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                    let strand = &vm.dna.helix.strands[idx];
                    let mut hasher = DefaultHasher::new();
                    strand.hash(&mut hasher);
                    let hash = hasher.finish();
                    let gene_str = nova_genetics::strand_to_string(strand);
                    let horcrux_str = format!("Horcrux:{:x}:{}", hash, gene_str);

                    vm.grid[ny][nx] = Value::Str(horcrux_str);

                    // Kill source strand
                    vm.dna.helix.strands[idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != idx);
                    vm.cladistics.kill_strand(idx, vm.tick_counter);

                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output.push(format!(
                        "HORCRUX: Strand {} soul bound to {},{}",
                        idx, nx, ny
                    ));
                } else {
                    vm.output
                        .push("Error: Coordinates out of bounds for horcrux".to_string());
                }
            } else {
                vm.output
                    .push("Error: Invalid strand index for horcrux".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for horcrux".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for horcrux".to_string());
    }
    None
}

pub fn exec_rebirth(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if let Value::Str(s) = &vm.grid[ny][nx] {
                    if s.starts_with("Horcrux:") {
                        let parts: Vec<&str> = s.splitn(3, ':').collect();
                        if parts.len() == 3 {
                            let _hash = parts[1];
                            let gene_src = parts[2];

                            match ChimeraParser::parse(Rule::strand, gene_src) {
                                Ok(mut pairs) => {
                                    let pair = pairs.next().unwrap();
                                    match crate::ast::Strand::try_from_pair(pair) {
                                        Ok(strand) => {
                                            if vm.dna.helix.strands.len() >= MAX_STRANDS {
                                                vm.output.push(
                                                    "REBIRTH ERROR: Strand limit exceeded"
                                                        .to_string(),
                                                );
                                            } else {
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
                                                    "Rebirth".to_string(),
                                                );

                                                vm.grid[ny][nx] = Value::Int(0); // Consume Horcrux
                                                vm.stack.push(Value::Int(new_idx as i64));
                                                vm.energy = vm.energy.saturating_sub(25);
                                                vm.output.push(format!(
                                                    "REBIRTH: Soul restored as strand {}",
                                                    new_idx
                                                ));
                                            }
                                        }
                                        Err(e) => {
                                            vm.output
                                                .push(format!("REBIRTH ERROR: Parse failed {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    vm.output.push(format!("REBIRTH ERROR: Syntax error {}", e));
                                }
                            }
                        } else {
                            vm.output
                                .push("REBIRTH ERROR: Malformed Horcrux".to_string());
                        }
                    } else {
                        vm.output.push("REBIRTH: Not a Horcrux".to_string());
                        vm.stack.push(Value::Int(-1));
                    }
                } else {
                    vm.output
                        .push("REBIRTH: Cell does not contain string".to_string());
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for rebirth".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for rebirth".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rebirth".to_string());
    }
    None
}
