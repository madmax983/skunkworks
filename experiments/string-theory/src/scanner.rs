use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileString {
    pub path: PathBuf,
    pub size: u64,
    pub extension: Option<String>,
}

impl FileString {
    pub fn get_frequency(&self) -> f32 {
        // Map size to frequency. Smaller files = higher pitch.
        // Size range: 10 bytes to ...
        // Let's say 100 bytes = 1000 Hz
        // 10000 bytes = 100 Hz

        // Logarithmic scale might be better?
        // f = 1000 * (100 / size)

        let s = self.size.max(50) as f32; // Minimum size 50 bytes
        let f = 200_000.0 / s;
        f.clamp(60.0, 2000.0)
    }

    pub fn get_decay(&self) -> f32 {
        match self.extension.as_deref() {
            Some("rs") => 0.996,                // Very resonant (Rust is hard)
            Some("toml") => 0.99,               // Configuration (structured)
            Some("md") => 0.98,                 // Documentation (dry)
            Some("json") => 0.985,              // Data
            Some("lock") => 0.97,               // Locked (damped)
            Some("png") | Some("jpg") => 0.992, // Images (glossy)
            _ => 0.99,
        }
    }
}

pub fn scan_directory(path: &str) -> Vec<FileString> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path_buf = entry.path().to_path_buf();
            let path_str = path_buf.to_string_lossy();

            // Skip build artifacts and hidden directories
            if path_str.contains("/target/")
                || path_str.contains("/.git/")
                || path_str.contains("/node_modules/")
            {
                continue;
            }

            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let extension = path_buf
                .extension()
                .map(|s| s.to_string_lossy().to_string());

            files.push(FileString {
                path: path_buf,
                size,
                extension,
            });
        }
    }

    // Sort by path for consistency
    files.sort_by(|a, b| a.path.cmp(&b.path));

    files
}
