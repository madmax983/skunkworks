use chrono::{DateTime, TimeZone, Utc};
use git2::{Repository, Sort};
use std::env;

#[derive(Clone, Debug)]
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub message: String,
}

pub fn get_commit_history() -> anyhow::Result<Vec<Commit>> {
    let repo_path = env::current_dir()?;
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    revwalk.set_sorting(Sort::TIME | Sort::REVERSE)?;
    revwalk.push_head()?;

    let mut commits = Vec::new();
    let mut count = 0;

    // We only take the last 50 commits to avoid clustering too much
    // Or we can take all of them and sample. Let's take up to 200.
    for id in revwalk.flatten() {
        if let Ok(commit) = repo.find_commit(id) {
            let author = commit.author();
            let time = commit.time();
            let date = Utc.timestamp_opt(time.seconds(), 0).unwrap();

            commits.push(Commit {
                hash: id.to_string(),
                author: author.name().unwrap_or("Unknown").to_string(),
                date,
                message: commit.summary().unwrap_or("").to_string(),
            });

            count += 1;
            if count >= 200 {
                break;
            }
        }
    }

    Ok(commits)
}
