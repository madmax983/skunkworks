use anyhow::{Context, Result};
use std::process::Command;

pub fn get_last_commit_time() -> Result<i64> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%ct"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Git command failed"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let timestamp = stdout.trim().parse::<i64>()?;
    Ok(timestamp)
}

pub fn count_commits_last_24h() -> Result<usize> {
    let output = Command::new("git")
        .args(["log", "--since=24 hours ago", "--oneline"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Git command failed"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let count = stdout.lines().count();
    Ok(count)
}
