use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn scan<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        if path.is_file() {
            let metadata = fs::metadata(path).ok();
            let size = metadata.map(|m| m.len()).unwrap_or(0);
            return FileNode {
                path: path.to_path_buf(),
                is_dir: false,
                size,
                children: Vec::new(),
            };
        }

        let mut children = Vec::new();
        let mut total_size = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                // Skip hidden files/dirs and target
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "target" || name == "node_modules" {
                        continue;
                    }
                }

                let child = FileNode::scan(&p);
                total_size += child.size;
                children.push(child);
            }
        }

        // Sort children by size descending
        children.sort_by(|a, b| b.size.cmp(&a.size));

        FileNode {
            path: path.to_path_buf(),
            is_dir: true,
            size: total_size,
            children,
        }
    }
}
