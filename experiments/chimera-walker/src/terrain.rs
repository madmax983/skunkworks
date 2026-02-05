use bevy::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub const NODE_WIDTH: f32 = 40.0;
pub const NODE_GAP: f32 = 5.0;
pub const START_X: f32 = -400.0;

#[derive(Clone, Debug)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct FileNode {
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
}

#[derive(Resource, Default)]
pub struct Terrain {
    pub nodes: Vec<FileNode>,
    pub current_index: usize,
    pub progress: f32, // 0.0 to 1.0 along current node
}

impl Terrain {
    pub fn new(root: &str) -> Self {
        let mut nodes = Vec::new();
        // Limit depth to avoid infinite loops or massive scans for this demo
        for entry in WalkDir::new(root).max_depth(4).sort_by_file_name() {
            if let Ok(entry) = entry {
                // Skip hidden files/dirs to keep it clean
                if entry.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }

                let file_type = if entry.file_type().is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                };

                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                nodes.push(FileNode {
                    path: entry.path().to_path_buf(),
                    file_type,
                    size,
                });
            }
        }

        // If empty, add dummy
        if nodes.is_empty() {
            nodes.push(FileNode {
                path: PathBuf::from("Empty"),
                file_type: FileType::Directory,
                size: 0,
            });
        }

        Self {
            nodes,
            current_index: 0,
            progress: 0.0,
        }
    }
}
