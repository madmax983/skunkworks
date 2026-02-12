use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Node {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
}

pub fn scan_directory(root: &str, max_depth: usize) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut path_to_index: HashMap<String, usize> = HashMap::new();

    let root_path = Path::new(root);
    // Normalize root path string for consistent lookups?
    // Actually, WalkDir returns paths relative to root if root is relative.
    let root_str = root_path.to_string_lossy().to_string();

    nodes.push(Node {
        path: root_str.clone(),
        name: root_path.file_name().unwrap_or(root_path.as_os_str()).to_string_lossy().to_string(),
        is_dir: true,
        depth: 0,
        children: Vec::new(),
        parent: None,
    });
    path_to_index.insert(root_str.clone(), 0);

    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(max_depth)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        let depth = entry.depth();
        let is_dir = entry.file_type().is_dir();
        let name = entry.file_name().to_string_lossy().to_string();

        let parent_path = path.parent().map(|p| p.to_string_lossy().to_string());
        let parent_idx = parent_path.and_then(|p| path_to_index.get(&p).cloned());

        if let Some(pidx) = parent_idx {
            let node_idx = nodes.len();
            nodes[pidx].children.push(node_idx);

            nodes.push(Node {
                path: path_str.clone(),
                name,
                is_dir,
                depth,
                children: Vec::new(),
                parent: Some(pidx),
            });
            path_to_index.insert(path_str, node_idx);
        } else {
             // If parent is not found, we skip adding this node to avoid orphans.
             // This might happen if permission error prevented scanning parent, etc.
        }
    }

    nodes
}
