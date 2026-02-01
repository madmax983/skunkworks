use anyhow::{Context, Result};
use git2::{Repository, Sort};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommitData {
    pub hash: String,
    pub parents: Vec<String>,
    pub author: String,
    pub time: i64,
    pub churn: usize,
}

pub fn harvest_repo(path: &str) -> Result<Vec<CommitData>> {
    // Memory mentions: "When accessing the git repository via `git2` in tests (especially from workspace sub-crates), use `Repository::discover(".")` instead of `Repository::open(".")` to correctly locate the git root."
    // I should use `discover` to find the repo root even if I pass "."
    let repo = Repository::discover(path).context("Failed to discover repository")?;

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;
    revwalk.push_head()?;

    let mut commits = Vec::new();
    let mut processed = HashMap::new();

    // Use a limit to avoid parsing massive repos forever in this experiment
    let limit = 500;

    for (i, oid) in revwalk.enumerate() {
        if i >= limit {
            break;
        }
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let hash = oid.to_string();

        // Skip if already processed (though revwalk shouldn't duplicate unless graph is weird)
        if processed.contains_key(&hash) {
            continue;
        }

        let mut parents = Vec::new();
        for p in commit.parents() {
            parents.push(p.id().to_string());
        }

        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let time = commit.time().seconds();

        // Calculate churn (simplification: lines changed in this commit vs parent)
        // Note: For merge commits, diff against first parent?
        // Or just use diff stats.
        let churn = if commit.parent_count() > 0 {
            let parent = commit.parent(0)?;
            let tree = commit.tree()?;
            let parent_tree = parent.tree()?;
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            let stats = diff.stats()?;
            stats.insertions() + stats.deletions()
        } else {
            // Initial commit
            let tree = commit.tree()?;
            let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
            let stats = diff.stats()?;
            stats.insertions() + stats.deletions()
        };

        let data = CommitData {
            hash: hash.clone(),
            parents,
            author,
            time,
            churn,
        };

        commits.push(data);
        processed.insert(hash, true);
    }

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvest_repo() {
        let commits = harvest_repo(".").expect("Failed to harvest repo");
        assert!(!commits.is_empty());
        println!("Harvested {} commits", commits.len());
        println!("First commit: {:?}", commits[0]);
    }
}
