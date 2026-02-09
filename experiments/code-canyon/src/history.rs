use git2::{DiffOptions, Oid, Repository};
use std::path::{Path, PathBuf};

pub struct History {
    repo: Repository,
    commits: Vec<Oid>,
    current_index: usize,
}

impl History {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        let repo = Repository::open(path)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;

        let commits: Result<Vec<Oid>, git2::Error> = revwalk.collect();
        let commits = commits?;

        Ok(Self {
            repo,
            commits,
            current_index: 0,
        })
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn total_commits(&self) -> usize {
        self.commits.len()
    }

    pub fn next_commit(&mut self) -> Option<Vec<PathBuf>> {
        if self.current_index >= self.commits.len() {
            return None;
        }

        let oid = self.commits[self.current_index];
        self.current_index += 1;

        let commit = self.repo.find_commit(oid).ok()?;
        let tree = commit.tree().ok()?;

        let mut diff_paths = Vec::new();

        match commit.parent(0) {
            Ok(parent) => {
                let parent_tree = parent.tree().ok()?;
                let diff = self
                    .repo
                    .diff_tree_to_tree(
                        Some(&parent_tree),
                        Some(&tree),
                        Some(&mut DiffOptions::new()),
                    )
                    .ok()?;

                diff.foreach(
                    &mut |delta, _| {
                        if let Some(path) = delta.new_file().path() {
                            diff_paths.push(path.to_path_buf());
                        } else if let Some(path) = delta.old_file().path() {
                            // Also count deleted/renamed files as "touched"
                            diff_paths.push(path.to_path_buf());
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )
                .ok()?;
            }
            Err(_) => {
                // Initial commit: diff against empty tree or just list all files
                // Listing all files in tree is easier via diff against None
                let diff = self
                    .repo
                    .diff_tree_to_tree(None, Some(&tree), Some(&mut DiffOptions::new()))
                    .ok()?;

                diff.foreach(
                    &mut |delta, _| {
                        if let Some(path) = delta.new_file().path() {
                            diff_paths.push(path.to_path_buf());
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )
                .ok()?;
            }
        }

        Some(diff_paths)
    }
}
