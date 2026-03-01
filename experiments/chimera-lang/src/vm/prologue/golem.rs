use super::{
    normalize_coords, PrologueAgent, MAT_BRICK, MAT_EARTH, MAT_HEART, MAT_SHIELD, MAT_WALL,
};
use crate::vm::{ChimeraVM, Value};
use std::collections::{HashSet, VecDeque};

/// The Golem Agent (🗿) is a multi-tile construct.
///
/// It treats adjacent "Material" blocks as part of its body.
/// It moves as a single unit based on signals received by the Heart.
pub fn process_golem_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Read Signal at Heart for Direction
    // Signal codes: 0=N, 1=E, 2=S, 3=W
    // Or directional inputs? Let's use signal grid.
    let direction_sig = vm.prologue_state.signal_grid[y][x].clone();
    let (dy, dx) = match direction_sig {
        Some(Value::Int(0)) => (-1, 0), // N
        Some(Value::Int(1)) => (0, 1),  // E
        Some(Value::Int(2)) => (1, 0),  // S
        Some(Value::Int(3)) => (0, -1), // W
        _ => (0, 0),                    // No movement
    };

    if dy == 0 && dx == 0 {
        return Some((current_agent, None));
    }

    // 2. Identify Body (BFS)
    let mut body = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((y, x));
    body.insert((y, x));

    let materials = [MAT_HEART, MAT_EARTH, MAT_BRICK, MAT_WALL, MAT_SHIELD];

    while let Some((cy, cx)) = queue.pop_front() {
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (ny, nx) in neighbors {
            if let Some((ty, tx)) = normalize_coords(cy as i64 + ny, cx as i64 + nx) {
                if !body.contains(&(ty, tx)) {
                    if let Value::Str(s) = &grid_snapshot[ty][tx] {
                        if materials.contains(&s.as_str()) {
                            body.insert((ty, tx));
                            queue.push_back((ty, tx));
                        }
                    }
                }
            }
        }
    }

    // 3. Collision Check
    // Calculate new positions for all body parts
    // If ANY part hits a non-empty cell that is NOT part of the body itself, block.
    let mut new_positions = Vec::new();
    let mut blocked = false;

    for &(by, bx) in &body {
        if let Some((ny, nx)) = normalize_coords(by as i64 + dy, bx as i64 + dx) {
            // Check occupancy in SNAPSHOT
            // If (ny, nx) is in body, it's fine (moving into self's old space).
            // If (ny, nx) is occupied by something else, BLOCK.
            if !body.contains(&(ny, nx)) && !matches!(grid_snapshot[ny][nx], Value::Int(0)) {
                blocked = true;
                break;
            }
            new_positions.push(((by, bx), (ny, nx)));
        } else {
            // Hit edge of world (if wrapping handled by normalize_coords returns None)
            // But normalize_coords wraps if Torus, or returns None if Plane?
            // current impl of normalize_coords returns None if out of bounds (Plane) or wraps?
            // Let's assume Plane behavior from previous files (if not Torus).
            // Actually normalize_coords in mod.rs checks 0..GRID_SIZE.
            blocked = true;
            break;
        }
    }

    if blocked {
        // Blocked - Stay put
        return Some((current_agent, None));
    }

    // 4. Move
    // We need to write to VM grid.
    // Order matters to avoid overwriting if we don't clear first.
    // Safest: Clear all old positions, then write all new positions.

    // Save values
    let mut values = Vec::new();
    for ((oy, ox), _) in &new_positions {
        values.push(vm.grid[*oy][*ox].clone());
    }

    // Clear old
    for ((oy, ox), _) in &new_positions {
        vm.grid[*oy][*ox] = Value::Int(0);
    }

    // Write new
    for (i, (_, (ny, nx))) in new_positions.iter().enumerate() {
        vm.grid[*ny][*nx] = values[i].clone();
    }

    // Calculate new heart pos for agent update
    let new_heart_pos = if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        (ny, nx)
    } else {
        (y, x) // Should not happen if not blocked
    };

    Some((current_agent, Some(new_heart_pos)))
}
