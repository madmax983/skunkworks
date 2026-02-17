use anyhow::Result;
pub use git_associates::{Commit, FileChange, GitModel};

pub fn get_repo_history(path: &str, limit: usize) -> Result<Vec<Commit>> {
    let model = GitModel::open(path)?;
    // We need history with diffs because vis/audio use file changes
    let mut history = model.history_with_diffs(limit)?;

    // Playback history: Oldest -> Newest
    history.reverse();

    Ok(history)
}
