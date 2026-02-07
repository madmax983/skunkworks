use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileGene {
    pub x: f32,
    pub y: f32,
    pub feed: f32,
    pub kill: f32,
    pub path: String,
}

pub fn scan_codebase(root: &Path) -> Result<Vec<FileGene>> {
    let mut genes = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            // Ignore hidden folders and target
            !path.components().any(|c| {
                let s = c.as_os_str().to_string_lossy();
                s == "target" || s == ".git" || s == ".ds_store"
            })
        })
    {
        if entry.file_type().is_file() {
            let path_str = entry.path().to_string_lossy().to_string();
            let metadata = entry.metadata()?;
            let size = metadata.len();

            // Calculate Position from Path Hash
            let mut hasher = DefaultHasher::new();
            path_str.hash(&mut hasher);
            let hash = hasher.finish();

            // Map to [0, 1]
            // We use different bits for X and Y to avoid correlation
            let x_raw = (hash & 0xFFFF) as f32;
            let y_raw = ((hash >> 16) & 0xFFFF) as f32;

            let x = x_raw / 65536.0;
            let y = y_raw / 65536.0;

            // Calculate Feed Rate from File Size (Log Scale)
            // Range: 0.01 to 0.1
            // Size 0 -> 0.01
            // Size 1MB -> ~0.08
            let size_clamped = (size as f32).max(1.0);
            let log_size = size_clamped.ln();
            let feed = 0.01 + (log_size / 20.0).clamp(0.0, 0.09);

            // Calculate Kill Rate from Extension (or just random stable range)
            // Range: 0.045 to 0.07
            let ext = entry
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let mut ext_hasher = DefaultHasher::new();
            ext.hash(&mut ext_hasher);
            let ext_hash = ext_hasher.finish();

            let kill_offset = (ext_hash % 100) as f32 / 100.0; // 0.0 to 1.0
            let kill = 0.045 + (kill_offset * 0.025);

            genes.push(FileGene {
                x,
                y,
                feed,
                kill,
                path: path_str,
            });
        }
    }

    Ok(genes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_scan_codebase() -> Result<()> {
        let test_dir = "test_scan_dir";
        fs::create_dir_all(test_dir)?;

        // Create dummy files
        let file1 = format!("{}/test1.txt", test_dir);
        let mut f1 = File::create(&file1)?;
        f1.write_all(b"hello world")?;

        let file2 = format!("{}/test2.rs", test_dir);
        let mut f2 = File::create(&file2)?;
        f2.write_all(vec![0; 1024].as_slice())?; // Larger file

        let genes = scan_codebase(Path::new(test_dir))?;

        assert_eq!(genes.len(), 2);

        for gene in &genes {
            assert!(gene.x >= 0.0 && gene.x <= 1.0);
            assert!(gene.y >= 0.0 && gene.y <= 1.0);
            assert!(gene.feed >= 0.01 && gene.feed <= 0.1);
            assert!(gene.kill >= 0.045 && gene.kill <= 0.07);
        }

        fs::remove_dir_all(test_dir)?;
        Ok(())
    }
}
