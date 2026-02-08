#[cfg(feature = "nova")]
use crate::ast::JunctionType;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_mix(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., radius ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

                let mut ingredients = Vec::new();

                // If center already has a dish, keep it as base?
                // Or just mix everything including center.
                // Let's mix everything.

                // Collect and Clear neighbors (excluding center for now)
                for &(tx, ty) in &coords {
                    if tx == cx && ty == cy {
                        continue;
                    } // Skip center for a moment

                    let val = vm.grid[ty][tx].clone();
                    if !matches!(val, Value::Int(0)) {
                        ingredients.push(val);
                        vm.grid[ty][tx] = Value::Int(0);
                    }
                }

                // Add center content
                let center_val = vm.grid[cy][cx].clone();
                if let Value::Junction(JunctionType::Dish, mut existing) = center_val {
                    ingredients.append(&mut existing);
                } else if !matches!(center_val, Value::Int(0)) {
                    ingredients.push(center_val);
                }

                if !ingredients.is_empty() {
                    // Create Dish
                    vm.grid[cy][cx] = Value::Junction(JunctionType::Dish, ingredients);
                    vm.output
                        .push(format!("MIX: Created mixture at {},{}", cx, cy));
                } else {
                    vm.output.push("MIX: Nothing to mix".to_string());
                }

                vm.energy = vm.energy.saturating_sub(5);
            }
        } else {
            vm.output.push("Error: Type mismatch for mix".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for mix".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_brew(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., heat ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(heat) = val {
            let (cy, cx) = vm.context_loc;
            let center_val = vm.grid[cy][cx].clone();

            if let Value::Junction(JunctionType::Dish, ingredients) = center_val {
                // Check recipes
                let mut ingredient_names: Vec<String> = ingredients
                    .iter()
                    .filter_map(|v| match v {
                        Value::Str(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect();
                ingredient_names.sort();

                let mut product = None;
                let mut potency = heat;

                // Recipes
                if ingredient_names.contains(&"Water".to_string())
                    && ingredient_names.contains(&"Fire".to_string())
                {
                    if heat >= 10 {
                        product = Some("Acid");
                        potency += 10;
                    } else {
                        product = Some("Steam");
                    }
                } else if ingredient_names.contains(&"Life".to_string())
                    && ingredient_names.contains(&"Energy".to_string())
                {
                    if heat >= 5 {
                        product = Some("Elixir");
                        potency += 20;
                    }
                } else if ingredient_names.contains(&"Chaos".to_string())
                    && ingredient_names.contains(&"Entropy".to_string())
                {
                    product = Some("Mutagen");
                    potency += 30;
                } else if ingredient_names.contains(&"Earth".to_string())
                    && ingredient_names.contains(&"Water".to_string())
                {
                    product = Some("Mud");
                }

                if let Some(name) = product {
                    // Create Solution: [ "Solution", Name, Potency ]
                    let solution = Value::Junction(
                        JunctionType::Dish,
                        vec![
                            Value::Str("Solution".to_string()),
                            Value::Str(name.to_string()),
                            Value::Int(potency),
                        ],
                    );
                    vm.grid[cy][cx] = solution;
                    vm.output
                        .push(format!("BREW: Created {} (Potency {})", name, potency));
                    vm.energy = vm.energy.saturating_sub(10);
                } else {
                    vm.output.push("BREW: Failed (Inert mixture)".to_string());
                }
            } else {
                vm.output
                    .push("BREW: Current cell is not a mixture".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for brew".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for brew".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_splash(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., radius, dy, dx ]
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let r_val = vm.stack.pop().unwrap();

        if let (Value::Int(dx), Value::Int(dy), Value::Int(r)) = (x_val, y_val, r_val) {
            let (cy, cx) = vm.context_loc;
            let center_val = vm.grid[cy][cx].clone();

            // Check if it's a solution
            let mut is_solution = false;
            let mut solution_name = String::new();
            let mut potency = 0;

            if let Value::Junction(JunctionType::Dish, args) = &center_val {
                if args.len() >= 3 {
                    if let Value::Str(tag) = &args[0] {
                        if tag == "Solution" {
                            is_solution = true;
                            if let Value::Str(name) = &args[1] {
                                solution_name = name.clone();
                            }
                            if let Value::Int(p) = &args[2] {
                                potency = *p;
                            }
                        }
                    }
                }
            }

            if is_solution {
                // Remove potion from source
                vm.grid[cy][cx] = Value::Int(0);

                // Calculate target center
                if let Some((ty, tx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    let targets = vm.get_circular_coords(tx as i64, ty as i64, r);

                    vm.output
                        .push(format!("SPLASH: Threw {} at {},{}", solution_name, tx, ty));

                    match solution_name.as_str() {
                        "Acid" => {
                            for (tx, ty) in targets {
                                vm.grid[ty][tx] = Value::Int(0); // Destroy
                                                                 // Damage walls?
                                vm.membranes[ty][tx] = 0;
                            }
                            vm.output
                                .push("SPLASH: Acid melted everything!".to_string());
                        }
                        "Elixir" => {
                            for (tx, ty) in targets {
                                if let Value::Int(n) = &mut vm.grid[ty][tx] {
                                    *n = n.saturating_add(potency);
                                }
                            }
                            vm.energy = vm.energy.saturating_add(potency);
                            vm.output
                                .push("SPLASH: Elixir revitalized the area!".to_string());
                        }
                        "Mutagen" => {
                            for (tx, ty) in targets {
                                vm.mutagen_grid[ty][tx] =
                                    vm.mutagen_grid[ty][tx].saturating_add(potency);
                            }
                            // Trigger mutation?
                            vm.mutate();
                            vm.output.push("SPLASH: Mutagen released!".to_string());
                        }
                        "Steam" => {
                            for (tx, ty) in targets {
                                vm.moisture_grid[ty][tx] =
                                    vm.moisture_grid[ty][tx].saturating_add(potency);
                            }
                            vm.output.push("SPLASH: Steam cloud formed!".to_string());
                        }
                        _ => {
                            vm.output.push("SPLASH: No effect.".to_string());
                        }
                    }
                } else {
                    vm.output.push("SPLASH: Target out of bounds".to_string());
                }

                vm.energy = vm.energy.saturating_sub(5);
            } else {
                vm.output.push("SPLASH: Not a solution".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for splash".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for splash".to_string());
    }
    None
}
