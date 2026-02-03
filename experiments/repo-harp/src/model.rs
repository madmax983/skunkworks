use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::dsp::KarplusStrong;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct RepoString {
    pub path: PathBuf,
    pub file_size: u64,
    pub age_seconds: u64,
    pub frequency: f32,
}

impl RepoString {
    pub fn to_dsp(&self, sample_rate: f32) -> KarplusStrong {
        KarplusStrong::new(self.frequency, sample_rate)
    }
}

pub fn scan_directory(root: &Path) -> Vec<RepoString> {
    let mut results = Vec::new();
    let now = SystemTime::now();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let metadata = entry.metadata().ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

            // Skip empty files
            if size == 0 { continue; }

            let modified = metadata
                .and_then(|m| m.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            let age = now.duration_since(modified).unwrap_or_default().as_secs();

            // Calculate Frequency here
            let size_clamped = (size as f32).clamp(100.0, 1_000_000.0);
            let log_size = size_clamped.log10();
            let t = (log_size - 2.0) / 4.0; // 0.0 to 1.0
            let freq = 880.0 * 0.5_f32.powf(t * 4.0);

            results.push(RepoString {
                path: entry.path().to_path_buf(),
                file_size: size,
                age_seconds: age,
                frequency: freq,
            });

            // Limit to top 50 files for performance/visual sanity
            if results.len() >= 50 {
                break;
            }
        }
    }

    // Sort by path for stability
    results.sort_by(|a, b| a.path.cmp(&b.path));

    results
}
