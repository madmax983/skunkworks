use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

/// The Shuttle Agent (ð) weaves through the grid, interacting with Warps (║).
/// Now upgraded with "Loom" Logic (Tension & Knots).
pub fn process_shuttle_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // Unpack State: [dy, dx, Payload, Underfoot, Tension]
    // Default: East (0, 1), Payload 0, Underfoot Empty(0), Tension 0
    let (dy, dx, mut payload, underfoot, mut tension) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 5 => {
            if let (Value::Int(dy), Value::Int(dx), p, u, Value::Int(t)) =
                (&list[0], &list[1], &list[2], &list[3], &list[4])
            {
                ((*dy), (*dx), p.clone(), u.clone(), *t)
            } else {
                (0, 1, Value::Int(0), Value::Int(0), 0)
            }
        }
        Value::Junction(_, list) if list.len() == 4 => {
            // Backward compatibility for old agents
            if let (Value::Int(dy), Value::Int(dx), p, u) = (&list[0], &list[1], &list[2], &list[3])
            {
                ((*dy), (*dx), p.clone(), u.clone(), 0)
            } else {
                (0, 1, Value::Int(0), Value::Int(0), 0)
            }
        }
        _ => (0, 1, Value::Int(0), Value::Int(0), 0),
    };

    // 1. Interact with Underfoot (the cell we are currently standing on)
    let current_cell = &underfoot;
    if let Value::Str(s) = current_cell {
        match s.as_str() {
            // Loom Logic
            "(" => tension += 1,
            ")" => tension -= 1,
            "8" => {
                // Knot: Interaction based on Tension
                if tension > 0 {
                    // High Tension: Push Payload to Warp (Write)
                    if let Some((wy, wx)) = find_warp(vm, y, x) {
                        vm.prologue_state
                            .registers
                            .insert((wy, wx), payload.clone());
                    }
                    tension = 0; // Snap/Reset
                } else if tension < 0 {
                    // Low Tension: Pull Warp to Payload (Read)
                    if let Some((wy, wx)) = find_warp(vm, y, x) {
                        payload = vm
                            .prologue_state
                            .registers
                            .get(&(wy, wx))
                            .cloned()
                            .unwrap_or(Value::Int(0));
                    }
                    tension = 0; // Relax/Reset
                } else {
                    // Zero Tension: No-op (Pass through)
                }
            }

            // Legacy Arithmetic Logic (Direct Ops)
            "+" => {
                if find_warp(vm, y, x).is_some() {
                    payload = op_warp(vm, y, x, &payload, |a, b| a + b);
                } else {
                    return Some((
                        update_state(updated_agent, dx, -dy, payload, underfoot, tension),
                        Some((y, x)),
                    ));
                }
            }
            "-" => payload = op_warp(vm, y, x, &payload, |a, b| a - b),
            "*" => payload = op_warp(vm, y, x, &payload, |a, b| a * b),
            "%" => payload = op_warp(vm, y, x, &payload, |a, b| if b != 0 { a % b } else { 0 }),

            // Legacy Data Ops
            "§" => {
                // Twist: Swap
                if let Some((wy, wx)) = find_warp(vm, y, x) {
                    let warp_val = vm
                        .prologue_state
                        .registers
                        .get(&(wy, wx))
                        .cloned()
                        .unwrap_or(Value::Int(0));
                    vm.prologue_state
                        .registers
                        .insert((wy, wx), payload.clone());
                    payload = warp_val;
                }
            }
            "!" => {
                // Force Write
                if let Some((wy, wx)) = find_warp(vm, y, x) {
                    vm.prologue_state
                        .registers
                        .insert((wy, wx), payload.clone());
                }
            }
            "?" => {
                // Force Read
                if let Some((wy, wx)) = find_warp(vm, y, x) {
                    payload = vm
                        .prologue_state
                        .registers
                        .get(&(wy, wx))
                        .cloned()
                        .unwrap_or(Value::Int(0));
                }
            }

            // Movement Control
            "^" => {
                return Some((
                    update_state(updated_agent, -1, 0, payload, underfoot, tension),
                    Some((y, x)),
                ))
            }
            "v" => {
                return Some((
                    update_state(updated_agent, 1, 0, payload, underfoot, tension),
                    Some((y, x)),
                ))
            }
            "<" => {
                return Some((
                    update_state(updated_agent, 0, -1, payload, underfoot, tension),
                    Some((y, x)),
                ))
            }
            ">" => {
                return Some((
                    update_state(updated_agent, 0, 1, payload, underfoot, tension),
                    Some((y, x)),
                ))
            }
            _ => {}
        }
    }

    // 2. Move
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        // Collision check against SNAPSHOT (to see if blocked by another agent)
        if is_blocking(&grid_snapshot[ny][nx]) {
            // Bounce
            return Some((
                update_state(updated_agent, -dy, -dx, payload, underfoot, tension),
                None,
            ));
        }

        // Restore underfoot at current pos
        vm.grid[y][x] = underfoot;

        // Capture new underfoot from LIVE grid
        // This ensures we pick up whatever is actually there (e.g. static runes)
        let next_underfoot = vm.grid[ny][nx].clone();

        return Some((
            update_state(updated_agent, dy, dx, payload, next_underfoot, tension),
            Some((ny, nx)),
        ));
    }

    Some((
        update_state(updated_agent, -dy, -dx, payload, underfoot, tension),
        None,
    ))
}

pub fn apply_weave_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "║" {
        // Warp Sink: Reads signal from Self (deposited by Source '!' or others)
        // and updates register.
        if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
            vm.prologue_state.registers.insert((y, x), val.clone());
        }
    }
}

fn update_state(
    mut agent: PrologueAgent,
    dy: i64,
    dx: i64,
    payload: Value,
    underfoot: Value,
    tension: i64,
) -> PrologueAgent {
    agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![
            Value::Int(dy),
            Value::Int(dx),
            payload,
            underfoot,
            Value::Int(tension),
        ],
    );
    agent
}

fn find_warp(vm: &ChimeraVM, _y: usize, x: usize) -> Option<(usize, usize)> {
    // Scan column x for '║'
    for cy in 0..GRID_SIZE {
        if let Value::Str(s) = &vm.grid[cy][x] {
            if s == "║" {
                return Some((cy, x));
            }
        }
    }
    None
}

fn op_warp<F>(vm: &ChimeraVM, y: usize, x: usize, payload: &Value, op: F) -> Value
where
    F: Fn(i64, i64) -> i64,
{
    if let Some((wy, wx)) = find_warp(vm, y, x) {
        let warp_val = vm
            .prologue_state
            .registers
            .get(&(wy, wx))
            .unwrap_or(&Value::Int(0));
        if let (Value::Int(a), Value::Int(b)) = (payload, warp_val) {
            return Value::Int(op(*a, *b));
        }
    }
    payload.clone()
}

fn is_blocking(val: &Value) -> bool {
    // Block on walls '#' or other active agents (to prevent overlay)
    // Note: We don't block on runes like +, *, etc. as we move *over* them.
    if let Value::Str(s) = val {
        // We added '8' (Knot) which is a Rune, so we don't block on it.
        // We block on Walls (#), Agents, etc.
        return s == "#" || s == "@" || s == "K" || s == "H" || s == "C" || s == "ð" || s == "₣";
    }
    false
}
