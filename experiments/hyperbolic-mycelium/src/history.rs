use git2::{Repository, Sort};
use strsim::normalized_levenshtein;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CommitNode {
    pub oid: String,
    pub message: String,
    pub parents: Vec<String>,
    pub timestamp: i64,
}

pub fn load_repo(path: &str) -> Result<Vec<CommitNode>, git2::Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;

    // Attempt to push head, if it fails (e.g. empty repo), return empty vec
    if revwalk.push_head().is_err() {
        return Ok(Vec::new());
    }

    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

    let mut nodes = Vec::new();

    for oid_result in revwalk {
        if let Ok(oid) = oid_result {
            if let Ok(commit) = repo.find_commit(oid) {
                let message = commit.summary().unwrap_or("").to_string();
                let parents = commit.parent_ids().map(|id| id.to_string()).collect();
                let timestamp = commit.time().seconds();

                nodes.push(CommitNode {
                    oid: oid.to_string(),
                    message,
                    parents,
                    timestamp,
                });
            }
        }
    }

    // Reverse to have oldest first (root)
    nodes.reverse();

    Ok(nodes)
}

pub fn calculate_similarity(a: &str, b: &str) -> f32 {
    normalized_levenshtein(a, b) as f32
}
