use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};

/// The Splicer Agent (✂) cuts lists into head and tail.
///
/// If it encounters a List (Junction) to its West:
/// 1. Takes the Head (first element) and writes it to North.
/// 2. Takes the Tail (rest of list) and writes it to South.
/// 3. Consumes the original List (clears West).
pub fn process_splicer_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // Scan West for Junction (List)
    if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        let val = &grid_snapshot[wy][wx];

        if let Value::Junction(j_type, ref list) = val {
            if !list.is_empty() {
                // Split List
                let head = list[0].clone();
                let tail = Value::Junction(j_type.clone(), list[1..].to_vec());

                // Write Head to North
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    vm.grid[ny][nx] = head;
                }

                // Write Tail to South
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    vm.grid[sy][sx] = tail;
                }

                // Consume West
                vm.grid[wy][wx] = Value::Int(0);

                // Signal success
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

                vm.output.push(format!("SPLICER: Split list at {},{}", wx, wy));

                // Do not move if active
                return Some((current_agent, None));
            }
        }
    }

    // Move Logic: Random
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();
    use rand::seq::SliceRandom;
    let target_pos = neighbors
        .choose(&mut rng)
        .and_then(|(dy, dx)| normalize_coords(y as i64 + dy, x as i64 + dx))
        .and_then(|(ny, nx)| match &grid_snapshot[ny][nx] {
            Value::Int(0) => Some((ny, nx)),
            _ => None,
        });

    Some((current_agent, target_pos))
}
