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
            size: 15.0,
            age: 0.0,
            color: if is_root { GREEN } else { WHITE },
            children: Vec::new(),
            parents: Vec::new(),
            marked: false,
            alive: true,
            ref_count: 0,
        };

        self.nodes.insert(id, node);
        id
    }

    pub fn allocate(&mut self, source_pos: Vec2) -> usize {
        // Spawn slightly above source
        let pos = source_pos + vec2(macroquad::rand::gen_range(-20.0, 20.0), -40.0);
        self.create_node(pos, false)
    }

    pub fn add_reference(&mut self, from: usize, to: usize) {
        if let Some(parent) = self.nodes.get_mut(&from) {
            if !parent.children.contains(&to) {
                parent.children.push(to);
            }
        }
        if let Some(child) = self.nodes.get_mut(&to) {
            if !child.parents.contains(&from) {
                child.parents.push(from);
                child.ref_count += 1;
            }
        }
    }

    pub fn remove_reference(&mut self, from: usize, to: usize) {
        if let Some(parent) = self.nodes.get_mut(&from) {
            parent.children.retain(|&x| x != to);
        }
        if let Some(child) = self.nodes.get_mut(&to) {
            child.parents.retain(|&x| x != from);
            if child.ref_count > 0 {
                child.ref_count -= 1;
            }
        }
    }

    pub fn update(&mut self) {
        // Simple spring layout
        let k = 0.05; // spring constant
        let repulsion = 200.0;
        let damping = 0.8;
        let target_dist = 60.0;

        let ids: Vec<usize> = self.nodes.keys().copied().collect();
        let mut forces: HashMap<usize, Vec2> = HashMap::new();

        for &id in &ids {
            forces.insert(id, vec2(0.0, 0.0));
        }

        // Repulsion
        for i in 0..ids.len() {
            for j in i + 1..ids.len() {
                let id_a = ids[i];
                let id_b = ids[j];
                let node_a = &self.nodes[&id_a];
                let node_b = &self.nodes[&id_b];

                let dir = node_a.pos - node_b.pos;
                let dist = dir.length();
                if dist > 0.1 && dist < 150.0 {
                    let force = dir.normalize() * (repulsion / dist);
                    *forces.get_mut(&id_a).unwrap() += force;
                    *forces.get_mut(&id_b).unwrap() -= force;
                }
            }
        }

        // Attraction (Springs)
        for &id in &ids {
            let node = &self.nodes[&id];
            let pos = node.pos;
            let children = node.children.clone();

            for child_id in children {
                if let Some(child) = self.nodes.get(&child_id) {
                    let dir = child.pos - pos;
                    let dist = dir.length();
                    if dist > target_dist {
                        let force = dir.normalize() * (dist - target_dist) * k;
                        *forces.get_mut(&id).unwrap() += force;
                        *forces.get_mut(&child_id).unwrap() -= force;
                    }
                }
            }
        }

        // Apply forces
        for &id in &ids {
            let is_root = self.roots.contains(&id);
            if let Some(node) = self.nodes.get_mut(&id) {
                if !is_root {
                    let f = forces[&id];
                    node.pos += f * damping;

                    // Gravity
                    node.pos.y += 0.5;

                    // Bound
                    node.pos.x = node.pos.x.clamp(50.0, 750.0);
                    node.pos.y = node.pos.y.clamp(50.0, 550.0);
                }

                if node.alive {
                    node.age += 1.0;
                } else {
                    node.size *= 0.95; // Shrink before dying
                }
            }
        }

        // Cleanup fully dead nodes
        let dead_ids: Vec<usize> = self
            .nodes
            .iter()
            .filter(|(_, n)| !n.alive && n.size < 1.0)
            .map(|(k, _)| *k)
            .collect();

        for id in dead_ids {
            self.nodes.remove(&id);
            for node in self.nodes.values_mut() {
                node.children.retain(|&x| x != id);
                node.parents.retain(|&x| x != id);
            }
        }
    }

    pub fn determine_reachability(&mut self) -> Vec<usize> {
        let mut reachable = Vec::new();
        let mut queue = VecDeque::new();
        for &root in &self.roots {
            queue.push_back(root);
            reachable.push(root);
        }

        while let Some(current) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&current) {
                for &child in &node.children {
                    if !reachable.contains(&child) {
                        reachable.push(child);
                        queue.push_back(child);
                    }
                }
            }
        }
        reachable
    }
}
