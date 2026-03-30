use locus::Vec2;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub content: String,
    pub pos: Vec2,
    pub vel: Vec2,
    pub entropy: f64, // Rot/entropy level
}

#[derive(Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String, content: String) -> usize {
        let id = self.nodes.len();
        let entropy = if content.len() > 10000 {
            0.9
        } else if content.len() > 5000 {
            0.7
        } else if content.len() > 1000 {
            0.4
        } else {
            0.1
        };

        // Safe Assumption: Use `rand::random::<f64>()` because locus::Vec2
        // uses f64 specifically, to avoid mismatched types.
        self.nodes.push(Node {
            id,
            name,
            content,
            pos: Vec2::new(rand::random::<f64>() * 800.0 - 400.0, rand::random::<f64>() * 600.0 - 300.0),
            vel: Vec2::new(0.0, 0.0),
            entropy,
        });
        id
    }

    pub fn scan_directory(&mut self, root: &str) {
        let mut name_to_id = HashMap::new();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "rs" || ext == "md" || ext == "toml" {
                        if let Ok(content) = fs::read_to_string(path) {
                            let name = path.file_name().unwrap().to_string_lossy().to_string();
                            let id = self.add_node(name.clone(), content);
                            name_to_id.insert(name, id);
                        }
                    }
                }
            }
        }

        let node_count = self.nodes.len();
        if node_count > 1 {
            for i in 0..node_count {
                let num_edges = (rand::random::<usize>() % 3) + 1;
                for _ in 0..num_edges {
                    let target = rand::random::<usize>() % node_count;
                    if target != i {
                        self.edges.push(Edge { from: i, to: target });
                        self.adjacency.entry(i).or_insert_with(Vec::new).push(target);
                        self.adjacency.entry(target).or_insert_with(Vec::new).push(i);
                    }
                }
            }
        }
    }
}
