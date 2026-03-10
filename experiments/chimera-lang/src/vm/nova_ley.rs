use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeyNode {
    pub y: usize,
    pub x: usize,
    pub power: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeyNetwork {
    pub nodes: Vec<LeyNode>,
    pub connections: Vec<Vec<usize>>,
}

impl Default for LeyNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl LeyNetwork {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, y: usize, x: usize, power: i64) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(LeyNode { y, x, power });
        self.connections.push(Vec::new());
        idx
    }

    pub fn generate_random(&mut self) {
        let mut rng = rand::thread_rng();
        // Generate 3-5 nodes
        let count = rng.gen_range(3..=5);
        for _ in 0..count {
            self.nodes.push(LeyNode {
                y: rng.gen_range(0..GRID_SIZE),
                x: rng.gen_range(0..GRID_SIZE),
                power: rng.gen_range(50..200),
            });
            self.connections.push(Vec::new());
        }

        // Randomly connect
        if count > 1 {
            for i in 0..count {
                let target = rng.gen_range(0..count);
                if i != target {
                    self.connect(i, target);
                }
            }
            // Ensure graph connectivity (simplified: connect 0 to all others if isolated)
            // This is just a game mechanic, minimal connectivity is fine.
        }
    }

    pub fn connect(&mut self, idx1: usize, idx2: usize) {
        if idx1 < self.nodes.len()
            && idx2 < self.nodes.len()
            && idx1 != idx2
            && !self.connections[idx1].contains(&idx2)
        {
            self.connections[idx1].push(idx2);
            self.connections[idx2].push(idx1);
        }
    }

    pub fn find_nearest(&self, y: usize, x: usize) -> Option<(usize, f64)> {
        let mut min_dist = f64::MAX;
        let mut nearest_idx = None;

        for (i, node) in self.nodes.iter().enumerate() {
            let dy = node.y as f64 - y as f64;
            let dx = node.x as f64 - x as f64;
            let dist = (dy * dy + dx * dx).sqrt();
            if dist < min_dist {
                min_dist = dist;
                nearest_idx = Some(i);
            }
        }

        nearest_idx.map(|idx| (idx, min_dist))
    }
}

pub fn exec_ley_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::LeySense => exec_ley_sense(vm),
        OpCode::LeyTap => exec_ley_tap(vm),
        OpCode::LeyWarp => exec_ley_warp(vm),
        OpCode::LeyShift => exec_ley_shift(vm),
        _ => None,
    }
}

/// **OpCode:** `LeySense`
/// **Stack:** `[ ... ] -> [ ..., dy, dx, distance, power ]`
fn exec_ley_sense(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if let Some((idx, dist)) = vm.ley_network.find_nearest(cy, cx) {
        let node = &vm.ley_network.nodes[idx];
        let dy = node.y as i64 - cy as i64;
        let dx = node.x as i64 - cx as i64;

        vm.stack.push(Value::Int(dy));
        vm.stack.push(Value::Int(dx));
        let dist_int: i64 = dist.round() as i64;
        vm.stack.push(Value::Int(dist_int));
        vm.stack.push(Value::Int(node.power));
    } else {
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(-1));
        vm.stack.push(Value::Int(0));
    }
    None
}

/// **OpCode:** `LeyTap`
/// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
fn exec_ley_tap(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if let Some((idx, dist)) = vm.ley_network.find_nearest(cy, cx) {
        if dist < 1.5 {
            // Within 1 cell (diagonal is 1.414)
            let node = &mut vm.ley_network.nodes[idx];
            let gain = node.power;

            // Risk check: If power is high, chance of damage
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..200) < gain {
                // Overload!
                let damage = gain / 2;
                vm.energy = vm.energy.saturating_sub(damage);
                vm.output
                    .push(format!("LEY_TAP: Overload! Lost {} energy", damage));
                vm.stack.push(Value::Int(-damage));

                // Explode grid
                vm.grid[cy][cx] = Value::Int(0);

                // Reduce node power
                node.power /= 2;
            } else {
                vm.energy = vm.energy.saturating_add(gain);
                vm.output.push(format!("LEY_TAP: Absorbed {} energy", gain));
                vm.stack.push(Value::Int(gain));

                // Drain node
                node.power = (node.power * 9) / 10;
            }
        } else {
            vm.output.push("LEY_TAP: Too far from node".to_string());
            vm.stack.push(Value::Int(0));
        }
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

/// **OpCode:** `LeyWarp`
/// **Stack:** `[ ..., target_node_idx ] -> [ ... ]`
fn exec_ley_warp(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Determine current node
    let (cy, cx) = vm.context_loc;
    if let Some((current_idx, dist)) = vm.ley_network.find_nearest(cy, cx) {
        if dist < 1.5 {
            // Check connections
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(target_idx) = val {
                    let tid = target_idx as usize;
                    if vm.ley_network.connections[current_idx].contains(&tid) {
                        let target_node = &vm.ley_network.nodes[tid];
                        vm.context_loc = (target_node.y, target_node.x);
                        vm.energy = vm.energy.saturating_sub(20);
                        vm.output
                            .push(format!("LEY_WARP: Teleported to Node {}", tid));
                    } else {
                        vm.output
                            .push(format!("LEY_WARP: No connection to Node {}", tid));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for ley_warp".to_string());
                }
            } else {
                // Auto-warp to first connection
                if let Some(&first) = vm.ley_network.connections[current_idx].first() {
                    let target_node = &vm.ley_network.nodes[first];
                    vm.context_loc = (target_node.y, target_node.x);
                    vm.energy = vm.energy.saturating_sub(20);
                    vm.output
                        .push(format!("LEY_WARP: Teleported to Node {}", first));
                } else {
                    vm.output.push("LEY_WARP: Dead end".to_string());
                }
            }
        } else {
            vm.output.push("LEY_WARP: Not on a Ley Node".to_string());
        }
    }
    None
}

/// **OpCode:** `LeyShift`
/// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
fn exec_ley_shift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if let Some((idx, dist)) = vm.ley_network.find_nearest(cy, cx) {
        if dist < 1.5 {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (y_val, x_val) {
                    let old_y = vm.ley_network.nodes[idx].y;
                    let old_x = vm.ley_network.nodes[idx].x;

                    if let Some((ny, nx)) =
                        vm.normalize_coords(old_y as i64 + dy, old_x as i64 + dx)
                    {
                        vm.ley_network.nodes[idx].y = ny;
                        vm.ley_network.nodes[idx].x = nx;
                        vm.energy = vm.energy.saturating_sub(50);
                        vm.output
                            .push(format!("LEY_SHIFT: Moved Node {} to {},{}", idx, nx, ny));
                    } else {
                        vm.output.push("LEY_SHIFT: Out of bounds".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for ley_shift".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ley_shift".to_string());
            }
        } else {
            vm.output.push("LEY_SHIFT: Not on a Ley Node".to_string());
        }
    }
    None
}
