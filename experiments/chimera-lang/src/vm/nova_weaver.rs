#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaverNode {
    pub id: usize,
    pub op: OpCode,
    pub args: Vec<Nucleotide>,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaverGraph {
    pub nodes: HashMap<usize, WeaverNode>,
    pub edges: Vec<(usize, usize)>, // (from, to)
    pub selected_node: Option<usize>,
    pub next_id: usize,
    pub cursor: (f64, f64),
}

impl WeaverGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            selected_node: None,
            next_id: 0,
            cursor: (0.0, 0.0),
        }
    }

    pub fn add_node(&mut self, op: OpCode, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, WeaverNode {
            id,
            op,
            args: Vec::new(),
            x,
            y,
        });
        id
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.retain(|(from, to)| *from != id && *to != id);
        if self.selected_node == Some(id) {
            self.selected_node = None;
        }
    }

    pub fn link(&mut self, from: usize, to: usize) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            // Avoid duplicates
            if !self.edges.contains(&(from, to)) {
                self.edges.push((from, to));
            }
        }
    }

    pub fn get_node_at(&self, x: f64, y: f64) -> Option<usize> {
        for node in self.nodes.values() {
            if x >= node.x && x <= node.x + 8.0 && y >= node.y && y <= node.y + 4.0 {
                return Some(node.id);
            }
        }
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaverState {
    pub graph: WeaverGraph,
}

impl WeaverState {
    pub fn new() -> Self {
        Self {
            graph: WeaverGraph::new(),
        }
    }
}

pub fn compile_graph(graph: &WeaverGraph) -> Vec<Gene> {
    // Simple linearization: Find start nodes (indegree 0) and traverse.
    // If cycles, break them?
    // For now, let's just sort by ID or position for determinism if disconnected.
    // Better: Sort by X coordinate. Left-to-Right execution.
    // This is the simplest visual programming model.
    // Edges can be "jumps".

    // Strategy 1: Spatial Sort (Reader Order)
    // Execute nodes from top-left to bottom-right.
    // Ignore edges for sequence? No, edges are crucial.

    // Strategy 2: Explicit Start Node?
    // Let's assume the node with lowest ID is start, or user marks it.
    // Let's try Spatial Sort.
    // 1. Sort nodes by Y then X.
    // 2. Iterate and generate genes.
    // 3. Edges representing Jumps?

    // Actually, "The Weaver" implies weaving threads.
    // Let's stick to Spatial Sort for now. It's robust.
    // "Reading" the loom.

    let mut nodes: Vec<&WeaverNode> = graph.nodes.values().collect();
    nodes.sort_by(|a, b| {
        a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal)
            .then(a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut genes = Vec::new();
    for node in nodes {
        genes.push(Gene {
            op: node.op.clone(),
            args: node.args.clone(),
        });
    }
    genes
}

/// Executes Weave-related OpCodes.
pub fn exec_weave_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Weave => {
            // Stack: [ ..., strand_a, strand_b, pattern ] -> [ ..., new_strand_idx ]
            if vm.stack.len() >= 3 {
                let pattern_val = vm.stack.pop().unwrap();
                let b_val = vm.stack.pop().unwrap();
                let a_val = vm.stack.pop().unwrap();

                if let (Value::Int(idx_a), Value::Int(idx_b)) = (a_val, b_val) {
                    let pattern_str = match pattern_val {
                        Value::Str(s) => s,
                        Value::Int(idx_p) => {
                            // Compile pattern from strand
                            let mut s = String::new();
                            if idx_p >= 0 && (idx_p as usize) < vm.dna.helix.strands.len() {
                                for gene in &vm.dna.helix.strands[idx_p as usize].genes {
                                    // Use first letter of OpCode as pattern char
                                    let op_str = gene.op.to_string();
                                    if let Some(c) = op_str.chars().next() {
                                        s.push(c.to_ascii_uppercase());
                                    }
                                }
                            }
                            s
                        }
                        _ => String::new(),
                    };

                    if idx_a >= 0
                        && idx_b >= 0
                        && (idx_a as usize) < vm.dna.helix.strands.len()
                        && (idx_b as usize) < vm.dna.helix.strands.len()
                    {
                        let strand_a = &vm.dna.helix.strands[idx_a as usize];
                        let strand_b = &vm.dna.helix.strands[idx_b as usize];

                        let mut new_genes = Vec::new();
                        let mut ptr_a = 0;
                        let mut ptr_b = 0;
                        let mut rng = rand::thread_rng();

                        for c in pattern_str.chars() {
                            match c {
                                'A' => {
                                    if ptr_a < strand_a.genes.len() {
                                        new_genes.push(strand_a.genes[ptr_a].clone());
                                        ptr_a += 1;
                                    }
                                }
                                'B' => {
                                    if ptr_b < strand_b.genes.len() {
                                        new_genes.push(strand_b.genes[ptr_b].clone());
                                        ptr_b += 1;
                                    }
                                }
                                'X' => {
                                    // Randomly pick A or B
                                    if rng.gen_bool(0.5) {
                                        if ptr_a < strand_a.genes.len() {
                                            new_genes.push(strand_a.genes[ptr_a].clone());
                                            ptr_a += 1;
                                        }
                                    } else {
                                        if ptr_b < strand_b.genes.len() {
                                            new_genes.push(strand_b.genes[ptr_b].clone());
                                            ptr_b += 1;
                                        }
                                    }
                                }
                                '0' => {
                                    // Skip / Gap (Insert Nop?)
                                    // Let's insert a Nop to preserve structure/timing
                                    new_genes.push(Gene {
                                        op: OpCode::Nop,
                                        args: vec![],
                                    });
                                }
                                _ => {}
                            }
                        }

                        // Append any new strand to Helix
                        let new_strand_idx = vm.dna.helix.strands.len();
                        vm.dna.helix.strands.push(Strand { genes: new_genes });
                        vm.telomeres.push(50); // Default telomere
                        #[cfg(feature = "cortex")]
                        {
                            vm.synapse_map.push(vec![]);
                            vm.activation_levels.push(0);
                        }

                        vm.stack.push(Value::Int(new_strand_idx as i64));
                        vm.output.push(format!(
                            "WEAVE: Created strand {} from {} and {} with pattern '{}'",
                            new_strand_idx, idx_a, idx_b, pattern_str
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid strand indices for Weave".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for Weave strands".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Weave".to_string());
            }
        }
        OpCode::Unravel => {
            // Stack: [ ..., strand_idx ] -> [ ... ]
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if idx >= 0 && (idx as usize) < vm.dna.helix.strands.len() {
                    let u_idx = idx as usize;
                    let genes_count = vm.dna.helix.strands[u_idx].genes.len();

                    // Clear genes
                    vm.dna.helix.strands[u_idx].genes.clear();

                    // Reclaim energy: 1 energy per gene
                    let reclaimed = genes_count as i64;
                    vm.energy = vm.energy.saturating_add(reclaimed);

                    vm.output.push(format!("UNRAVEL: Destroyed strand {}, reclaimed {} energy", idx, reclaimed));
                } else {
                    vm.output.push("Error: Invalid strand index for Unravel".to_string());
                }
            } else {
                vm.output.push("Error: Invalid arg for Unravel".to_string());
            }
        }
        _ => {}
    }
    None
}
