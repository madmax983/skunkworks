use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct FSNode {
    pub name: String,
    pub path: PathBuf,
    pub node_type: NodeType,
    pub size: u64,
    pub children: Vec<FSNode>,
}

impl FSNode {
    pub fn new(path: PathBuf, node_type: NodeType, size: u64) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        Self {
            name,
            path,
            node_type,
            size,
            children: Vec::new(),
        }
    }
}

pub fn scan_fs(path: &Path, max_depth: usize) -> Option<FSNode> {
    scan_recursive(path, 0, max_depth)
}

fn scan_recursive(path: &Path, current_depth: usize, max_depth: usize) -> Option<FSNode> {
    if !path.exists() {
        return None;
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return None,
    };

    let node_type = if metadata.is_dir() {
        NodeType::Directory
    } else {
        NodeType::File
    };

    let size = metadata.len();
    let mut node = FSNode::new(path.to_path_buf(), node_type.clone(), size);

    if node_type == NodeType::Directory && current_depth < max_depth {
        if let Ok(entries) = fs::read_dir(path) {
            let mut children = Vec::new();
            for entry in entries.flatten() {
                let child_path = entry.path();
                // Avoid symlink loops or hidden files if necessary
                if let Some(child_node) = scan_recursive(&child_path, current_depth + 1, max_depth) {
                    children.push(child_node);
                }
            }
            // Sort children: Directories first, then files
            children.sort_by(|a, b| {
                match (&a.node_type, &b.node_type) {
                    (NodeType::Directory, NodeType::File) => std::cmp::Ordering::Less,
                    (NodeType::File, NodeType::Directory) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
            node.children = children;

            // Accumulate size for directory
            node.size = node.children.iter().map(|c| c.size).sum();
        }
    }

    Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_current_dir() {
        let root = Path::new(".");
        let node = scan_fs(root, 2);
        assert!(node.is_some());
        let node = node.unwrap();
        assert_eq!(node.node_type, NodeType::Directory);
        assert!(node.children.len() > 0);

        // Find Cargo.toml
        let cargo = node.children.iter().find(|c| c.name == "Cargo.toml");
        assert!(cargo.is_some());
    }
}
