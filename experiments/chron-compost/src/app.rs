use anyhow::Result;
use std::fs;
use std::io::Read;

pub struct App {
    pub path: String,
    pub content: Vec<String>,
}

impl App {
    pub fn new(path: String) -> Result<Self> {
        let mut content_str = String::new();
        let limit = 1024 * 1024; // 1MB limit
        let file = fs::File::open(&path)?;
        let bytes_read = file.take(limit + 1).read_to_string(&mut content_str)?;

        if bytes_read as u64 > limit {
            anyhow::bail!("File is too large to read safely (exceeds 1MB)");
        }

        let content: Vec<String> = content_str.lines().map(|s| s.to_string()).collect();

        Ok(Self { path, content })
    }
}
