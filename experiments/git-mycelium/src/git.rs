use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub files_changed: Vec<String>,
}

pub fn get_commit_history() -> Result<Vec<Commit>> {
    let output = Command::new("git")
        .args([
            "log",
            "--pretty=format:%H|%an",
            "--name-only",
            "-n",
            "100", // Limit to 100 for speed
        ])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_git_log(&stdout)
}

fn parse_git_log(output: &str) -> Result<Vec<Commit>> {
    let mut commits = Vec::new();
    let mut current_commit = None;

    for line in output.lines() {
        if line.contains('|') && line.len() > 10 {
            // Probably a commit line (Hash | Author)
            if let Some(c) = current_commit.take() {
                commits.push(c);
            }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                current_commit = Some(Commit {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    files_changed: Vec::new(),
                });
            }
        } else if !line.trim().is_empty() {
            // Probably a file changed
            if let Some(ref mut c) = current_commit {
                c.files_changed.push(line.trim().to_string());
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
    fn test_parse_git_log() {
        let input = "a1b2c3d|Alice\nsrc/main.rs\nsrc/lib.rs\n\ne5f6g7h|Bob\nCargo.toml\n";
        let commits = parse_git_log(input).unwrap();

        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].hash, "a1b2c3d");
        assert_eq!(commits[0].author, "Alice");
        assert_eq!(commits[0].files_changed.len(), 2);
        assert_eq!(commits[0].files_changed[0], "src/main.rs");

        assert_eq!(commits[1].hash, "e5f6g7h");
        assert_eq!(commits[1].author, "Bob");
        assert_eq!(commits[1].files_changed.len(), 1);
        assert_eq!(commits[1].files_changed[0], "Cargo.toml");
    }
}
