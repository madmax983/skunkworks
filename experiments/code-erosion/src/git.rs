use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub timestamp: i64,
    pub files: Vec<PathBuf>,
}

pub struct GitScanner;

impl GitScanner {
    pub fn load_history() -> Result<Vec<Commit>> {
        let output = Command::new("git")
            .args(&["log", "--name-only", "--reverse", "--format=COMMIT %H %at"])
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn git log")?;

        let reader = BufReader::new(output.stdout.context("No stdout")?);
        let mut commits = Vec::new();
        let mut current_commit: Option<Commit> = None;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if line.starts_with("COMMIT ") {
                if let Some(c) = current_commit.take() {
                    commits.push(c);
                }
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    current_commit = Some(Commit {
                        hash: parts[1].to_string(),
                        timestamp: parts[2].parse().unwrap_or(0),
                        files: Vec::new(),
                    });
                }
            } else {
                // File path
                if let Some(c) = current_commit.as_mut() {
                    c.files.push(PathBuf::from(line.trim()));
                }
            }
        }

        if let Some(c) = current_commit {
            commits.push(c);
        }

        Ok(commits)
    }

    pub fn map_path(path: &str, width: usize, height: usize) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let h = hasher.finish();

        // Use a clearer mapping?
        // Just random hash for now.
        // In the future, maybe semantic clustering.
        let idx = (h as usize) % (width * height);
        (idx % width, idx / width)
    }
}
