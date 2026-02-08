#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn exec_scribe(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., value ]
    if let Some(val) = vm.stack.pop() {
        let rune = match val {
            Value::Str(s) => s,
            Value::Int(n) => n.to_string(),
            _ => "UNKNOWN".to_string(),
        };

        let (cy, cx) = vm.context_loc;
        if cy < crate::vm::GRID_SIZE && cx < crate::vm::GRID_SIZE {
            vm.void_grid[cy][cx] = Some(rune.clone());
            vm.energy = vm.energy.saturating_sub(5);
            vm.output.push(format!("SCRIBE: '{}' at {},{}", rune, cx, cy));
        } else {
            vm.output.push("SCRIBE ERROR: Out of bounds".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for scribe".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_read_void(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if cy < crate::vm::GRID_SIZE && cx < crate::vm::GRID_SIZE {
        if let Some(rune) = &vm.void_grid[cy][cx] {
            vm.stack.push(Value::Str(rune.clone()));
        } else {
            vm.stack.push(Value::Int(0)); // Empty
        }
        vm.energy = vm.energy.saturating_sub(1);
    } else {
        vm.output.push("READ_VOID ERROR: Out of bounds".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_spell(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;

    // Read neighbors: N, E, S, W
    let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut words = Vec::new();
    let mut used_coords = Vec::new();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            if let Some(rune) = &vm.void_grid[ny][nx] {
                words.push(rune.clone());
                used_coords.push((ny, nx));
            }
        }
    }

    if words.is_empty() {
        vm.output.push("SPELL: No runes found nearby".to_string());
        return None;
    }

    let sentence = words.join("");
    let lower_sentence = sentence.to_lowercase();

    let mut matched_spell = None;
    let mut effect_desc = "";

    // Spell Dictionary
    if lower_sentence.contains("firestorm") {
        matched_spell = Some(OpCode::Storm);
        effect_desc = "Firestorm";
        // Logic handled below
    } else if lower_sentence.contains("lifedeath") {
        matched_spell = Some(OpCode::Reincarnate);
        effect_desc = "Cycle of Life";
    } else if lower_sentence.contains("shifttime") {
        matched_spell = Some(OpCode::TimeWarp);
        effect_desc = "Time Shift";
    } else if lower_sentence.contains("growwild") {
        matched_spell = Some(OpCode::Terraform);
        effect_desc = "Wild Growth";
    }

    if let Some(op) = matched_spell {
        vm.output.push(format!("SPELL: Cast '{}'", effect_desc));
        vm.energy = vm.energy.saturating_sub(20);

        // Consume Runes
        for (y, x) in used_coords {
            vm.void_grid[y][x] = None;
        }

        // Apply Effects
        match op {
            OpCode::Storm => {
                // Intense Storm
                vm.stack.push(Value::Int(5)); // Radius
                vm.stack.push(Value::Int(50)); // Intensity
                return crate::vm::nova::exec_nova_op(vm, OpCode::Storm, &[]);
            }
            OpCode::Reincarnate => {
                // Reincarnate Self
                vm.stack.push(Value::Int(vm.ip.0 as i64));
                return crate::vm::nova::exec_nova_op(vm, OpCode::Reincarnate, &[]);
            }
            OpCode::TimeWarp => {
                // Slow time locally
                vm.stack.push(Value::Int(3)); // Radius
                vm.stack.push(Value::Int(0)); // Factor 0 = Stasis
                return crate::vm::nova::exec_nova_op(vm, OpCode::TimeWarp, &[]);
            }
            OpCode::Terraform => {
                // Jungle (Swamp=1)
                vm.stack.push(Value::Int(1)); // Biome
                vm.stack.push(Value::Int(5)); // Radius
                return crate::vm::nova::exec_nova_op(vm, OpCode::Terraform, &[]);
            }
            _ => {}
        }
    } else {
        vm.output.push(format!("SPELL: Fizzled (Sentence: '{}')", sentence));
        vm.energy = vm.energy.saturating_sub(5);
    }

    None
}
