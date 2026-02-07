#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::nova::{Organelle, OrganelleType};
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sigil {
    pub pattern: Vec<(i64, i64, Value)>,
    pub strand_idx: usize,
}

#[cfg(feature = "nova")]
pub fn exec_invoke(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(sigil_name) = val {
            return perform_invoke(vm, &sigil_name);
        } else {
            vm.output
                .push("Error: Type mismatch for invoke".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for invoke".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn perform_invoke(vm: &mut ChimeraVM, name: &str) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;

    // Check Dynamic Registry first
    if let Some(sigil) = vm.sigil_registry.get(name).cloned() {
        if check_dynamic_pattern(vm, cy, cx, &sigil.pattern) {
            consume_dynamic_pattern(vm, cy, cx, &sigil.pattern);
            vm.energy = vm.energy.saturating_sub(10);
            vm.output.push(format!("INVOKE: '{}' activated!", name));

            // Invoke means CALL the strand
            if vm.call_stack.len() < crate::vm::MAX_CALL_STACK_DEPTH {
                vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));
                return Some((sigil.strand_idx, 0));
            } else {
                vm.output.push("Error: Call stack overflow during invoke".to_string());
                return None;
            }
        } else {
             // Fallthrough to hardcoded or fail
        }
    }

    match name {
        "Ward" => {
            // Ring of 8
            let offsets = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            if let Some(material) = check_pattern_uniform(vm, cy, cx, &offsets) {
                consume_pattern(vm, cy, cx, &offsets);
                // Effect: Toggle all membranes at center (Shield)
                // 15 = 1|2|4|8 = N|S|E|W
                vm.membranes[cy][cx] = 15;
                vm.output
                    .push(format!("INVOKE: Ward activated using {}", material));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("INVOKE: Ward failed (Pattern mismatch)".to_string());
            }
        }
        "Vitality" => {
            // Cross of 4 (N, S, E, W)
            let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            if let Some(material) = check_pattern_uniform(vm, cy, cx, &offsets) {
                let value = match material {
                    Value::Int(n) => n,
                    Value::Str(s) => s.len() as i64,
                    _ => 1,
                };
                consume_pattern(vm, cy, cx, &offsets);
                let gain = value.saturating_mul(5);
                vm.energy = vm.energy.saturating_add(gain);
                vm.output
                    .push(format!("INVOKE: Vitality restored {} energy", gain));
            } else {
                vm.output.push("INVOKE: Vitality failed".to_string());
            }
        }
        "Void" => {
            // 2x2 Square to the Bottom-Right: (0,0), (0,1), (1,0), (1,1)
            // Note: (0,0) is the center/executor.
            let offsets = [(0, 0), (0, 1), (1, 0), (1, 1)];
            if check_pattern_uniform(vm, cy, cx, &offsets).is_some() {
                consume_pattern(vm, cy, cx, &offsets);
                // Spawn Void
                if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
                    let organelle = Organelle {
                        stack: Vec::new(),
                        ip: (0, 0),
                        context_loc: (cy, cx),
                        call_stack: Vec::new(),
                        recursion_depth: 0,
                        halted: false,
                        kind: OrganelleType::Void,
                        direction: (0, 0),
                        ttl: None,
                    };
                    vm.organelles.push(organelle);
                    vm.output.push("INVOKE: Void Summoned".to_string());
                } else {
                    vm.output
                        .push("INVOKE: Organelle limit reached".to_string());
                }
            } else {
                vm.output.push("INVOKE: Void failed".to_string());
            }
        }
        _ => {
            // Only warn if not found in registry either
            if !vm.sigil_registry.contains_key(name) {
                vm.output.push(format!("INVOKE: Unknown Sigil '{}'", name));
            }
        }
    }
    None
}

#[cfg(feature = "nova")]
fn check_pattern_uniform(
    vm: &ChimeraVM,
    cy: usize,
    cx: usize,
    offsets: &[(i64, i64)],
) -> Option<Value> {
    let mut first_val: Option<Value> = None;

    for &(dy, dx) in offsets {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            let val = &vm.grid[ny][nx];
            if let Value::Int(0) = val {
                return None; // Empty space breaks pattern
            }

            if let Some(ref first) = first_val {
                if first != val {
                    return None; // Mismatch
                }
            } else {
                first_val = Some(val.clone());
            }
        } else {
            return None; // Out of bounds
        }
    }
    first_val
}

#[cfg(feature = "nova")]
fn check_dynamic_pattern(
    vm: &ChimeraVM,
    cy: usize,
    cx: usize,
    pattern: &[(i64, i64, Value)],
) -> bool {
    for (dy, dx, expected) in pattern {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + *dy, cx as i64 + *dx) {
            let val = &vm.grid[ny][nx];
            if val != expected {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(feature = "nova")]
fn consume_pattern(vm: &mut ChimeraVM, cy: usize, cx: usize, offsets: &[(i64, i64)]) {
    for &(dy, dx) in offsets {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            vm.grid[ny][nx] = Value::Int(0);
        }
    }
}

#[cfg(feature = "nova")]
fn consume_dynamic_pattern(vm: &mut ChimeraVM, cy: usize, cx: usize, pattern: &[(i64, i64, Value)]) {
    for (dy, dx, _) in pattern {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + *dy, cx as i64 + *dx) {
            vm.grid[ny][nx] = Value::Int(0);
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_inscribe(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., strand_idx, radius, sigil_name ]
    if vm.stack.len() >= 3 {
        let name_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();
        let strand_val = vm.stack.pop().unwrap();

        if let (Value::Str(name), Value::Int(radius), Value::Int(s_idx)) = (name_val, radius_val, strand_val) {
            if radius > 0 {
                let (cy, cx) = vm.context_loc;
                let mut pattern = Vec::new();

                // Scan circular area
                let coords = vm.get_circular_coords(cx as i64, cy as i64, radius);

                for (tx, ty) in coords {
                    let val = &vm.grid[ty][tx];
                    if !matches!(val, Value::Int(0)) {
                        // Calculate relative offset
                        // Handle topology wrapping if needed?
                        // For simplicity, we store relative coordinates derived from linear difference.
                        // Ideally we should use modular difference for torus.
                        // But get_circular_coords returns absolute coords.

                        // Let's rely on simple difference and assume local coherence.
                        // Or better: In `get_circular_coords`, we iterated dx, dy.
                        // But we don't have that here easily without re-calculating.

                        // Let's recalc relative offsets.
                        let dy = (ty as i64) - (cy as i64);
                        let dx = (tx as i64) - (cx as i64);

                        // We must handle wrapping if we want the sigil to be portable across boundaries.
                        // But standard subtraction is fine if the pattern is "local".
                        // Wait, if (cy, cx) is (0,0) and (ty, tx) is (15, 15) via wrapping?
                        // get_circular_coords logic:
                        // "for y in 0..GRID_SIZE ... let dy = y - cy ... if dist_sq <= r_sq"
                        // This uses minimal linear distance on a plane (mostly).
                        // It does NOT handle wrapping logic for distance calculation unless we implemented it there.
                        // Looking at `get_circular_coords` in `vm/mod.rs`:
                        // It iterates 0..16, subtracts, checks dist.
                        // This implies it only finds neighbors that are "linearly" close.
                        // So (0,0) neighbors are (0,1), (1,0) etc. (15,15) is far away (dist 15).
                        // So Torus wrapping is ignored in `get_circular_coords` for Plane/Torus mixed logic.

                        // So simple subtraction is correct for the coords returned by `get_circular_coords`.

                        pattern.push((dy, dx, val.clone()));
                    }
                }

                if !pattern.is_empty() {
                    let sigil = Sigil {
                        pattern,
                        strand_idx: s_idx as usize,
                    };
                    vm.sigil_registry.insert(name.clone(), sigil);
                    vm.output.push(format!("INSCRIBE: Learned '{}' with radius {}", name, radius));
                    vm.energy = vm.energy.saturating_sub(25);
                } else {
                    vm.output.push("INSCRIBE: No pattern found (empty space)".to_string());
                }
            } else {
                vm.output.push("Error: Invalid radius for inscribe".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for inscribe".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for inscribe".to_string());
    }
    None
}
