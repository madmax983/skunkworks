#[cfg(feature = "nova")]
use crate::ast::JunctionType;
#[cfg(feature = "nova")]
use crate::vm::{iterate_circle, ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_mix(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., radius ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let mut ingredients = Vec::new();

                // If center already has a dish, keep it as base?
                // Or just mix everything including center.
                // Let's mix everything.

                // Collect and Clear neighbors (excluding center for now)
                iterate_circle(
                    #[cfg(feature = "nova")]
                    vm.topology,
                    cx as i64,
                    cy as i64,
                    r,
                    |tx, ty| {
                        if tx == cx && ty == cy {
                            return;
                        } // Skip center for a moment

                        let val = vm.grid[ty][tx].clone();
                        if !matches!(val, Value::Int(0)) {
                            ingredients.push(val);
                            vm.grid[ty][tx] = Value::Int(0);
                        }
                    },
                );

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
                    vm.output
                        .push(format!("SPLASH: Threw {} at {},{}", solution_name, tx, ty));

                    match solution_name.as_str() {
                        "Acid" => {
                            iterate_circle(
                                #[cfg(feature = "nova")]
                                vm.topology,
                                tx as i64,
                                ty as i64,
                                r,
                                |tx, ty| {
                                    vm.grid[ty][tx] = Value::Int(0); // Destroy
                                                                     // Damage walls?
                                    vm.membranes[ty][tx] = 0;
                                },
                            );
                            vm.output
                                .push("SPLASH: Acid melted everything!".to_string());
                        }
                        "Elixir" => {
                            iterate_circle(
                                #[cfg(feature = "nova")]
                                vm.topology,
                                tx as i64,
                                ty as i64,
                                r,
                                |tx, ty| {
                                    if let Value::Int(n) = &mut vm.grid[ty][tx] {
                                        *n = n.saturating_add(potency);
                                    }
                                },
                            );
                            vm.energy = vm.energy.saturating_add(potency);
                            vm.output
                                .push("SPLASH: Elixir revitalized the area!".to_string());
                        }
                        "Mutagen" => {
                            iterate_circle(
                                #[cfg(feature = "nova")]
                                vm.topology,
                                tx as i64,
                                ty as i64,
                                r,
                                |tx, ty| {
                                    vm.mutagen_grid[ty][tx] =
                                        vm.mutagen_grid[ty][tx].saturating_add(potency);
                                },
                            );
                            // Trigger mutation?
                            vm.mutate();
                            vm.output.push("SPLASH: Mutagen released!".to_string());
                        }
                        "Steam" => {
                            iterate_circle(
                                #[cfg(feature = "nova")]
                                vm.topology,
                                tx as i64,
                                ty as i64,
                                r,
                                |tx, ty| {
                                    vm.moisture_grid[ty][tx] =
                                        vm.moisture_grid[ty][tx].saturating_add(potency);
                                },
                            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (8, 8);
        vm
    }

    #[test]
    fn test_mix_creation() {
        let mut vm = setup_vm();

        // Setup neighbors
        // Center is (8,8)
        vm.grid[8][9] = Value::Str("Water".to_string());
        vm.grid[7][8] = Value::Str("Fire".to_string());

        // Push radius 2
        vm.stack.push(Value::Int(2));

        exec_mix(&mut vm);

        // Check center
        match &vm.grid[8][8] {
            Value::Junction(JunctionType::Dish, ingredients) => {
                let has_water = ingredients
                    .iter()
                    .any(|v| matches!(v, Value::Str(s) if s == "Water"));
                let has_fire = ingredients
                    .iter()
                    .any(|v| matches!(v, Value::Str(s) if s == "Fire"));
                assert!(has_water, "Mixture should contain Water");
                assert!(has_fire, "Mixture should contain Fire");
            }
            _ => panic!("Expected Dish junction at center, got {:?}", vm.grid[8][8]),
        }

        // Check neighbors cleared
        assert_eq!(vm.grid[8][9], Value::Int(0));
        assert_eq!(vm.grid[7][8], Value::Int(0));
    }

    #[test]
    fn test_mix_radius_zero() {
        let mut vm = setup_vm();
        vm.grid[8][9] = Value::Str("Water".to_string());
        vm.stack.push(Value::Int(0));

        exec_mix(&mut vm);

        // Center should be unchanged (0)
        assert_eq!(vm.grid[8][8], Value::Int(0));
        // Neighbor should be unchanged
        assert_eq!(vm.grid[8][9], Value::Str("Water".to_string()));
    }

    #[test]
    fn test_brew_acid() {
        let mut vm = setup_vm();
        let ingredients = vec![
            Value::Str("Water".to_string()),
            Value::Str("Fire".to_string()),
        ];
        vm.grid[8][8] = Value::Junction(JunctionType::Dish, ingredients);

        // Heat 10
        vm.stack.push(Value::Int(10));

        exec_brew(&mut vm);

        // Check result
        if let Value::Junction(JunctionType::Dish, args) = &vm.grid[8][8] {
            assert_eq!(args[0], Value::Str("Solution".to_string()));
            assert_eq!(args[1], Value::Str("Acid".to_string()));
            assert_eq!(args[2], Value::Int(20)); // 10 heat + 10 bonus
        } else {
            panic!("Expected Solution Dish");
        }
    }

    #[test]
    fn test_brew_steam() {
        let mut vm = setup_vm();
        let ingredients = vec![
            Value::Str("Water".to_string()),
            Value::Str("Fire".to_string()),
        ];
        vm.grid[8][8] = Value::Junction(JunctionType::Dish, ingredients);

        // Heat 5 (less than 10)
        vm.stack.push(Value::Int(5));

        exec_brew(&mut vm);

        if let Value::Junction(JunctionType::Dish, args) = &vm.grid[8][8] {
            assert_eq!(args[1], Value::Str("Steam".to_string()));
        } else {
            panic!("Expected Solution Dish");
        }
    }

    #[test]
    fn test_brew_elixir() {
        let mut vm = setup_vm();
        let ingredients = vec![
            Value::Str("Life".to_string()),
            Value::Str("Energy".to_string()),
        ];
        vm.grid[8][8] = Value::Junction(JunctionType::Dish, ingredients);

        // Heat 5
        vm.stack.push(Value::Int(5));

        exec_brew(&mut vm);

        if let Value::Junction(JunctionType::Dish, args) = &vm.grid[8][8] {
            assert_eq!(args[1], Value::Str("Elixir".to_string()));
            assert_eq!(args[2], Value::Int(25)); // 5 + 20
        } else {
            panic!("Expected Solution Dish");
        }
    }

    #[test]
    fn test_splash_acid() {
        let mut vm = setup_vm();

        // Setup Solution
        let solution = Value::Junction(
            JunctionType::Dish,
            vec![
                Value::Str("Solution".to_string()),
                Value::Str("Acid".to_string()),
                Value::Int(50),
            ],
        );
        vm.grid[8][8] = solution;

        // Setup target area
        // Target 8,9 (East 1)
        vm.grid[8][9] = Value::Int(100);

        // Splash: radius 1, dy 0, dx 1
        vm.stack.push(Value::Int(1)); // r
        vm.stack.push(Value::Int(0)); // y
        vm.stack.push(Value::Int(1)); // x

        exec_splash(&mut vm);

        // Center should be empty (thrown)
        assert_eq!(vm.grid[8][8], Value::Int(0));
        // Target should be destroyed
        assert_eq!(vm.grid[8][9], Value::Int(0));
    }

    #[test]
    fn test_splash_elixir() {
        let mut vm = setup_vm();

        // Setup Solution
        let solution = Value::Junction(
            JunctionType::Dish,
            vec![
                Value::Str("Solution".to_string()),
                Value::Str("Elixir".to_string()),
                Value::Int(10),
            ],
        );
        vm.grid[8][8] = solution;

        // Setup target
        vm.grid[8][9] = Value::Int(5);

        // Splash: radius 1, dy 0, dx 1
        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(1));

        exec_splash(&mut vm);

        // Target should be increased
        assert_eq!(vm.grid[8][9], Value::Int(15));
    }

    #[test]
    fn test_splash_out_of_bounds() {
        let mut vm = setup_vm();
        // Use Plane topology to enforce bounds checking
        vm.topology = crate::vm::Topology::Plane;

        let solution = Value::Junction(
            JunctionType::Dish,
            vec![
                Value::Str("Solution".to_string()),
                Value::Str("Acid".to_string()),
                Value::Int(50),
            ],
        );
        vm.grid[8][8] = solution;

        // Splash far away: radius 1, dy 100, dx 100
        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Int(100));
        vm.stack.push(Value::Int(100));

        exec_splash(&mut vm);

        // Center consumed
        assert_eq!(vm.grid[8][8], Value::Int(0));
        // Error logged
        assert!(
            vm.output.iter().any(|s| s.contains("Target out of bounds")),
            "Output was: {:?}",
            vm.output
        );
    }
}
