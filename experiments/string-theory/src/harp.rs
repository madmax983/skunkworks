use walkdir::WalkDir;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct StringEntity {
    pub path: PathBuf,
    pub size: u64,
    pub freq: f32,
    pub energy: f32, // For visualization state
}

pub fn scan_directory(root: &str) -> Vec<StringEntity> {
    let mut strings = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            if size == 0 { continue; }

            // Map size to frequency
            // 50 samples (882Hz) to 1050 samples (42Hz)
            let delay_samples = 50 + (size % 1000);
            let freq = 44100.0 / delay_samples as f32;

            strings.push(StringEntity {
                path: entry.path().to_path_buf(),
                size,
                freq,
                energy: 0.0,
            });
        }
    }
    // Sort by path for stable order
    strings.sort_by(|a, b| a.path.cmp(&b.path));
    strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let strings = scan_directory(".");
        assert!(!strings.is_empty(), "Should find files in current directory");
        for s in strings.iter().take(5) {
            println!("Found string: {:?} Size: {} Freq: {}", s.path, s.size, s.freq);
            assert!(s.freq > 0.0);
            assert!(s.freq < 22050.0);
        }
    }
}
