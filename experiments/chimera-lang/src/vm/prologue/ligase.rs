use super::{normalize_coords, PrologueAgent};
use crate::ast::JunctionType;
use crate::vm::{ChimeraVM, Value};
use rand::seq::SliceRandom;

/// The Ligase Agent (🔗) joins values into lists.
///
/// If it encounters values to its North and South:
/// 1. Combines them into a `Value::Junction(Any, [North, South])`.
/// 2. Writes the result to East.
/// 3. Clears North and South.
pub fn process_ligase_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // Scan North and South
    let north_pos = normalize_coords(y as i64 - 1, x as i64);
    let south_pos = normalize_coords(y as i64 + 1, x as i64);

    if let (Some((ny, nx)), Some((sy, sx))) = (north_pos, south_pos) {
        let n_val = &grid_snapshot[ny][nx];
        let s_val = &grid_snapshot[sy][sx];

        let has_n = !matches!(n_val, Value::Int(0));
        let has_s = !matches!(s_val, Value::Int(0));

        if has_n && has_s {
            // Join
            let new_list = Value::Junction(JunctionType::Any, vec![n_val.clone(), s_val.clone()]);

            // Write to East
            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.grid[ey][ex] = new_list;
            }

            // Clear inputs
            vm.grid[ny][nx] = Value::Int(0);
            vm.grid[sy][sx] = Value::Int(0);

            // Signal success
            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

            // Do not move if active
            return Some((current_agent, None));
        }
    }

    // Move Logic: Random
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();
    let target_pos = neighbors
        .choose(&mut rng)
        .and_then(|(dy, dx)| normalize_coords(y as i64 + dy, x as i64 + dx))
        .and_then(|(ny, nx)| match &grid_snapshot[ny][nx] {
            Value::Int(0) => Some((ny, nx)),
            _ => None,
        });

    Some((current_agent, target_pos))
}
