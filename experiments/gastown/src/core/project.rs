use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[allow(dead_code)]
pub fn clone_repo(url: &str, path: &Path) -> Result<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!("Git clone failed");
    }
    Ok(())
}
