use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct DependencyGraph {
    pub graph: DiGraph<String, ()>,
    pub node_map: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn build(root: &Path) -> Self {
        let mut dep_graph = Self::new();
        let mut files = HashMap::new();

        // 1. Collect all .rs files
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                let path_str = path.to_string_lossy().to_string();
                let stem = path.file_stem().unwrap().to_string_lossy().to_string();

                // Add node
                let idx = dep_graph.graph.add_node(path_str.clone());
                dep_graph.node_map.insert(path_str.clone(), idx);

                // Store for fuzzy matching (stem -> list of full paths)
                files.entry(stem).or_insert_with(Vec::new).push(path_str);
            }
        }

        // 2. Scan content for edges
        let use_regex = Regex::new(r"use\s+([\w:]+);").unwrap();
        let mod_regex = Regex::new(r"mod\s+(\w+);").unwrap();

        let nodes: Vec<String> = dep_graph.node_map.keys().cloned().collect();

        for file_path in nodes {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let source_idx = *dep_graph.node_map.get(&file_path).unwrap();

                // Check imports/mods
                for line in content.lines() {
                    let mut target_stem = None;

                    if let Some(caps) = use_regex.captures(line) {
                        let full_use = &caps[1];
                        // Get first part: use foo::bar -> foo
                        if let Some(first_part) = full_use.split("::").next() {
                            if first_part != "crate" && first_part != "std" && first_part != "super" && first_part != "self" {
                                target_stem = Some(first_part.to_string());
                            }
                        }
                    } else if let Some(caps) = mod_regex.captures(line) {
                         target_stem = Some(caps[1].to_string());
                    }

                    if let Some(stem) = target_stem {
                        if let Some(candidates) = files.get(&stem) {
                            for target_path in candidates {
                                // Simple heuristic: if multiple candidates, link to all of them?
                                // Or try to pick the closest one?
                                // For Geology, "All" creates more interesting caves (connectivity).
                                // But avoid self-loops
                                if target_path != &file_path {
                                    if let Some(target_idx) = dep_graph.node_map.get(target_path) {
                                        dep_graph.graph.add_edge(source_idx, *target_idx, ());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        dep_graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_scan() {
        let root = Path::new(".");
        let graph = DependencyGraph::build(root);
        assert!(graph.graph.node_count() > 0);
        // We should find "graph.rs"
        let found = graph.node_map.keys().any(|k| k.contains("graph.rs"));
        assert!(found);
    }
}
