use super::{normalize_coords, PrologueAgent};
use super::epigenetics::EpigeneticMark;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use rand::seq::SliceRandom;

pub fn process_chromatin_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);
    let mut rng = rand::thread_rng();

    // Parse State: [Function, Target]
    let (func, target) = match &agent.state {
        Value::Junction(_, list) if list.len() >= 2 => {
            let f = if let Value::Int(n) = list[0] { n } else { 0 };
            let t = if let Value::Str(s) = &list[1] { s.clone() } else { "".to_string() };
            (f, t)
        }
        _ => (0, "".to_string()),
    };

    if target.is_empty() {
        return Some((current_agent, None));
    }

    // Scan neighbors for Target
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == &target {
                    // Apply Epigenetic Mark
                    let mark = match func {
                        0 => EpigeneticMark::Methylated,
                        1 => EpigeneticMark::Phosphorylated,
                        2 => EpigeneticMark::None,
                        3 => EpigeneticMark::Rotting,
                        _ => EpigeneticMark::None,
                    };

                    // Only apply if changed
                    if vm.prologue_state.epigenetic_grid[ny][nx] != mark {
                        vm.prologue_state.epigenetic_grid[ny][nx] = mark;
                        // Light up
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        vm.output.push(format!("CHROMATIN: Applied {:?} to {} at {},{}", mark, target, nx, ny));
                    }
                }
            }
        }
    }

    // Move Randomly
    let mut possible_moves = Vec::new();
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            match &grid_snapshot[ny][nx] {
                Value::Int(0) => possible_moves.push((ny, nx)),
                Value::Str(s) if s == "~" => possible_moves.push((ny, nx)),
                _ => {}
            }
        }
    }

    let target_pos = if !possible_moves.is_empty() {
        possible_moves.choose(&mut rng).cloned()
    } else {
        None
    };

    Some((current_agent, target_pos))
}
