use super::normalize_coords;
use crate::vm::prologue::epigenetics::EpigeneticMark;
use crate::vm::prologue::PrologueAgent;
use crate::vm::ChimeraVM;
use crate::vm::Value;
use rand::Rng;

pub fn apply_scavenger_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "♻" {
        // Recycle: Signal West -> Spawn Scavenger South
        if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
            if let Some(_sig) = &vm.prologue_state.signal_grid[wy][wx] {
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    vm.grid[sy][sx] = Value::Str("♻".to_string());
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    vm.output
                        .push(format!("SCAVENGER: Agent spawned at {},{}", sx, sy));
                }
            }
        }
    }
}

pub fn process_scavenger_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    _grid_snapshot: &[Vec<Value>],
) -> Option<(usize, usize)> {
    let (y, x) = (agent.y, agent.x);
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut best_target = None;
    let mut found_rot = false;

    // Scan neighbors for Rotting marks
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if vm.prologue_state.epigenetic_grid[ny][nx] == EpigeneticMark::Rotting {
                // Found food!
                best_target = Some((ny, nx));
                found_rot = true;
                break; // Take the first one found
            }
        }
    }

    if let Some((ny, nx)) = best_target {
        if found_rot {
            // Consume the Rot
            vm.prologue_state.epigenetic_grid[ny][nx] = EpigeneticMark::None;
            // Gain Energy
            vm.energy += 5;
            // Emit satisfaction signal?
            // vm.prologue_state.signal_grid[ny][nx] = Some(Value::Int(1)); // Maybe?
        }
        return Some((ny, nx));
    }

    // If no food, move randomly (Brownian motion)
    let mut rng = rand::thread_rng();
    let mut possible_moves = Vec::new();
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            // Avoid obstacles (non-empty cells)
            // But we can move into empty space.
            let cell_empty = match &vm.grid[ny][nx] {
                Value::Int(0) => true,
                Value::Str(s) if s.is_empty() || s == "." => true,
                _ => false,
            };

            if cell_empty {
                possible_moves.push((ny, nx));
            }
        }
    }

    if !possible_moves.is_empty() {
        let idx = rng.gen_range(0..possible_moves.len());
        Some(possible_moves[idx])
    } else {
        None // Stay put
    }
}
