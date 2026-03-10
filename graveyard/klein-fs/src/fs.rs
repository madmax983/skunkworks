use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
    pub name: String,
}

pub fn scan_directory(path: &Path) -> Vec<FileNode> {
    let mut nodes = Vec::new();

    // Sort directories to keep structure coherent
    let walker = WalkDir::new(path).sort_by_file_name();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();
        let is_dir = entry.file_type().is_dir();
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy().to_string();

        nodes.push(FileNode {
            path,
            is_dir,
            depth,
            name,
        });
    }

    nodes
}
