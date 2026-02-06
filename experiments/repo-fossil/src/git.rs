use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub date: NaiveDateTime,
    pub message: String,
    pub author: String,
}

pub fn load_history() -> Result<Vec<Commit>> {
    // Format: Hash|Date|Author|Message
    // We limit to 100 commits to keep the "excavation site" manageable
    let output = Command::new("git")
        .args(&[
            "log",
            "--date=iso-strict",
            "--pretty=format:%H|%ad|%an|%s",
            "-n", "100",
        ])
        .output()
        .context("Failed to execute git log")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            let hash = parts[0].to_string();
            let date_str = parts[1];
            let author = parts[2].to_string();
            let message = parts[3..].join("|"); // Rejoin in case message contained |

            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                commits.push(Commit {
                    hash,
                    date: date.naive_utc(),
                    author,
                    message,
                });
            }
        }
    }

    Ok(commits)
}

pub fn get_files_in_commit(hash: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(&["ls-tree", "-r", "--name-only", hash])
        .output()
        .context("Failed to execute git ls-tree")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Filter out non-text files roughly by extension or just ignore
    // For now we accept everything, but we might fail to read binary later
    let files: Vec<String> = stdout
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(files)
}

pub fn get_file_content(hash: &str, path: &str) -> Result<String> {
    let spec = format!("{}:{}", hash, path);
    let output = Command::new("git")
        .args(&["show", &spec])
        .output()
        .context("Failed to execute git show")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to read file content"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
