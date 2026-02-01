use anyhow::{Result, Context};
use git2::{Repository, Sort};

#[derive(Debug, Clone)]
pub struct MusicalCommit {
    pub hash: String,
    pub author: String,
    pub timestamp: i64,
}

pub fn harvest_repo(path: &str) -> Result<Vec<MusicalCommit>> {
    let repo = Repository::discover(path).context("Failed to open git repo")?;
    let mut walker = repo.revwalk().context("Failed to create revwalker")?;
    walker.set_sorting(Sort::TIME)?;
    walker.push_head()?;

    let mut musical_commits = Vec::new();

    for oid in walker {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let hash = oid.to_string();
        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let timestamp = commit.time().seconds();

        musical_commits.push(MusicalCommit {
            hash,
            author,
            timestamp,
        });
    }

    // Playback history: Oldest -> Newest
    musical_commits.reverse();

    Ok(musical_commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvest_commits() {
        // We assume the current directory is a git repo (which it is)
        let commits = harvest_repo(".").expect("Failed to harvest repo");
        assert!(!commits.is_empty(), "Should have found commits");

        let first = &commits[0];
        assert_eq!(first.hash.len(), 40, "Hash should be 40 chars");
    }
}
