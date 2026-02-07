use std::process::Command;
use anyhow::{Result, Context};
use std::path::Path;

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
