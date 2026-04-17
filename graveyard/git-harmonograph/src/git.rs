use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub message: String,
}

pub fn get_commit_history() -> Result<Vec<Commit>> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H|%an|%s", "-n", "100"]) // Limit to 100 for perf
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Ok(Vec::new()); // Or error? For now, empty list is safe.
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_git_log(&stdout)
}

fn parse_git_log(output: &str) -> Result<Vec<Commit>> {
    let mut commits = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            commits.push(Commit {
                hash: parts[0].to_string(),
                author: parts[1].to_string(),
                // Reassemble message in case it contained pipes (unlikely but possible)
                message: parts[2..].join("|"),
            });
        }
    }

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_log() {
        let input = "a1b2c3d|Alice|Fix bug\ne5f6g7h|Bob|Add feature with | pipe";
        let commits = parse_git_log(input).unwrap();

        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].hash, "a1b2c3d");
        assert_eq!(commits[0].author, "Alice");
        assert_eq!(commits[0].message, "Fix bug");

        assert_eq!(commits[1].hash, "e5f6g7h");
        assert_eq!(commits[1].author, "Bob");
        assert_eq!(commits[1].message, "Add feature with | pipe");
    }
}
