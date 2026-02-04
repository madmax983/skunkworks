use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub added: usize,
    pub deleted: usize,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub timestamp: i64,
    pub changes: Vec<FileChange>,
}

pub struct GitScanner;

impl GitScanner {
    pub fn load_history() -> Result<Vec<Commit>> {
        // Use --numstat for added/deleted counts
        let output = Command::new("git")
            .args(&["log", "--numstat", "--reverse", "--format=COMMIT %H %at"])
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
                        changes: Vec::new(),
                    });
                }
            } else {
                // Parse numstat line: added\tdeleted\tpath
                // e.g. "5\t2\tsrc/main.rs"
                // Binary files: "-\t-\timages/logo.png"

                // We split by tab
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let added = parts[0].parse().unwrap_or(0);
                    let deleted = parts[1].parse().unwrap_or(0);
                    let path = PathBuf::from(parts[2]);

                    if let Some(c) = current_commit.as_mut() {
                        c.changes.push(FileChange {
                            path,
                            added,
                            deleted,
                        });
                    }
                }
            }
        }

        if let Some(c) = current_commit {
            commits.push(c);
        }

        Ok(commits)
    }

    pub fn map_path(path: &str, width: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let h = hasher.finish();
        (h as usize) % width
    }
}
