use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};

/// Logic for the Pilot Agent (⚓).
///
/// The Pilot is a programmable cursor that moves across the grid, executing
/// the instructions found in the cells it traverses. It maintains a stack
/// and carries the value of the cell it is currently occupying ("underfoot"),
/// restoring it when it moves.
pub fn process_pilot_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Extract State: [dy, dx, underfoot]
    // Default: East (0, 1), Empty Underfoot
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

    // 2. Execute Logic (Process Underfoot Instruction)
    // The instruction is the value the agent is currently standing on.
    let instruction = &underfoot;

    match instruction {
        Value::Int(n) => {
            // Push Number
            updated_agent.stack.push(Value::Int(*n));
        }
        Value::Str(s) => {
            match s.as_str() {
                // Movement Control (Changes Direction)
                "N" => { dy = -1; dx = 0; }
                "S" => { dy = 1; dx = 0; }
                "E" => { dy = 0; dx = 1; }
                "W" => { dy = 0; dx = -1; }
                "[" => { // Turn Left (Relative to current facing?) or Absolute? Let's do Absolute rotation.
                    // N(-1,0) -> W(0,-1) -> S(1,0) -> E(0,1) -> N
                    let temp = dy;
                    dy = -dx;
                    dx = temp;
                }
                "]" => { // Turn Right
                    let temp = dy;
                    dy = dx;
                    dx = -temp;
                }
                "R" => { // Reverse
                    dy = -dy;
                    dx = -dx;
                }

                // Stack Ops
                "+" | "add" => binary_op(&mut updated_agent.stack, |a, b| a + b),
                "-" | "sub" => binary_op(&mut updated_agent.stack, |a, b| a - b),
                "*" | "mul" => binary_op(&mut updated_agent.stack, |a, b| a * b),
                "/" | "div" => binary_op(&mut updated_agent.stack, |a, b| if b != 0 { a / b } else { 0 }),
                "%" | "mod" => binary_op(&mut updated_agent.stack, |a, b| if b != 0 { a % b } else { 0 }),
                "&" | "and" => binary_op(&mut updated_agent.stack, |a, b| a & b),
                "|" | "or" => binary_op(&mut updated_agent.stack, |a, b| a | b),
                "^" | "xor" => binary_op(&mut updated_agent.stack, |a, b| a ^ b),
                "~" | "not" => {
                    if let Some(Value::Int(n)) = updated_agent.stack.pop() {
                        updated_agent.stack.push(Value::Int(!n));
                    }
                }
                "dup" => {
                    if let Some(val) = updated_agent.stack.last() {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "drop" => { updated_agent.stack.pop(); }
                "swap" => {
                    let len = updated_agent.stack.len();
                    if len >= 2 {
                        updated_agent.stack.swap(len - 1, len - 2);
                    }
                }

                // IO
                "!" | "emit" => {
                    if let Some(val) = updated_agent.stack.pop() {
                        // Write to Signal Grid (Bang)
                        vm.prologue_state.delayed_signals[y][x] = Some(val);
                    }
                }
                "?" | "read" => {
                    if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
                        updated_agent.stack.push(val.clone());
                    } else {
                        updated_agent.stack.push(Value::Int(0));
                    }
                }
                "." | "print" => {
                     if let Some(val) = updated_agent.stack.pop() {
                        vm.output.push(format!("⚓ Pilot: {}", val));
                     }
                }

                // Grid Interaction
                "g" | "get" => {
                    // Pop direction, Read neighbor
                    // 0=N, 1=E, 2=S, 3=W
                    if let Some(Value::Int(dir)) = updated_agent.stack.pop() {
                         let (oy, ox) = match dir {
                             0 => (-1, 0),
                             1 => (0, 1),
                             2 => (1, 0),
                             3 => (0, -1),
                             _ => (0, 0),
                         };
                         if let Some((ny, nx)) = normalize_coords(y as i64 + oy, x as i64 + ox) {
                             updated_agent.stack.push(grid_snapshot[ny][nx].clone());
                         } else {
                             updated_agent.stack.push(Value::Int(0));
                         }
                    }
                }
                "p" | "put" => {
                    // Pop val, Pop direction, Write neighbor
                    if updated_agent.stack.len() >= 2 {
                        let dir_val = updated_agent.stack.pop().unwrap();
                        let val = updated_agent.stack.pop().unwrap();
                        if let Value::Int(dir) = dir_val {
                            let (oy, ox) = match dir {
                                0 => (-1, 0),
                                1 => (0, 1),
                                2 => (1, 0),
                                3 => (0, -1),
                                _ => (0, 0),
                            };
                            if let Some((ny, nx)) = normalize_coords(y as i64 + oy, x as i64 + ox) {
                                // Only write if empty or simple type?
                                // Let's allow overwrite for now, except special agents
                                let target_cell = &vm.grid[ny][nx];
                                let is_special = match target_cell {
                                    Value::Str(s) => matches!(s.as_str(), "@" | "K" | "H" | "C" | "⚓"),
                                    _ => false,
                                };
                                if !is_special {
                                    vm.grid[ny][nx] = val;
                                }
                            }
                        }
                    }
                }

                // Elektra Integration
                #[cfg(feature = "elektra")]
                "v" | "volts" => {
                    // Read voltage at current cell
                    let voltage = vm.voltage_grid[y][x];
                    updated_agent.stack.push(Value::Int(voltage as i64));
                }

                // Literals
                _ => {
                    if let Ok(n) = s.parse::<i64>() {
                        updated_agent.stack.push(Value::Int(n));
                    } else if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
                         updated_agent.stack.push(Value::Str(s[1..s.len()-1].to_string()));
                    } else {
                        // Unknown string instruction - treat as literal string push
                        updated_agent.stack.push(Value::Str(s.clone()));
                    }
                }
            }
        }
        _ => {}
    }

    // 3. Calculate Move
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        // Check Collision
         if let Value::Str(s) = &grid_snapshot[ny][nx] {
            if matches!(s.as_str(), "@" | "K" | "H" | "C" | "♻" | "♬" | "₣" | "⚓" | "P" | "ζ") {
                // Blocked - Stay put
                updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![Value::Int(dy), Value::Int(dx), underfoot.clone()],
                );
                return Some((updated_agent, None));
            }
        }

        // Restore OLD Underfoot to CURRENT Grid Pos
        vm.grid[y][x] = underfoot.clone();

        // Capture NEW Underfoot from NEW Grid Pos (Snapshot? No, from VM Grid ideally, but snapshot is safer for consistency)
        // Actually, logic dictates we move onto what was there at start of tick.
        let next_val = grid_snapshot[ny][nx].clone();

        // Update State
        updated_agent.state = Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(dy), Value::Int(dx), next_val],
        );

        return Some((updated_agent, Some((ny, nx))));
    }

    // Blocked by Edge - Stay put
    updated_agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Int(dy), Value::Int(dx), underfoot.clone()],
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
            // Type Mismatch - Push back or Fail? Push 0
            stack.push(Value::Int(0));
        }
    }
}
