use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};

/// Processes the behavior of the Pilot Agent (`⚓`).
///
/// The Pilot acts as a programmable cursor. It executes the rune it is currently
/// standing on ("underfoot"), carries a stack, and moves based on its internal velocity.
/// Crucially, it restores the grid cell it leaves behind, allowing non-destructive traversal.
pub fn process_pilot_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Extract State: [dy, dx, underfoot]
    // Default state: Moving East (0, 1), Underfoot is Empty (0)
    let (mut dy, mut dx, underfoot) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 3 => {
            if let (Value::Int(y), Value::Int(x)) = (&list[0], &list[1]) {
                (*y as i64, *x as i64, list[2].clone())
            } else {
                (0, 1, Value::Int(0))
            }
        }
        _ => (0, 1, Value::Int(0)),
    };

    // 2. Execute Instruction (The rune underfoot)
    let instruction = &underfoot;
    let mut skipped = false;

    match instruction {
        Value::Int(n) => {
            // Push number to stack
            updated_agent.stack.push(Value::Int(*n));
        }
        Value::Str(s) => {
            match s.as_str() {
                // Movement Control
                "N" => { dy = -1; dx = 0; }
                "S" => { dy = 1; dx = 0; }
                "E" => { dy = 0; dx = 1; }
                "W" => { dy = 0; dx = -1; }

                // Stack Arithmetic
                "+" => binary_op(&mut updated_agent.stack, |a, b| a + b),
                "-" => binary_op(&mut updated_agent.stack, |a, b| a - b),
                "*" => binary_op(&mut updated_agent.stack, |a, b| a * b),
                "/" | "%" => binary_op(&mut updated_agent.stack, |a, b| if b != 0 { a % b } else { 0 }), // Using Mod for now as / is scarce
                "&" => binary_op(&mut updated_agent.stack, |a, b| a & b),
                "|" => binary_op(&mut updated_agent.stack, |a, b| a | b),
                "^" => binary_op(&mut updated_agent.stack, |a, b| a ^ b),

                // IO
                "!" => {
                    // Bang: Emit top of stack to Signal Grid at current location
                    if let Some(val) = updated_agent.stack.pop() {
                        vm.prologue_state.signal_grid[y][x] = Some(val);
                    }
                }
                "?" => {
                    // Query: Read Signal Grid at current location -> Push to Stack
                    if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "g" => {
                    // Get: Pop (dy, dx), Push grid[y+dy][x+dx]
                    if updated_agent.stack.len() >= 2 {
                        let off_x = updated_agent.stack.pop().unwrap();
                        let off_y = updated_agent.stack.pop().unwrap();
                        if let (Value::Int(oy), Value::Int(ox)) = (off_y, off_x) {
                            if let Some((ny, nx)) = normalize_coords(y as i64 + oy, x as i64 + ox) {
                                let val = vm.grid[ny][nx].clone();
                                updated_agent.stack.push(val);
                            } else {
                                updated_agent.stack.push(Value::Int(0));
                            }
                        }
                    }
                }
                "p" => {
                    // Put: Pop val, dy, dx -> Write val to grid[y+dy][x+dx]
                     if updated_agent.stack.len() >= 3 {
                        let off_x = updated_agent.stack.pop().unwrap();
                        let off_y = updated_agent.stack.pop().unwrap();
                        let val = updated_agent.stack.pop().unwrap();
                        if let (Value::Int(oy), Value::Int(ox)) = (off_y, off_x) {
                            if let Some((ny, nx)) = normalize_coords(y as i64 + oy, x as i64 + ox) {
                                // Prevent overwriting self or other agents?
                                // For now, just write.
                                vm.grid[ny][nx] = val;
                            }
                        }
                     }
                }
                "@" => {
                    // Where: Push y, x
                    updated_agent.stack.push(Value::Int(y as i64));
                    updated_agent.stack.push(Value::Int(x as i64));
                }
                "~" => {
                    // Skip: Jump over next cell
                    skipped = true;
                }
                "[" => {
                    // Turn Left (Relative to current heading)
                    // N(-1,0) -> W(0,-1) -> S(1,0) -> E(0,1) -> N
                    let (ndy, ndx) = (-dx, dy);
                    dy = ndy; dx = ndx;
                }
                "]" => {
                    // Turn Right
                    // N(-1,0) -> E(0,1) -> S(1,0) -> W(0,-1) -> N
                    let (ndy, ndx) = (dx, -dy);
                    dy = ndy; dx = ndx;
                }

                // Default: Push string literal if enclosed in quotes, or ignore
                _ => {
                     if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                        updated_agent
                            .stack
                            .push(Value::Str(s[1..s.len() - 1].to_string()));
                    } else if let Ok(n) = s.parse::<i64>() {
                         updated_agent.stack.push(Value::Int(n));
                    }
                }
            }
        }
        _ => {}
    }

    // 3. Calculate Target
    let step = if skipped { 2 } else { 1 };
    let target_dy = dy * step;
    let target_dx = dx * step;

    let target = normalize_coords(y as i64 + target_dy, x as i64 + target_dx);

    if let Some((ny, nx)) = target {
        // Check for collision with other agents
        if let Value::Str(s) = &grid_snapshot[ny][nx] {
             if matches!(s.as_str(), "@" | "K" | "H" | "C" | "♻" | "♬" | "₣" | "⚓") {
                // Blocked
                 updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![Value::Int(dy), Value::Int(dx), underfoot],
                );
                return Some((updated_agent, None));
             }
        }

        // 4. Move and Restore

        // Restore: Write the old 'underfoot' value back to current position
        // This effectively erases the '⚓' from the grid at (y,x) and puts back what was there.
        // NOTE: process_agents in mod.rs will try to clear (y,x) to 0, but only if grid[y][x] == "⚓".
        // By overwriting it here with 'underfoot', we prevent process_agents from clearing it.
        vm.grid[y][x] = underfoot;

        // Capture new underfoot from destination
        // We must read from VM grid (live) or snapshot?
        // Snapshot is safer for concurrent moves, but if we want to see effects of other agents...
        // Snapshot is standard.
        // BUT, what if destination is empty (0)?
        let next_val = vm.grid[ny][nx].clone();

        // Update State
        updated_agent.state = Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(dy), Value::Int(dx), next_val],
        );

        return Some((updated_agent, Some((ny, nx))));
    }

    // If out of bounds or blocked (target is None), stay put.
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
             // Silently fail for type mismatch
        }
    }
}
