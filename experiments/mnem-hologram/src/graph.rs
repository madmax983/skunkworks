use locus::Vec2;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub content: String,
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: f32, // 0.0 to 1.0
}

#[derive(Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, content: String) -> usize {
        let mut rng = rand::thread_rng();
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            content,
            pos: Vec2::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)),
            vel: Vec2::new(0.0, 0.0),
            health: 1.0,
        });
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }

    pub fn scan_directory(&mut self, root: &str) {
        let walker = WalkDir::new(root).into_iter();
        let mut file_map = HashMap::new();

        // Pass 1: Create nodes
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.path().extension().is_some_and(|ext| ext == "rs") {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    let id = self.add_node(content);
                    file_map.insert(name, id);
                }
            }
        }

        // Pass 2: Create edges (very naive)
        let re = Regex::new(r"(?:mod|use|crate)\s+(?:::\s*)?([a-zA-Z0-9_]+)").unwrap();
        let mut new_edges = Vec::new();

        for node in &self.nodes {
            for cap in re.captures_iter(&node.content) {
                if let Some(target_match) = cap.get(1) {
                    let target_name = target_match.as_str();
                    let target_rs = format!("{}.rs", target_name);

                    if let Some(&target_id) = file_map.get(&target_rs) {
                        if target_id != node.id {
                            new_edges.push((node.id, target_id));
                        }
                    }
                }
            }
        }

        for (from, to) in new_edges {
            self.add_edge(from, to);
        }
    }
}
