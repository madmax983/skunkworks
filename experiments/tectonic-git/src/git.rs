use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct CommitData {
    pub hash: String,
    pub timestamp: i64,
    pub stress_level: f64, // Normalized stress score
    pub details: String, // Description of what caused stress
}

pub struct GitScanner;

impl GitScanner {
    pub fn scan() -> Result<Vec<CommitData>> {
        let output = Command::new("git")
            .args([
                "log",
                "--reverse",
                "--pretty=format:COMMIT %H %at",
                "-p", // Patch output
            ])
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn git command")?
            .stdout
            .context("Failed to open git stdout")?;

        let reader = BufReader::new(output);
        let mut commits = Vec::new();

        let mut current_hash = String::new();
        let mut current_ts = 0;
        let mut current_stress = 0.0;
        let mut in_commit = false;

        for line_res in reader.lines() {
            let line = line_res?;

            if line.starts_with("COMMIT ") {
                // Save previous
                if in_commit {
                    commits.push(CommitData {
                        hash: current_hash.clone(),
                        timestamp: current_ts,
                        stress_level: current_stress,
                        details: format!("Stress: {:.1}", current_stress),
                    });
                }

                // Start new
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    current_hash = parts[1].to_string();
                    current_ts = parts[2].parse().unwrap_or(0);
                    current_stress = 0.0;
                    in_commit = true;
                }
            } else if in_commit {
                // Analyze diff lines
                if line.starts_with('+') && !line.starts_with("+++") {
                    // Check for stress keywords
                    let content = &line[1..];
                    if content.contains("TODO") { current_stress += 1.0; }
                    if content.contains("FIXME") { current_stress += 2.0; }
                    if content.contains("unwrap()") { current_stress += 1.5; }
                    if content.contains("panic!") { current_stress += 5.0; }
                    if content.contains("unsafe") { current_stress += 3.0; }
                }
            }
        }

        // Push last one
        if in_commit {
            commits.push(CommitData {
                hash: current_hash,
                timestamp: current_ts,
                stress_level: current_stress,
                details: format!("Stress: {:.1}", current_stress),
            });
        }

        Ok(commits)
    }
}
