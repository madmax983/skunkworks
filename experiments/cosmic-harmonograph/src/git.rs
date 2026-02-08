use anyhow::{Context, Result};
use git2::{Repository, Sort};

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub message: String,
}

pub fn get_recent_commits(limit: usize) -> Result<Vec<Commit>> {
    let repo = Repository::discover(".")
        .context("Failed to discover git repository")?;

    let mut revwalk = repo.revwalk()
        .context("Failed to create revwalk")?;

    revwalk.set_sorting(Sort::TIME)
        .context("Failed to set sorting")?;

    revwalk.push_head()
        .context("Failed to push HEAD")?;

    let mut commits = Vec::new();

    for oid in revwalk.take(limit) {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        commits.push(Commit {
            hash: oid.to_string(),
            author: commit.author().name().unwrap_or("Unknown").to_string(),
            message: commit.message().unwrap_or("").to_string(),
        });
    }

    // Reverse so we go from oldest to newest in the sample?
    // Or newest first?
    // Usually we want to iterate through history. Let's keep newest first (reverse chronological)
    // but maybe the caller wants to play them in order.
    // Let's return them as is (Newest -> Oldest).

    Ok(commits)
}
