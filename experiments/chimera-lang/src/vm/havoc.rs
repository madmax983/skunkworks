use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HavocEngine {
    pub rate: f64,
    pub scope: u8, // 1=Mem, 2=Stack, 4=Exec
}

impl HavocEngine {
    pub fn new() -> Self {
        Self {
            rate: 0.0,
            scope: 0,
        }
    }

    pub fn tick(&mut self, vm: &mut ChimeraVM) {
        if self.rate <= 0.0 || self.scope == 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.rate.clamp(0.0, 1.0)) {
            self.inject_fault(vm);
        }
    }

    fn inject_fault(&mut self, vm: &mut ChimeraVM) {
        let mut rng = rand::thread_rng();
        // Determine available scopes
        let mut scopes = Vec::new();
        if (self.scope & 1) != 0 { scopes.push(1); }
        if (self.scope & 2) != 0 { scopes.push(2); }
        if (self.scope & 4) != 0 { scopes.push(4); }

        if scopes.is_empty() { return; }

        let chosen_scope = scopes[rng.gen_range(0..scopes.len())];

        match chosen_scope {
            1 => {
                // Memory Fault
                let y = rng.gen_range(0..crate::vm::GRID_SIZE);
                let x = rng.gen_range(0..crate::vm::GRID_SIZE);
                let old_val = vm.grid[y][x].clone();

                if let Value::Int(mut n) = old_val {
                    // Bitflip
                    let bit = rng.gen_range(0..64);
                    n ^= 1 << bit;
                    vm.grid[y][x] = Value::Int(n);
                    vm.output.push(format!("HAVOC: Bitflip at {},{}", x, y));
                } else if let Value::Str(s) = old_val {
                    // Char scrambling
                    if !s.is_empty() {
                        let mut chars: Vec<char> = s.chars().collect();
                        let idx = rng.gen_range(0..chars.len());
                        chars[idx] = (rng.gen_range(33..126) as u8) as char;
                        vm.grid[y][x] = Value::Str(chars.into_iter().collect());
                        vm.output.push(format!("HAVOC: Corrupted string at {},{}", x, y));
                    }
                }
            },
            2 => {
                // Stack Fault
                if !vm.stack.is_empty() {
                    if rng.gen_bool(0.5) {
                        vm.stack.pop();
                        vm.output.push("HAVOC: Stack Drop".to_string());
                    } else {
                        // Swap
                        let len = vm.stack.len();
                        if len >= 2 {
                            let idx1 = rng.gen_range(0..len);
                            let idx2 = rng.gen_range(0..len);
                            vm.stack.swap(idx1, idx2);
                            vm.output.push("HAVOC: Stack Scramble".to_string());
                        }
                    }
                }
            },
            4 => {
                // Execution Fault
                if !vm.dna.helix.strands.is_empty() {
                    let event = rng.gen_range(0..3);
                    match event {
                        0 => {
                            // Jump
                            let s_idx = rng.gen_range(0..vm.dna.helix.strands.len());
                            vm.ip = (s_idx, 0);
                            vm.output.push(format!("HAVOC: Jump to Strand {}", s_idx));
                        },
                        1 => {
                            // Skip Gene
                            vm.ip.1 += 1;
                            vm.output.push("HAVOC: Skipped Gene".to_string());
                        },
                        2 => {
                            // Reset Energy
                            vm.energy /= 2;
                            vm.output.push("HAVOC: Energy Drain".to_string());
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }
}
