use super::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;

#[derive(Debug, Clone, Default)]
pub struct ReactorState {
    pub heat_grid: Vec<Vec<f32>>,
}

impl ReactorState {
    pub fn new() -> Self {
        Self {
            heat_grid: vec![vec![0.0; GRID_SIZE]; GRID_SIZE],
        }
    }
}

pub fn process_reactor(vm: &mut ChimeraVM) {
    let mut mutations = Vec::new();
    let mut swaps = Vec::new();

    // 1. Scan Phase
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let cell = &vm.grid[y][x];

            // Decay heat
            vm.reactor.heat_grid[y][x] *= 0.9;

            if let Value::Str(s) = cell {
                let neighbors = [
                    (-1, 0), (1, 0), (0, -1), (0, 1)
                ];

                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        match s.as_str() {
                            "^" => {
                                // Mercury: Catalyst (Increment)
                                mutations.push((ny, nx, 1));
                                vm.reactor.heat_grid[y][x] += 5.0;
                            },
                            "v" => {
                                // Sulfur: Acid (Decrement)
                                mutations.push((ny, nx, -1));
                                vm.reactor.heat_grid[y][x] += 5.0;
                            },
                            "@" => {
                                // Stone: Transmute
                                mutations.push((ny, nx, 100)); // Special code for gold
                                vm.reactor.heat_grid[y][x] += 20.0;
                            },
                            "~" => {
                                // Aether: Flow (Swap)
                                // Only swap with one random neighbor to avoid chaos
                                if rand::thread_rng().gen_bool(0.25) {
                                    swaps.push(((y, x), (ny, nx)));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // 2. Reaction Phase
    for (y, x, effect) in mutations {
        let val = &mut vm.grid[y][x];
        match val {
            Value::Int(n) => {
                if effect == 100 {
                    // Transmute Lead/Empty to Gold(100)
                    if *n == 0 || *n == 1 { // Lead is sometimes 1
                        *n = 100;
                        vm.output.push(format!("REACTOR: Transmutation at {},{}", x, y));
                    }
                } else {
                    *n = n.wrapping_add(effect);
                }
            },
            Value::Str(s) => {
                // Mutate string?
                if effect == 100 && s == "Lead" {
                    *val = Value::Str("Gold".to_string());
                }
            }
            _ => {}
        }
    }

    // 3. Flow Phase
    for (p1, p2) in swaps {
        let (y1, x1) = p1;
        let (y2, x2) = p2;
        // Check bounds again just in case (though normalize_coords handles it)
        if y1 < GRID_SIZE && x1 < GRID_SIZE && y2 < GRID_SIZE && x2 < GRID_SIZE {
             // To avoid conflicts, we swap clones or use a temp.
             // But since we are iterating, we need to be careful not to double swap.
             // For simplicity, just swap values.
             let temp = vm.grid[y1][x1].clone();
             vm.grid[y1][x1] = vm.grid[y2][x2].clone();
             vm.grid[y2][x2] = temp;
        }
    }
}
