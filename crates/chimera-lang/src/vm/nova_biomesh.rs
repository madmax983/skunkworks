#![cfg(feature = "nova")]

use crate::ast::Nucleotide;
use crate::vm::{ChimeraVM, Value};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct BioMeshNode {
    pub id: u64,
    pub buffer: VecDeque<Value>,
    pub connections: Vec<(usize, usize)>,
}

impl BioMeshNode {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            buffer: VecDeque::new(),
            connections: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BioMeshState {
    pub nodes: HashMap<(usize, usize), BioMeshNode>,
}

impl BioMeshState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

pub fn exec_mesh_net(vm: &mut ChimeraVM, args: &[Nucleotide]) -> Option<(usize, usize)> {
    // MeshNet(id)
    // Turns current cell into a node.
    let id = if let Some(Nucleotide::Number(n)) = args.first() {
        *n as u64
    } else if let Some(val) = vm.stack.pop() {
        if let Value::Int(n) = val {
            n as u64
        } else {
            0
        }
    } else {
        0
    };

    let (cy, cx) = vm.context_loc;
    vm.biomesh.nodes.insert((cy, cx), BioMeshNode::new(id));
    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("MESH_NET: Created Node {} at {},{}", id, cx, cy));
    None
}

pub fn exec_mesh_grow(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // MeshGrow()
    // Connects to all adjacent nodes.
    let (cy, cx) = vm.context_loc;

    // Check if we are a node
    if vm.biomesh.nodes.contains_key(&(cy, cx)) {
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut count = 0;

        // We need to mutate self, so we can't iterate immutably easily while mutating.
        // Collect valid neighbors first.
        let mut valid_neighbors = Vec::new();

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                if vm.biomesh.nodes.contains_key(&(ny, nx)) {
                    valid_neighbors.push((ny, nx));
                }
            }
        }

        // Add connections bidirectionally
        for (ny, nx) in valid_neighbors {
            // Add to current
            if let Some(node) = vm.biomesh.nodes.get_mut(&(cy, cx)) {
                if !node.connections.contains(&(ny, nx)) {
                    node.connections.push((ny, nx));
                    count += 1;
                }
            }
            // Add to neighbor
            if let Some(node) = vm.biomesh.nodes.get_mut(&(ny, nx)) {
                if !node.connections.contains(&(cy, cx)) {
                    node.connections.push((cy, cx));
                }
            }
        }

        vm.energy = vm.energy.saturating_sub(5 * count);
        vm.output
            .push(format!("MESH_GROW: Connected to {} neighbors", count));
    } else {
        vm.output.push("MESH_GROW: Not a mesh node".to_string());
    }
    None
}

pub fn exec_mesh_prune(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // MeshPrune()
    // Removes all connections from current node.
    let (cy, cx) = vm.context_loc;

    if let Some(node) = vm.biomesh.nodes.get_mut(&(cy, cx)) {
        let connections = std::mem::take(&mut node.connections);

        // Remove back-links
        for (ny, nx) in connections {
            if let Some(neighbor) = vm.biomesh.nodes.get_mut(&(ny, nx)) {
                neighbor.connections.retain(|&c| c != (cy, cx));
            }
        }

        vm.energy = vm.energy.saturating_sub(5);
        vm.output.push("MESH_PRUNE: Disconnected node".to_string());
    } else {
        vm.output.push("MESH_PRUNE: Not a mesh node".to_string());
    }
    None
}

pub fn exec_mesh_send(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // MeshSend(target_id, value)
    // Uses BFS to find path to target_id.

    if vm.stack.len() >= 2 {
        let val = vm.stack.pop().unwrap();
        let id_val = vm.stack.pop().unwrap();

        if let Value::Int(target_id) = id_val {
            let target_u64 = target_id as u64;
            let (cy, cx) = vm.context_loc;

            if !vm.biomesh.nodes.contains_key(&(cy, cx)) {
                vm.output
                    .push("MESH_SEND: Origin is not a node".to_string());
                return None;
            }

            // BFS
            let mut queue = VecDeque::new();
            let mut visited = HashMap::new(); // coord -> parent

            queue.push_back((cy, cx));
            visited.insert((cy, cx), (cy, cx)); // Root parent is self

            let mut found_target_coord = None;

            while let Some(curr) = queue.pop_front() {
                if let Some(node) = vm.biomesh.nodes.get(&curr) {
                    if node.id == target_u64 && curr != (cy, cx) {
                        found_target_coord = Some(curr);
                        break;
                    }

                    for &next in &node.connections {
                        if !visited.contains_key(&next) {
                            visited.insert(next, curr);
                            queue.push_back(next);
                        }
                    }
                }
            }

            if let Some(target_coord) = found_target_coord {
                // Deliver packet instantly (for now)
                if let Some(node) = vm.biomesh.nodes.get_mut(&target_coord) {
                    node.buffer.push_back(val);
                    vm.output.push(format!(
                        "MESH_SEND: Packet delivered to Node {}",
                        target_u64
                    ));
                }
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push(format!("MESH_SEND: Target Node {} unreachable", target_u64));
            }
        } else {
            vm.output.push("Error: Target ID must be Int".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for MeshSend".to_string());
    }
    None
}

pub fn exec_mesh_recv(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // MeshRecv()
    // Pops from local buffer.
    let (cy, cx) = vm.context_loc;

    if let Some(node) = vm.biomesh.nodes.get_mut(&(cy, cx)) {
        if let Some(val) = node.buffer.pop_front() {
            vm.stack.push(val);
            vm.output.push("MESH_RECV: Packet received".to_string());
        } else {
            vm.stack.push(Value::Int(0)); // Empty
        }
    } else {
        vm.output.push("MESH_RECV: Not a mesh node".to_string());
    }
    None
}
