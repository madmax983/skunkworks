#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::nova::{Organelle, OrganelleType};
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_invoke(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(sigil_name) = val {
            perform_invoke(vm, &sigil_name);
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
fn perform_invoke(vm: &mut ChimeraVM, name: &str) {
    let (cy, cx) = vm.context_loc;

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
            vm.output.push(format!("INVOKE: Unknown Sigil '{}'", name));
        }
    }
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
fn consume_pattern(vm: &mut ChimeraVM, cy: usize, cx: usize, offsets: &[(i64, i64)]) {
    for &(dy, dx) in offsets {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            vm.grid[ny][nx] = Value::Int(0);
        }
    }
}
