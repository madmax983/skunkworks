use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub timestamp: i64,
    pub subject: String,
    pub files: Vec<PathBuf>,
}

pub fn get_git_log(limit: usize) -> Result<Vec<Commit>> {
    let output = Command::new("git")
        .args([
            "log",
            "--name-only",
            &format!("--format=START_COMMIT|%h|%an|%at|%s"),
            &format!("-n{}", limit),
        ])
        .output()
        .context("Failed to execute git log")?;

    if !output.status.success() {
        anyhow::bail!(
            "git log failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 in git log")?;
    parse_log(&stdout)
}

pub fn parse_log(input: &str) -> Result<Vec<Commit>> {
    let mut commits = Vec::new();
    let mut current_commit: Option<Commit> = None;

    for line in input.lines() {
        if line.starts_with("START_COMMIT|") {
            // Save previous commit if exists
            if let Some(c) = current_commit.take() {
                commits.push(c);
            }

            // Parse new commit header
            let parts: Vec<&str> = line.splitn(5, '|').collect();
            if parts.len() < 5 {
                continue;
            }

            let hash = parts[1].to_string();
            let author = parts[2].to_string();
            let timestamp = parts[3].parse::<i64>().unwrap_or(0);
            let subject = parts[4].to_string();

            current_commit = Some(Commit {
                hash,
                author,
                timestamp,
                subject,
                files: Vec::new(),
            });
        } else if !line.trim().is_empty() {
            // It's a file
            if let Some(ref mut c) = current_commit {
                c.files.push(PathBuf::from(line.trim()));
            }
        }
    }

    // Push the last one
    if let Some(c) = current_commit {
        commits.push(c);
    }

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log() {
        let input = "START_COMMIT|abc1234|Nova|1700000000|Fix bug
src/main.rs
src/lib.rs

START_COMMIT|def5678|Genesis|1700000010|Add feature
Cargo.toml
";
        let commits = parse_log(input).unwrap();
        assert_eq!(commits.len(), 2);

        assert_eq!(commits[0].hash, "abc1234");
        assert_eq!(commits[0].author, "Nova");
        assert_eq!(commits[0].files.len(), 2);
        assert_eq!(commits[0].files[0].to_string_lossy(), "src/main.rs");

        assert_eq!(commits[1].hash, "def5678");
        assert_eq!(commits[1].files.len(), 1);
        assert_eq!(commits[1].files[0].to_string_lossy(), "Cargo.toml");
    }
}
