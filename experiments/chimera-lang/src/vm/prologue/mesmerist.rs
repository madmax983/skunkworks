use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

/// Projects the Mesmerist's gaze as signals on the grid.
///
/// This is called during the Signal Preparation phase.
/// The Mesmerist reads from West (Suggestion) and projects a cone East.
pub fn emit_gaze(vm: &mut ChimeraVM, y: usize, x: usize) {
    // 1. Read Suggestion from West
    let suggestion = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        match &vm.grid[wy][wx] {
            Value::Str(s) if !s.is_empty() => Some(Value::Str(s.clone())),
            Value::Junction(t, items) => Some(Value::Junction(*t, items.clone())),
            _ => None,
        }
    } else {
        None
    };

    if let Some(signal) = suggestion {
        // 2. Project Cone East
        // Range 3
        let range = 3;
        for r in 1..=range {
            let tx = x as i64 + r as i64;
            // Cone width grows with range: +/- (r-1)
            let width = r - 1;
            for dy in -(width as i64)..=(width as i64) {
                let ty = y as i64 + dy;
                if let Some((ny, nx)) = normalize_coords(ty, tx) {
                    // Write signal
                    vm.prologue_state.signal_grid[ny][nx] = Some(signal.clone());
                }
            }
        }
    }
}

/// Processes the Mesmerist agent's movement.
///
/// For now, it behaves like a Chaos agent (moves randomly).
pub fn process_mesmerist_logic(
    _vm: &mut ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(usize, usize)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 10% chance to move
    if rng.gen_bool(0.1) {
        let (y, x) = (agent.y, agent.x);
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut possible = Vec::new();

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                if matches!(grid_snapshot[ny][nx], Value::Int(0)) {
                    possible.push((ny, nx));
                }
            }
        }

        if !possible.is_empty() {
            let idx = rng.gen_range(0..possible.len());
            return Some(possible[idx]);
        }
    }

    None
}
