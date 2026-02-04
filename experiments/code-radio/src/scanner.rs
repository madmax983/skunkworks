use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Station {
    pub path: PathBuf,
    pub freq: f64,
    pub content: String,
    pub size: usize,
}

pub fn scan_workspace(root: &Path) -> Result<Vec<Station>> {
    let mut stations = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip hidden files/dirs and the target directory
        if path.to_string_lossy().contains("/.") || path.to_string_lossy().contains("/target/") {
            continue;
        }

        if path.is_file() {
            // Calculate Frequency from path
            let freq = path_to_freq(path);

            // Read file (limit size)
            if let Ok(metadata) = entry.metadata() {
                if metadata.len() > 100_000 {
                    // Skip large files
                    continue;
                }
            }

            // Try to read as string
            if let Ok(content) = fs::read_to_string(path) {
                if !content.is_empty() {
                    stations.push(Station {
                        path: path.to_path_buf(),
                        freq,
                        size: content.len(),
                        content,
                    });
                }
            }
        }
    }

    // Sort by frequency for easier finding
    stations.sort_by(|a, b| a.freq.partial_cmp(&b.freq).unwrap());

    Ok(stations)
}

fn path_to_freq(path: &Path) -> f64 {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    let hash = hasher.finish();

    // Map to 88.0 - 108.0 MHz
    // 20 MHz bandwidth
    let normalized = (hash % 2000) as f64 / 100.0;
    88.0 + normalized
}
