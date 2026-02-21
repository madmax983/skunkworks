#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use std::collections::HashMap;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub birth: Vec<u8>,
    pub survival: Vec<u8>,
}

#[cfg(feature = "nova")]
impl Default for Rule {
    fn default() -> Self {
        Self {
            birth: vec![3],
            survival: vec![2, 3],
        }
    }
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct GardenState {
    pub rules: HashMap<i64, Rule>,
}

#[cfg(feature = "nova")]
impl GardenState {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        // Species 1 uses standard Life
        rules.insert(1, Rule::default());
        Self { rules }
    }
}

#[cfg(feature = "nova")]
fn parse_life_rule(rule: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let parts: Vec<&str> = rule.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let parse_part = |s: &str, prefix: char| -> Vec<u8> {
        let s = s.trim();
        let nums = if s.starts_with(prefix) { &s[1..] } else { s };

        let mut digits = Vec::new();
        for c in nums.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            }
        }
        digits
    };

    let birth = parse_part(parts[0], 'B');
    let survival = parse_part(parts[1], 'S');

    Some((birth, survival))
}

#[cfg(feature = "nova")]
pub fn exec_sow(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: rule_string, species_id (top)
    if vm.stack.len() >= 2 {
        let id_val = vm.stack.pop().unwrap();
        let rule_val = vm.stack.pop().unwrap();

        if let (Value::Int(id), Value::Str(rule_str)) = (id_val, rule_val) {
            if let Some((b, s)) = parse_life_rule(&rule_str) {
                let rule = Rule {
                    birth: b,
                    survival: s,
                };
                vm.garden.rules.insert(id, rule);
                vm.output
                    .push(format!("SOW: Species {} rule set to {}", id, rule_str));
            } else {
                vm.output
                    .push(format!("SOW: Invalid rule format '{}'", rule_str));
            }
        } else {
            vm.output.push("Error: Type mismatch for sow".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for sow".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_harvest(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: radius (top)
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let mut s = String::new();
                crate::vm::iterate_circle(
                    #[cfg(feature = "nova")]
                    vm.topology,
                    cx as i64,
                    cy as i64,
                    r as i64,
                    |x, y| {
                        if let Value::Int(v) = &vm.grid[y][x] {
                            s.push_str(&v.to_string());
                        } else {
                            s.push('?');
                        }
                        s.push(',');
                    },
                );
                vm.stack.push(Value::Str(s));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.stack.push(Value::Str("".to_string()));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for harvest".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for harvest".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_evolve(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let rows = vm.grid.len();
    let cols = if rows > 0 { vm.grid[0].len() } else { 0 };
    let mut next_grid = vm.grid.clone();

    for y in 0..rows {
        for x in 0..cols {
            let current_val = match &vm.grid[y][x] {
                Value::Int(n) => n,
                _ => &0,
            };

            // Count neighbors per species
            let mut neighbor_counts: HashMap<i64, u8> = HashMap::new();

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if let Value::Int(n) = vm.grid[ny][nx] {
                            if n > 0 {
                                *neighbor_counts.entry(n).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            if *current_val > 0 {
                // Survival
                let rule = vm.garden.rules.get(&current_val);
                let count = neighbor_counts.get(&current_val).unwrap_or(&0);

                let survives = if let Some(r) = rule {
                    r.survival.contains(&count)
                } else if *current_val == 1 {
                    [2, 3].contains(&count)
                } else {
                    false
                };

                if !survives {
                    next_grid[y][x] = Value::Int(0);
                }
            } else {
                // Birth
                let mut born_species = None;

                for (&species, &count) in &neighbor_counts {
                    let rule = vm.garden.rules.get(&species);
                    let born = if let Some(r) = rule {
                        r.birth.contains(&count)
                    } else if species == 1 {
                        count == 3
                    } else {
                        false
                    };

                    if born {
                        // Tie-break: Largest ID wins
                        if let Some(existing) = born_species {
                            if species > existing {
                                born_species = Some(species);
                            }
                        } else {
                            born_species = Some(species);
                        }
                    }
                }

                if let Some(s) = born_species {
                    next_grid[y][x] = Value::Int(s);
                }
            }
        }
    }

    vm.grid = next_grid;
    vm.energy = vm.energy.saturating_sub(20);
    None
}
