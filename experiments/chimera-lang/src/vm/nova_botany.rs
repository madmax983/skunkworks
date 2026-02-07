#[cfg(feature = "nova")]
use crate::vm::{nova::Organelle, ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_plant(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: rules, axiom (top)
    if vm.stack.len() < 2 {
        vm.output
            .push("Error: Stack underflow for plant".to_string());
        return None;
    }

    let axiom_val = vm.stack.pop().unwrap();
    let rules_val = vm.stack.pop().unwrap();

    // Validate types
    if let Value::Str(_) = &axiom_val {
        if let Value::Str(_) | Value::Junction(_, _) = &rules_val {
            if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
                vm.output
                    .push("Error: Organelle limit exceeded".to_string());
                return None;
            }

            let (cy, cx) = vm.context_loc;

            // Create Seed Organelle
            // Stack: [ rules, axiom, index, turtle_stack_placeholder ]
            let stack = vec![
                rules_val,
                axiom_val,
                Value::Int(0),                                              // Index
                Value::Junction(crate::ast::JunctionType::All, Vec::new()), // Turtle Stack
            ];

            let organelle = Organelle {
                stack,
                ip: (0, 0), // Not used
                context_loc: (cy, cx),
                call_stack: Vec::new(),
                recursion_depth: 0,
                halted: false,
                kind: crate::vm::nova::OrganelleType::Seed,
                direction: (0, 1), // Default East
                ttl: Some(1000),   // Finite life
                name: "Procedural Seed".to_string(),
                traits: vec!["Fractal".to_string()],
                genome_id: 0,
            };

            vm.organelles.push(organelle);
            vm.energy = vm.energy.saturating_sub(20);
            vm.output
                .push(format!("PLANT: Sowed seed at {},{}", cx, cy));
        } else {
            vm.output
                .push("Error: Rules must be String or Junction".to_string());
        }
    } else {
        vm.output.push("Error: Axiom must be a String".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn tick_seed(vm: &mut ChimeraVM, organelle: &mut Organelle) -> bool {
    // Stack: [ rules, current_string, index, turtle_stack ] (Top)
    // Note: vm.stack contains the organelle's data due to swap in tick_organelle
    if vm.stack.len() < 4 {
        return false;
    }

    // Peek/Pop state
    let mut turtle_stack_val = vm.stack.pop().unwrap();
    let mut index_val = vm.stack.pop().unwrap();
    let mut string_val = vm.stack.pop().unwrap();
    let rules_val = vm.stack.last().unwrap().clone();

    let mut alive = true;
    let mut grew = false;

    if let (Value::Int(idx), Value::Str(s)) = (&mut index_val, &mut string_val) {
        let i = *idx as usize;
        if i >= s.len() {
            alive = false;
        } else {
            let c = s.chars().nth(i).unwrap();
            let mut next_idx = i + 1;

            let mut expansion = None;

            match &rules_val {
                Value::Str(rule_s) => {
                    expansion = check_rule(c, rule_s);
                }
                Value::Junction(_, vals) => {
                    for v in vals {
                        if let Value::Str(rule_s) = v {
                            if let Some(res) = check_rule(c, rule_s) {
                                expansion = Some(res);
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }

            if let Some(replacement) = expansion {
                if s.len() + replacement.len() < 1000 {
                    let mut new_s = String::new();
                    new_s.push_str(&s[0..i]);
                    new_s.push_str(&replacement);
                    new_s.push_str(&s[i + 1..]);
                    *s = new_s;

                    next_idx = i;
                    vm.energy = vm.energy.saturating_sub(1);
                } else {
                    interpret_char(vm, organelle, c, &mut turtle_stack_val);
                    grew = true;
                }
            } else {
                interpret_char(vm, organelle, c, &mut turtle_stack_val);
                grew = true;
            }

            *idx = next_idx as i64;
        }
    } else {
        alive = false;
    }

    vm.stack.push(string_val);
    vm.stack.push(index_val);
    vm.stack.push(turtle_stack_val);

    if grew {
        // Optional: consume extra energy?
    }

    alive
}

#[cfg(feature = "nova")]
fn check_rule(c: char, rule_s: &str) -> Option<String> {
    if let Some((lhs, rhs)) = rule_s.split_once('=') {
        if lhs.trim().starts_with(c) {
            return Some(rhs.trim().to_string());
        }
    } else if let Some((lhs, rhs)) = rule_s.split_once("->") {
        if lhs.trim().starts_with(c) {
            return Some(rhs.trim().to_string());
        }
    }
    None
}

#[cfg(feature = "nova")]
fn interpret_char(
    vm: &mut ChimeraVM,
    organelle: &mut Organelle,
    c: char,
    turtle_stack: &mut Value,
) {
    let (cy, cx) = vm.context_loc;
    let (dy, dx) = organelle.direction;

    match c {
        'F' | 'G' => {
            vm.grid[cy][cx] = Value::Str("#".to_string());
            if let Some((ny, nx)) =
                vm.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64)
            {
                if matches!(vm.grid[ny][nx], Value::Int(0)) {
                    vm.context_loc = (ny, nx);
                }
            }
        }
        'f' => {
            if let Some((ny, nx)) =
                vm.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64)
            {
                vm.context_loc = (ny, nx);
            }
        }
        '+' => {
            organelle.direction = (dx, -dy);
        }
        '-' => {
            organelle.direction = (-dx, dy);
        }
        '[' => {
            if let Value::Junction(_, list) = turtle_stack {
                let state = format!("{},{},{},{}", cy, cx, dy, dx);
                list.push(Value::Str(state));
            }
        }
        ']' => {
            if let Value::Junction(_, list) = turtle_stack {
                if let Some(Value::Str(state)) = list.pop() {
                    let parts: Vec<&str> = state.split(',').collect();
                    if parts.len() == 4 {
                        let y: usize = parts[0].parse().unwrap_or(cy);
                        let x: usize = parts[1].parse().unwrap_or(cx);
                        let ndy: i8 = parts[2].parse().unwrap_or(dy);
                        let ndx: i8 = parts[3].parse().unwrap_or(dx);
                        if y < crate::vm::GRID_SIZE && x < crate::vm::GRID_SIZE {
                            vm.context_loc = (y, x);
                            organelle.direction = (ndy, ndx);
                        }
                    }
                }
            }
        }
        'L' => {
            vm.grid[cy][cx] = Value::Str("♣".to_string());
        }
        _ => {}
    }
}
