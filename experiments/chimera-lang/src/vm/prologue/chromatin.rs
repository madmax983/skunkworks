use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};
use rand::seq::SliceRandom;

pub fn process_chromatin_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);
    let mut rng = rand::thread_rng();

    // 0. Reprogram: Check North for Configuration List
    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        if let Value::Junction(_, ref list) = grid_snapshot[ny][nx] {
            if list.len() >= 2 {
                let func = if let Value::Int(n) = list[0] {
                    n
                } else {
                    0
                };
                let target = if let Value::Str(ref s) = list[1] {
                    s.clone()
                } else {
                    "".to_string()
                };

                // Update State if changed
                let new_state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![Value::Int(func), Value::Str(target.clone())],
                );

                if current_agent.state != new_state {
                    current_agent.state = new_state;
                    vm.output.push(format!(
                        "CHROMATIN: Reprogrammed to Function {}, Target '{}'",
                        func, target
                    ));
                }
            }
        }
    }

    // Parse State: [Function, Target]
    let (func, target) = match &current_agent.state {
        Value::Junction(_, list) if list.len() >= 2 => {
            let f = if let Value::Int(n) = list[0] {
                n
            } else {
                0
            };
            let t = if let Value::Str(s) = &list[1] {
                s.clone()
            } else {
                "".to_string()
            };
            (f, t)
        }
        _ => (0, "".to_string()),
    };

    if target.is_empty() {
        // Move Randomly even if empty target (seeking configuration?)
        // The original code returned Some((current_agent, None)) effectively stopping it.
        // Let's allow it to move randomly to find a config.
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
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
        return Some((current_agent, target_pos));
    }

    // Scan neighbors for Target
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == &target {
                    // Apply Epigenetic Mark
                    let mark = match func {
                        0 => super::epigenetics::EpigeneticMark::Methylated,
                        1 => super::epigenetics::EpigeneticMark::Phosphorylated,
                        2 => super::epigenetics::EpigeneticMark::None,
                        3 => super::epigenetics::EpigeneticMark::Rotting,
                        _ => super::epigenetics::EpigeneticMark::None,
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
