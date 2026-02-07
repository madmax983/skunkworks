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

    if let (Value::Int(iters), Value::Str(axiom)) = (iterations_val, axiom_val) {
        if iters < 0 {
            vm.output
                .push("Error: Negative iterations for morph".to_string());
            return;
        }

        let mut rules = HashMap::new();

        let parse_rule = |s: &str, map: &mut HashMap<char, String>| {
            if let Some((k_str, v)) = s.split_once('=') {
                if let Some(k) = k_str.trim().chars().next() {
                    map.insert(k, v.trim().to_string());
                }
            } else if let Some((k_str, v)) = s.split_once("->") {
                if let Some(k) = k_str.trim().chars().next() {
                    map.insert(k, v.trim().to_string());
                }
            }
        };

        match rules_val {
            Value::Str(s) => {
                for part in s.split(',') {
                    parse_rule(part, &mut rules);
                }
            }
            Value::Junction(_, vals) => {
                for v in vals {
                    if let Value::Str(s) = v {
                        parse_rule(&s, &mut rules);
                    }
                }
            }
            _ => {
                vm.output
                    .push("Error: Invalid rules format for morph".to_string());
                return;
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
                vm.output.push("MORPH: Length limit exceeded".to_string());
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

    if let (Value::Int(start_x), Value::Int(start_y), Value::Str(instr)) =
        (x_val, y_val, instr_val)
    {
        let mut x = start_x;
        let mut y = start_y;
        // Direction: 0=E, 1=S, 2=W, 3=N (Clockwise)
        // Default to East (0) to match standard 2D grid logic
        let mut dir = 0;

        // State stack for [ ]
        let mut stack = Vec::new();

        let mut cells_written = 0;

        for c in instr.chars() {
            match c {
                'F' | 'G' => {
                    // Draw and Move
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("#".to_string());
                        cells_written += 1;
                    }

                    // Move
                    match dir {
                        0 => x += 1, // E
                        1 => y += 1, // S
                        2 => x -= 1, // W
                        3 => y -= 1, // N
                        _ => {}
                    }
                }
                'f' => {
                    // Move without drawing
                    match dir {
                        0 => x += 1,
                        1 => y += 1,
                        2 => x -= 1,
                        3 => y -= 1,
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
