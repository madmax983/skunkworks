use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub path: PathBuf,
    pub name: String,
    pub mass: f32, // Lines of code
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub fn scan_dependencies(root: &Path) -> Result<Graph> {
    let mut nodes = Vec::new();
    let mut path_to_id = HashMap::new();
    let mut name_to_ids: HashMap<String, Vec<usize>> = HashMap::new();
    let mut edges = Vec::new();

    println!("Scanning {}...", root.display());

    // 1. Scan files
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let line_count = content.lines().count() as f32;
            let name = path.file_stem().unwrap().to_string_lossy().to_string();

            let id = nodes.len();
            nodes.push(Node {
                id,
                path: path.to_path_buf(),
                name: name.clone(),
                mass: line_count.max(10.0), // Min mass
            });
            path_to_id.insert(path.to_path_buf(), id);
            name_to_ids.entry(name).or_default().push(id);
        }
    }

    println!("Found {} nodes.", nodes.len());

    // 2. Scan imports
    let mod_regex = Regex::new(r"mod\s+([a-zA-Z0-9_]+);").unwrap();
    // Simple use regex: `use crate::foo` or `use foo::bar`
    // We capture the first segment
    let use_regex = Regex::new(r"use\s+(?:crate::|super::)?([a-zA-Z0-9_]+)").unwrap();

    for node in &nodes {
        let content = match fs::read_to_string(&node.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let parent_dir = node.path.parent().unwrap();

        // Check for child modules `mod foo;`
        for cap in mod_regex.captures_iter(&content) {
            let mod_name = &cap[1];
            let potential_paths = vec![
                parent_dir.join(format!("{}.rs", mod_name)),
                parent_dir.join(mod_name).join("mod.rs"),
            ];

            for p in potential_paths {
                if let Some(&target_id) = path_to_id.get(&p) {
                    edges.push(Edge {
                        source: node.id,
                        target: target_id,
                    });
                }
            }
        }

        // Fuzzy link for `use` statements
        for cap in use_regex.captures_iter(&content) {
            let use_name = &cap[1];
            if let Some(target_ids) = name_to_ids.get(use_name) {
                for &target_id in target_ids {
                    if target_id != node.id {
                        edges.push(Edge {
                            source: node.id,
                            target: target_id,
                        });
                    }
                }
            }
        }
    }

    // Dedup edges
    edges.sort_by(|a, b| a.source.cmp(&b.source).then(a.target.cmp(&b.target)));
    edges.dedup_by(|a, b| a.source == b.source && a.target == b.target);

    println!("Found {} edges.", edges.len());

    Ok(Graph { nodes, edges })
}
