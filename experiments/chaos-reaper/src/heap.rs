use macroquad::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub pos: Vec2,
    pub size: f32,
    pub age: f32,
    pub color: Color,
    pub children: Vec<usize>,
    pub parents: Vec<usize>,
    pub marked: bool,
    pub alive: bool,
    pub ref_count: usize,
}

pub struct Heap {
    pub nodes: HashMap<usize, Node>,
    pub roots: Vec<usize>,
    pub next_id: usize,
}

impl Heap {
    pub fn new() -> Self {
        let mut heap = Self {
            nodes: HashMap::new(),
            roots: Vec::new(),
            next_id: 0,
        };

        for i in 0..3 {
            let safe_pos = vec2(300.0 + (i as f32 - 1.0) * 100.0, 500.0);
            let id = heap.create_node(safe_pos, true);
            heap.roots.push(id);
        }
        heap
    }

    fn create_node(&mut self, pos: Vec2, is_root: bool) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let node = Node {
            id,
            pos,
            size: if is_root { 20.0 } else { 10.0 },
            age: 0.0,
            color: if is_root { GREEN } else { LIME },
            children: Vec::new(),
            parents: Vec::new(),
            marked: false,
            alive: true,
            ref_count: if is_root { 1 } else { 0 },
        };

        self.nodes.insert(id, node);
        id
    }

    pub fn allocate(&mut self, pos: Vec2) -> usize {
        self.create_node(pos, false)
    }

    pub fn link(&mut self, from: usize, to: usize) {
        if let Some(parent) = self.nodes.get_mut(&from) {
            parent.children.push(to);
        }
        if let Some(child) = self.nodes.get_mut(&to) {
            child.parents.push(from);
            child.ref_count += 1;
        }
    }

    pub fn mark_from_roots(&mut self) {
        for node in self.nodes.values_mut() {
            node.marked = false;
        }

        let mut queue = VecDeque::new();
        for &root in &self.roots {
            queue.push_back(root);
            if let Some(node) = self.nodes.get_mut(&root) {
                node.marked = true;
            }
        }

        while let Some(id) = queue.pop_front() {
            let children = if let Some(node) = self.nodes.get(&id) {
                node.children.clone()
            } else {
                Vec::new()
            };

            for child_id in children {
                if let Some(child) = self.nodes.get_mut(&child_id) {
                    if !child.marked && child.alive {
                        child.marked = true;
                        queue.push_back(child_id);
                    }
                }
            }
        }
    }

    pub fn sweep(&mut self) -> usize {
        let mut collected = 0;
        let mut to_kill = Vec::new();
        for node in self.nodes.values() {
            if !node.marked && node.alive {
                to_kill.push(node.id);
            }
        }

        for id in to_kill {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.alive = false;
                node.color = BROWN;
                collected += 1;
                // Decrement ref counts of children
                let children = node.children.clone();
                for child_id in children {
                    if let Some(child) = self.nodes.get_mut(&child_id) {
                        if child.ref_count > 0 {
                            child.ref_count -= 1;
                        }
                    }
                }
            }
        }
        collected
    }

    pub fn reference_counting(&mut self) -> usize {
        let mut collected = 0;

        let to_kill: Vec<usize> = self
            .nodes
            .values()
            .filter(|n| n.ref_count == 0 && !self.roots.contains(&n.id) && n.alive)
            .map(|n| n.id)
            .collect();

        for id in to_kill {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.alive = false;
                node.color = BROWN;
                collected += 1;
                // Decrement children ref counts to handle cascading GC
                let children = node.children.clone();
                for child_id in children {
                    if let Some(child) = self.nodes.get_mut(&child_id) {
                        if child.ref_count > 0 {
                            child.ref_count -= 1;
                        }
                    }
                }
            }
        }

        collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut heap = Heap::new();
        let id = heap.allocate(vec2(0.0, 0.0));
        assert!(heap.nodes.contains_key(&id));
        assert!(heap.nodes[&id].alive);
    }

    #[test]
    fn test_mark_and_sweep_cycle() {
        let mut heap = Heap::new();

        let a = heap.allocate(vec2(10.0, 10.0));
        let b = heap.allocate(vec2(20.0, 20.0));

        heap.link(a, b);
        heap.link(b, a);

        heap.mark_from_roots();
        let collected = heap.sweep();

        assert_eq!(collected, 2);
        assert!(!heap.nodes[&a].alive);
        assert!(!heap.nodes[&b].alive);
    }

    #[test]
    fn test_ref_counting_basic() {
        let mut heap = Heap::new();

        let root = heap.roots[0];
        let a = heap.allocate(vec2(10.0, 10.0));
        heap.link(root, a);

        assert_eq!(heap.nodes[&a].ref_count, 1);

        let b = heap.allocate(vec2(50.0, 50.0));

        let collected = heap.reference_counting();
        assert_eq!(collected, 1);
        assert!(!heap.nodes[&b].alive);
        assert!(heap.nodes[&a].alive);
    }
}
