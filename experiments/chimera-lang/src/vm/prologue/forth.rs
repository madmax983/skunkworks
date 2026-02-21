use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};

pub fn process_forth_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Extract State: [dy, dx, underfoot]
    let (mut dy, mut dx, underfoot) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 3 => {
            if let (Value::Int(y), Value::Int(x)) = (&list[0], &list[1]) {
                (*y as i64, *x as i64, list[2].clone())
            } else {
                (0, 1, Value::Int(0)) // Default East, Empty Underfoot
            }
        }
        _ => (0, 1, Value::Int(0)), // Default
    };

    // 2. Execute Logic (Process Underfoot)
    let cell = &underfoot;

    match cell {
        Value::Int(n) => {
            // Ignore 0 to avoid stack pollution on spawn?
            // No, 0 is a valid number.
            updated_agent.stack.push(Value::Int(*n));
        }
        Value::Str(s) => {
            match s.as_str() {
                // Movement Control
                "N" => { dy = -1; dx = 0; }
                "S" => { dy = 1; dx = 0; }
                "E" => { dy = 0; dx = 1; }
                "W" => { dy = 0; dx = -1; }

                // Stack Manipulation
                "\"" => { // Dup
                    if let Some(val) = updated_agent.stack.last() {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "_" => { // Drop
                    updated_agent.stack.pop();
                }
                "\\" => { // Swap
                    let len = updated_agent.stack.len();
                    if len >= 2 {
                        updated_agent.stack.swap(len - 1, len - 2);
                    }
                }

                // Math
                "+" => binary_op(&mut updated_agent.stack, |a, b| a + b),
                "-" => binary_op(&mut updated_agent.stack, |a, b| a - b),
                "*" => binary_op(&mut updated_agent.stack, |a, b| a * b),
                "/" => binary_op(&mut updated_agent.stack, |a, b| if b != 0 { a / b } else { 0 }),
                "%" => binary_op(&mut updated_agent.stack, |a, b| if b != 0 { a % b } else { 0 }),

                // IO
                "!" => { // Emit: Pop stack -> Signal Grid
                    if let Some(val) = updated_agent.stack.pop() {
                        // We can't write to signal_grid directly here as it's not passed mutably.
                        // But we have mutable access to VM? Yes.
                        // wait, `process_agents` takes `vm: &mut ChimeraVM`.
                        // But `process_agents` is called after `process_signal_propagation`.
                        // Writing to signal_grid now might be cleared next tick or used by other agents?
                        // `prepare_signals` clears it at start of next tick.
                        // So writing now is fine for other agents to see in THIS tick phase (Agents phase).
                        // Or next tick?
                        // Actually, `process_agents` runs after `process_sinks`.
                        // So signals emitted now won't be seen by Sinks until NEXT tick?
                        // `prepare_signals` clears `signal_grid` but applies `delayed_signals`.
                        // So we should write to `delayed_signals`.

                        // Accessing vm.prologue_state.delayed_signals
                        vm.prologue_state.delayed_signals[y][x] = Some(val);
                    }
                }
                "?" => { // Consume: Read Signal -> Push Stack
                    if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "." => { // Log: Pop -> Output
                    if let Some(val) = updated_agent.stack.pop() {
                        vm.output.push(format!("₣ {}: {}", updated_agent.stack.len(), val));
                    }
                }

                // Literals (Numbers in strings)
                _ => {
                    if let Ok(n) = s.parse::<i64>() {
                        updated_agent.stack.push(Value::Int(n));
                    } else if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                         updated_agent.stack.push(Value::Str(s[1..s.len()-1].to_string()));
                    }
                }
            }
        }
        _ => {}
    }

    // 4. Calculate Move Target
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        // Check if destination is blocked by another agent
        if let Value::Str(s) = &grid_snapshot[ny][nx] {
            if matches!(s.as_str(), "@" | "K" | "H" | "C" | "♻" | "♬" | "₣") {
                // Blocked - Stay put
                updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![Value::Int(dy), Value::Int(dx), underfoot],
                );
                return Some((updated_agent, None));
            }
        }

        // 1. Restore underfoot at current pos
        vm.grid[y][x] = underfoot;

        // 2. Capture new underfoot
        let next_val = vm.grid[ny][nx].clone();

        // 3. Update state
        updated_agent.state = Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(dy), Value::Int(dx), next_val],
        );

        return Some((updated_agent, Some((ny, nx))));
    }

    // If blocked/no move, we stay.
    updated_agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Int(dy), Value::Int(dx), underfoot],
    );

    Some((updated_agent, None))
}

fn binary_op<F>(stack: &mut Vec<Value>, op: F)
where
    F: Fn(i64, i64) -> i64,
{
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if let (Value::Int(ia), Value::Int(ib)) = (a, b) {
            stack.push(Value::Int(op(ia, ib)));
        } else {
            // Push back if fail? Or error?
            // Forth typically crashes or does weird stuff.
            // Let's just consume and push nothing (error).
        }
    }
}
