#![cfg(feature = "nova")]

use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type NodeId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructNode {
    pub id: NodeId,
    pub op: OpCode,
    pub value: Option<Value>, // For Push/Literal nodes
    pub x: f64,
    pub y: f64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructLink {
    pub from: NodeId,
    pub to: NodeId,
    pub input_idx: usize, // Which argument index this feeds into (0=Top, 1=Second...)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructState {
    pub nodes: HashMap<NodeId, ConstructNode>,
    pub links: Vec<ConstructLink>,
    pub next_id: NodeId,
    pub cursor: (f64, f64),
    pub selected_node: Option<NodeId>,
    pub linking_from: Option<NodeId>,
}

impl ConstructState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
            next_id: 0,
            cursor: (0.0, 0.0),
            selected_node: None,
            linking_from: None,
        }
    }

    pub fn add_node(&mut self, op: OpCode, val: Option<Value>) {
        let node = ConstructNode {
            id: self.next_id,
            op: op.clone(),
            value: val,
            x: self.cursor.0,
            y: self.cursor.1,
            label: format!("{:?}", op),
        };
        self.nodes.insert(self.next_id, node);
        self.selected_node = Some(self.next_id);
        self.next_id += 1;
    }

    pub fn delete_selection(&mut self) {
        if let Some(id) = self.selected_node {
            self.nodes.remove(&id);
            self.links.retain(|l| l.from != id && l.to != id);
            self.selected_node = None;
        }
    }

    pub fn start_link(&mut self) {
        if let Some(id) = self.selected_node {
            self.linking_from = Some(id);
        }
    }

    pub fn complete_link(&mut self) {
        if let (Some(from), Some(to)) = (self.linking_from, self.selected_node) {
            if from != to {
                // Determine input index based on existing links to 'to'
                let count = self.links.iter().filter(|l| l.to == to).count();
                self.links.push(ConstructLink {
                    from,
                    to,
                    input_idx: count,
                });
            }
        }
        self.linking_from = None;
    }

    pub fn compile(&self) -> Result<Strand, String> {
        // Find roots (nodes that are not inputs to any other node, or explicitly marked?)
        // In a pure expression tree, the root is the final result.
        // But we might have multiple roots (sequence of instructions).
        // Let's look for nodes that are NOT `from` in any link.

        let mut child_ids = HashSet::new();
        for link in &self.links {
            child_ids.insert(link.from);
        }

        let mut roots: Vec<NodeId> = self.nodes.keys()
            .filter(|k| !child_ids.contains(k))
            .cloned()
            .collect();

        // Sort roots by Y/X position to determine execution order?
        // Or specific "Start" node?
        // Let's sort by X coordinate (Left to Right execution flow for multiple roots)
        roots.sort_by(|a, b| {
            let na = &self.nodes[a];
            let nb = &self.nodes[b];
            na.x.partial_cmp(&nb.x).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut genes = Vec::new();

        for root in roots {
            self.compile_node(root, &mut genes)?;
        }

        Ok(Strand { genes })
    }

    fn compile_node(&self, id: NodeId, genes: &mut Vec<Gene>) -> Result<(), String> {
        // Tree expansion: Visit inputs in order (Reverse order for stack?)
        // If Op is Add. Stack should be [A, B]. Add -> A+B.
        // So we need to push A, then push B.
        // Input 0 = A. Input 1 = B.
        // So compile Input 0, then Input 1.

        // Find inputs
        let mut inputs: Vec<&ConstructLink> = self.links.iter().filter(|l| l.to == id).collect();
        inputs.sort_by_key(|l| l.input_idx);

        // Check recursion (if we were doing DAG, but we are doing tree expansion)
        // If we encounter a cycle, we will stack overflow or infinite loop.
        // Simple cycle check path?
        // For pure tree expansion, we don't mark visited globally, but along the path.
        // But here I passed `visited` which accumulates. This prevents code reuse (DAG behavior).
        // If we want DAG behavior (reuse result), we assume result is on stack.
        // But Chimera stack is consumed. So we MUST re-execute (Tree expansion) OR use `dup`.
        // Let's stick to Tree Expansion (re-execute). So `visited` should be path-based.

        // Use a local path set for cycle detection
        let mut path = HashSet::new();
        self.compile_recursive(id, genes, &mut path)
    }

    fn compile_recursive(&self, id: NodeId, genes: &mut Vec<Gene>, path: &mut HashSet<NodeId>) -> Result<(), String> {
        if path.contains(&id) {
            return Err(format!("Cycle detected at node {}", id));
        }
        path.insert(id);

        let node = self.nodes.get(&id).ok_or("Node not found")?;

        // 1. Compile Inputs
        let mut inputs: Vec<&ConstructLink> = self.links.iter().filter(|l| l.to == id).collect();
        inputs.sort_by_key(|l| l.input_idx);

        // For stack machine: Push args first.
        // Input 0 (Top/First arg)
        // Wait, `sub` computes `second - top` usually? Or `top - second`?
        // In Chimera: `a b sub` -> `a - b`.
        // `a` is pushed first (lower on stack). `b` is pushed last (top).
        // So Input 0 (A) should be compiled first?
        // Yes.
        for input in inputs {
            self.compile_recursive(input.from, genes, path)?;
        }

        // 2. Emit Op
        if node.op == OpCode::Push {
            // Push literal
            if let Some(val) = &node.value {
                match val {
                    Value::Int(n) => genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(*n)] }),
                    Value::Str(s) => genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::String(s.clone())] }),
                    _ => return Err("Unsupported value type in Construct".to_string()),
                }
            } else {
                // Just Push with no arg? Invalid in Chimera usually, but maybe Nop
            }
        } else {
            // Generic Op
            genes.push(Gene { op: node.op.clone(), args: vec![] });
        }

        path.remove(&id);
        Ok(())
    }
}
