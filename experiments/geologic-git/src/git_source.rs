use anyhow::Result;
use git_associates::{GitModel, model::Commit};
use std::path::Path;

pub fn fetch_history(path: &Path, limit: usize) -> Result<Vec<Commit>> {
    let model = GitModel::open(path)?;
    // We want diffs because we need insertion counts for uplift.
    model.history_with_diffs(limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_history() {
        // We run this test from the workspace root or the crate root.
        // "." should resolve to the crate root if run via cargo test within the crate,
        // or workspace root if run from workspace.
        // To be safe, let's use the current directory, which should be inside a git repo (the workspace).
        let path = Path::new(".");
        let commits = fetch_history(path, 5);

        // If we are in a fresh container without .git, this might fail.
        // But the environment is a git repo.
        match commits {
            Ok(c) => {
                assert!(!c.is_empty(), "Should return at least one commit");
                println!("Fetched {} commits", c.len());
                if let Some(first) = c.first() {
                    assert!(first.stats.is_some(), "Should have diff stats");
                }
            },
            Err(e) => {
                // If run in an environment without git history (e.g. some CI), we might want to skip or fail gracefully.
                // But for this environment, it should work.
                panic!("Failed to fetch history: {}", e);
            }
        }
    }
}
