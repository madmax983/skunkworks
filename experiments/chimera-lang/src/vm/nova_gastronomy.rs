#[cfg(feature = "nova")]
use crate::ast::JunctionType;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_gastronomy_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[crate::ast::Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Cook => exec_cook(vm),
        OpCode::Spice => exec_spice(vm),
        OpCode::Savor => exec_savor(vm),
        OpCode::Cultivate => exec_cultivate(vm),
        OpCode::Banquet => exec_banquet(vm),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn exec_cook(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(count) = val {
            if count >= 0 {
                let c = count as usize;
                if vm.stack.len() >= c {
                    let mut ingredients = Vec::new();
                    for _ in 0..c {
                        ingredients.push(vm.stack.pop().unwrap());
                    }
                    // Ingredients popped in reverse order (LIFO), so first popped is last arg.
                    // Usually for a junction [a, b, c], we want [a, b, c].
                    // Stack: ..., a, b, c (top).
                    // Pop c, b, a.
                    // Vector: [c, b, a].
                    // Reverse to get [a, b, c].
                    ingredients.reverse();

                    vm.stack
                        .push(Value::Junction(JunctionType::Dish, ingredients));
                    vm.energy = vm.energy.saturating_sub(5);
                    vm.output
                        .push(format!("COOK: Created Dish with {} ingredients", c));
                } else {
                    vm.output
                        .push("Error: Stack underflow for cook ingredients".to_string());
                }
            } else {
                vm.output.push("Error: Invalid count for cook".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for cook".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cook".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_spice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let spice_val = vm.stack.pop().unwrap();
        let dish_val = vm.stack.pop().unwrap();

        if let (Value::Junction(JunctionType::Dish, mut ingredients), Value::Str(spice)) =
            (dish_val, spice_val)
        {
            if ingredients.len() < crate::vm::MAX_JUNCTION_SIZE {
                ingredients.push(Value::Str(spice.clone()));
                vm.stack
                    .push(Value::Junction(JunctionType::Dish, ingredients));
                vm.energy = vm.energy.saturating_sub(2);
                vm.output.push(format!("SPICE: Added '{}'", spice));
            } else {
                vm.output.push("Error: Dish is full".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for spice".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for spice".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_savor(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Junction(JunctionType::Dish, ingredients) => {
                let mut energy_gain = 0;
                let mut buffs = Vec::new();

                // Analyze ingredients
                for item in ingredients {
                    match item {
                        Value::Int(n) => {
                            energy_gain += n;
                        }
                        Value::Str(s) => {
                            // Flavor Text / Buffs
                            match s.as_str() {
                                "Spicy" | "spicy" => buffs.push("Spicy"),
                                "Sweet" | "sweet" => buffs.push("Sweet"),
                                "Sour" | "sour" => buffs.push("Sour"),
                                "Bitter" | "bitter" => buffs.push("Bitter"),
                                "Umami" | "umami" => buffs.push("Umami"),
                                _ => energy_gain += s.len() as i64,
                            }
                        }
                        Value::Junction(_, _) => {
                            energy_gain += 10; // Complexity bonus
                        }
                        Value::Superposition(_) => {
                            energy_gain += 50; // Quantum flavor
                        }
                    }
                }

                // Apply Buff Multipliers
                if buffs.contains(&"Sweet") {
                    energy_gain *= 2;
                }

                // Apply Buffs to VM
                for buff in buffs {
                    // Duration: 20 ticks
                    vm.buffs.insert(buff.to_string(), 20);
                    vm.output.push(format!("SAVOR: Gained Buff '{}'", buff));
                }

                vm.energy = vm.energy.saturating_add(energy_gain);
                vm.output.push(format!(
                    "SAVOR: Consumed Dish, gained {} energy",
                    energy_gain
                ));
            }
            _ => {
                // Regular consume behavior for non-dishes
                // Actually, Savor should fail or just behave like consume?
                // Let's make it strict.
                vm.output
                    .push("Error: Savor requires a Dish (use Consume for others)".to_string());
            }
        }
    } else {
        vm.output
            .push("Error: Stack underflow for savor".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_cultivate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let val = &mut vm.grid[cy][cx];

    match val {
        Value::Int(n) => {
            *n = n.saturating_add(1);
            vm.output.push(format!("CULTIVATE: Growth to {}", n));
        }
        Value::Str(s) => {
            if s == "seed" {
                *val = Value::Str("sprout".to_string());
            } else if s == "sprout" {
                *val = Value::Str("plant".to_string());
            } else if s == "plant" {
                *val = Value::Str("fruit".to_string());
            } else {
                s.push('+'); // Genetically modify?
            }
            vm.output.push("CULTIVATE: Evolved string".to_string());
        }
        _ => {}
    }
    vm.energy = vm.energy.saturating_sub(5);
    None
}

#[cfg(feature = "nova")]
fn exec_banquet(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let amount_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();

        if let (Value::Int(r), Value::Int(amount)) = (radius_val, amount_val) {
            if r > 0 && amount > 0 {
                if vm.energy >= amount {
                    vm.energy -= amount;
                    let (cy, cx) = vm.context_loc;
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                    let count = coords.len();

                    if count > 0 {
                        let per_cell = amount / count as i64;
                        for (tx, ty) in coords {
                            if let Value::Int(n) = &mut vm.grid[ty][tx] {
                                *n = n.saturating_add(per_cell);
                            } else {
                                // Plant energy?
                                vm.grid[ty][tx] = Value::Int(per_cell);
                            }
                        }
                        vm.output
                            .push(format!("BANQUET: Distributed {} energy", amount));
                    }
                } else {
                    vm.output.push("BANQUET: Insufficient energy".to_string());
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for banquet".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for banquet".to_string());
    }
    None
}
