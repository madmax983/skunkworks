#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use std::collections::HashMap;

#[cfg(feature = "nova")]
pub fn exec_morph(vm: &mut ChimeraVM) {
    // Stack: axiom, rules, iterations (top)
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for morph".to_string());
        return;
    }

    let iterations_val = vm.stack.pop().unwrap();
    let rules_val = vm.stack.pop().unwrap();
    let axiom_val = vm.stack.pop().unwrap();

    if let (Value::Int(iters), Value::Str(rules_str), Value::Str(axiom)) =
        (iterations_val, rules_val, axiom_val)
    {
        if iters < 0 {
            vm.output
                .push("Error: Negative iterations for morph".to_string());
            return;
        }

        // Parse Rules: "A=AB,B=A"
        let mut rules = HashMap::new();
        for rule in rules_str.split(',') {
            if let Some((k, v)) = rule.split_once('=') {
                if k.len() == 1 {
                    rules.insert(k.chars().next().unwrap(), v.to_string());
                }
            }
        }

        let mut current = axiom;
        let safe_iters = iters.min(10); // Cap iterations to prevent explosion

        for _ in 0..safe_iters {
            let mut next = String::new();
            for c in current.chars() {
                if let Some(replacement) = rules.get(&c) {
                    next.push_str(replacement);
                } else {
                    next.push(c);
                }
            }
            current = next;
            if current.len() > 10000 {
                // Safety cap
                break;
            }
        }

        vm.stack.push(Value::Str(current.clone()));
        vm.energy = vm.energy.saturating_sub((current.len() / 10) as i64);
        vm.output
            .push(format!("MORPH: Expanded to length {}", current.len()));
    } else {
        vm.output.push("Error: Type mismatch for morph".to_string());
    }
}

#[cfg(feature = "nova")]
pub fn exec_grow(vm: &mut ChimeraVM) {
    // Stack: instructions, start_y, start_x (top)
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for grow".to_string());
        return;
    }

    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    let instr_val = vm.stack.pop().unwrap();

    if let (Value::Int(start_x), Value::Int(start_y), Value::Str(instr)) = (x_val, y_val, instr_val)
    {
        let mut x = start_x;
        let mut y = start_y;
        // Direction: 0=N, 1=E, 2=S, 3=W (Clockwise)
        // Default L-Systems usually start pointing Up (North).
        // Let's assume North is -Y.
        let mut dir = 0;

        // State stack for [ ]
        let mut stack = Vec::new();

        let mut cells_written = 0;

        for c in instr.chars() {
            match c {
                'F' | 'G' => {
                    // Draw and Move
                    // Write to current location
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        // Write a structure marker.
                        // Maybe Value::Str("#")?
                        vm.grid[ny][nx] = Value::Str("#".to_string());
                        cells_written += 1;
                    }

                    // Move
                    match dir {
                        0 => y -= 1, // N
                        1 => x += 1, // E
                        2 => y += 1, // S
                        3 => x -= 1, // W
                        _ => {}
                    }
                }
                '+' => {
                    // Turn Right
                    dir = (dir + 1) % 4;
                }
                '-' => {
                    // Turn Left
                    dir = (dir + 3) % 4;
                }
                '[' => {
                    stack.push((x, y, dir));
                }
                ']' => {
                    if let Some((sx, sy, sdir)) = stack.pop() {
                        x = sx;
                        y = sy;
                        dir = sdir;
                    }
                }
                _ => {}
            }
        }

        vm.energy = vm.energy.saturating_sub(cells_written);
        vm.output.push(format!(
            "GROW: Built structure with {} cells",
            cells_written
        ));
    } else {
        vm.output.push("Error: Type mismatch for grow".to_string());
    }
}
