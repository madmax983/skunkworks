#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_rune_inscribe(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            let rune_char = s.chars().next().unwrap_or('?');
            let (cy, cx) = vm.context_loc;
            vm.grid[cy][cx] = Value::Str(format!("Rune:{}", rune_char));
            vm.output
                .push(format!("RUNE: Inscribed '{}' at {},{}", rune_char, cx, cy));
            vm.energy = vm.energy.saturating_sub(10);
        } else {
            vm.output
                .push("Error: Type mismatch for RuneInscribe".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for RuneInscribe".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rune_invoke(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            let rune_char = s.chars().next().unwrap_or('?');
            let targets = scan_runes(vm, rune_char);

            if targets.is_empty() {
                vm.output.push(format!("RUNE: No '{}' runes found", rune_char));
            } else {
                vm.output
                    .push(format!("RUNE: Invoking {} instances of '{}'", targets.len(), rune_char));

                // Trigger linked runes recursively?
                // For simplicity, let's just trigger direct effects for now.
                // Links will be handled by re-invoking.

                let mut linked_runes = Vec::new();
                if let Some(linked) = vm.rune_links.get(&rune_char) {
                    linked_runes.push(*linked);
                }

                for (y, x) in targets {
                    trigger_rune_effect(vm, rune_char, y, x);
                }

                for linked in linked_runes {
                    vm.output.push(format!("RUNE: Chained invoke '{}'", linked));
                    // We need to invoke the linked rune too.
                    // Push linked rune to stack and recurse?
                    // Or just call scan and trigger loop again.
                    // Recursing might be safer via VM loop but we are in a function.
                    // Let's just do one level of chaining for now to avoid infinite loops.
                    let linked_targets = scan_runes(vm, linked);
                    for (ly, lx) in linked_targets {
                        trigger_rune_effect(vm, linked, ly, lx);
                    }
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for RuneInvoke".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for RuneInvoke".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rune_link(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let val_b = vm.stack.pop().unwrap();
        let val_a = vm.stack.pop().unwrap();

        if let (Value::Str(sa), Value::Str(sb)) = (val_a, val_b) {
            let ra = sa.chars().next().unwrap_or('?');
            let rb = sb.chars().next().unwrap_or('?');
            vm.rune_links.insert(ra, rb);
            vm.output.push(format!("RUNE: Linked '{}' -> '{}'", ra, rb));
        } else {
             vm.output
                .push("Error: Type mismatch for RuneLink".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for RuneLink".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rune_read(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            // Check bounds? vm.grid access usually needs usize.
            // Using vm.normalize_coords logic or just clamp?
            // vm.grid is Vec<Vec<Value>>.
            // Let's use is_valid_coord helper logic if possible, but it's private in mod.rs.
            // We'll duplicate check.
            if y >= 0 && y < crate::vm::GRID_SIZE as i64 && x >= 0 && x < crate::vm::GRID_SIZE as i64 {
                let val = &vm.grid[y as usize][x as usize];
                if let Value::Str(s) = val {
                    if let Some(stripped) = s.strip_prefix("Rune:") {
                        vm.stack.push(Value::Str(stripped.to_string()));
                        return None;
                    }
                }
                vm.stack.push(Value::Int(0)); // Not a rune
            } else {
                vm.output.push("Error: Grid index out of bounds".to_string());
            }
        } else {
             vm.output
                .push("Error: Type mismatch for RuneRead".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for RuneRead".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rune_sense(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let mut nearest: Option<(i64, i64, String)> = None;
    let mut min_dist = f64::MAX;

    for y in 0..crate::vm::GRID_SIZE {
        for x in 0..crate::vm::GRID_SIZE {
            if let Value::Str(s) = &vm.grid[y][x] {
                if let Some(stripped) = s.strip_prefix("Rune:") {
                    let dy = (y as i64) - (cy as i64);
                    let dx = (x as i64) - (cx as i64);
                    let dist = ((dy*dy + dx*dx) as f64).sqrt();

                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((dy, dx, stripped.to_string()));
                    }
                }
            }
        }
    }

    if let Some((dy, dx, r)) = nearest {
        vm.stack.push(Value::Int(dy));
        vm.stack.push(Value::Int(dx));
        vm.stack.push(Value::Str(r));
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

#[cfg(feature = "nova")]
fn scan_runes(vm: &ChimeraVM, target: char) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let pattern = format!("Rune:{}", target);
    for y in 0..crate::vm::GRID_SIZE {
        for x in 0..crate::vm::GRID_SIZE {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == &pattern {
                    found.push((y, x));
                }
            }
        }
    }
    found
}

#[cfg(feature = "nova")]
fn trigger_rune_effect(vm: &mut ChimeraVM, rune: char, y: usize, x: usize) {
    match rune {
        'F' | 'ᚠ' => { // Fehu (Wealth)
            vm.energy = vm.energy.saturating_add(10);
            vm.output.push(format!("RUNE EFFECT: Fehu at {},{} (+10 Energy)", x, y));
        }
        'T' | 'ᚦ' => { // Thurisaz (Giant/Thor) - Destroy neighbors
            let offsets = [(-1,0), (1,0), (0,-1), (0,1)];
            for (dy, dx) in offsets {
                // Warning: No wrapping check here, assuming simple bounds
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;
                if ny >= 0 && ny < crate::vm::GRID_SIZE as i64 && nx >= 0 && nx < crate::vm::GRID_SIZE as i64 {
                    vm.grid[ny as usize][nx as usize] = Value::Int(0);
                }
            }
            vm.output.push(format!("RUNE EFFECT: Thurisaz at {},{} (Smash)", x, y));
        }
        'K' | 'ᚲ' => { // Kenaz (Torch) - Light
             vm.light_grid[y][x] = vm.light_grid[y][x].saturating_add(100);
             vm.output.push(format!("RUNE EFFECT: Kenaz at {},{} (Light)", x, y));
        }
        'R' | 'ᚱ' => { // Raido (Ride) - Teleport Organism
            vm.context_loc = (y, x);
            vm.output.push(format!("RUNE EFFECT: Raido teleported to {},{}", x, y));
        }
        'A' | 'ᚨ' => { // Ansuz (God/Odin) - Wisdom (Genome info?)
            // Push genome length to stack
            let len = vm.dna.helix.strands.len();
            vm.stack.push(Value::Int(len as i64));
            vm.output.push(format!("RUNE EFFECT: Ansuz revealed helix length {}", len));
        }
        'G' | 'ᚷ' => { // Gebo (Gift) - Spawn Random Value
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let val = rng.gen_range(0..100);
            // Place in neighbor
            if x + 1 < crate::vm::GRID_SIZE {
                vm.grid[y][x+1] = Value::Int(val);
            }
             vm.output.push(format!("RUNE EFFECT: Gebo gifted {} at {},{}", val, x+1, y));
        }
        _ => {
            vm.output.push(format!("RUNE EFFECT: Unknown '{}' at {},{}", rune, x, y));
        }
    }
}
