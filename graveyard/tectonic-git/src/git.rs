use anyhow::Result;
use git_associates::{model::LineChange, GitModel};

#[derive(Debug, Clone)]
pub struct CommitData {
    pub hash: String,
    pub timestamp: i64,
    pub stress_level: f64,
    pub details: String,
}

pub struct GitScanner;

impl GitScanner {
    pub fn scan() -> Result<Vec<CommitData>> {
        // Try to open git repo, if fails return empty list? Original panicked or returned Err.
        let model = GitModel::open(".")?;
        let commits = model.history_with_diffs(500)?; // Load 500 commits with diffs

        let mut data = Vec::new();

        for commit in commits {
            let mut stress = 0.0;
            // Analyze files
            for file in &commit.files {
                for hunk in &file.hunks {
                    for line in &hunk.lines {
                        if let LineChange::Added(content) = line {
                            if content.contains("TODO") {
                                stress += 1.0;
                            }
                            if content.contains("FIXME") {
                                stress += 2.0;
                            }
                            if content.contains("unwrap()") {
                                stress += 1.5;
                            }
                            if content.contains("panic!") {
                                stress += 5.0;
                            }
                            if content.contains("unsafe") {
                                stress += 3.0;
                            }
                        }
                    }
                }
            }

            data.push(CommitData {
                hash: commit.hash,
                timestamp: commit.timestamp.timestamp(),
                stress_level: stress,
                details: format!("Stress: {:.1}", stress),
            });
        }

        // Replay order: Oldest -> Newest
        data.reverse();

        Ok(data)
    }
}
