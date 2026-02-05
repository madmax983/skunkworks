use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub message: String,
}

pub struct GitHistory {
    pub commits: Vec<Commit>,
}

impl GitHistory {
    pub fn load() -> Result<Self> {
        let output = Command::new("git")
            .args([
                "log",
                "--pretty=format:%H|%an|%s",
                "--reverse",
                "--no-merges", // Optional: skip merges to keep it linear?
            ])
            .output()
            .context("Failed to execute git log")?;

        let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 in git log output")?;

        let commits = parse_git_log(&stdout);

        Ok(Self { commits })
    }
}

fn parse_git_log(output: &str) -> Vec<Commit> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() == 3 {
                Some(Commit {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    message: parts[2].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_log() {
        let input = "a1b2c3d|Alice|Initial commit\ne5f6g7h|Bob|Fix bug";
        let commits = parse_git_log(input);

        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].hash, "a1b2c3d");
        assert_eq!(commits[0].author, "Alice");
        assert_eq!(commits[0].message, "Initial commit");
        assert_eq!(commits[1].hash, "e5f6g7h");
    }
}
