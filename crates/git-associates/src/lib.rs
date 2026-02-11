pub mod model;

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use git2::{Repository, Sort, DiffFlags};
use model::{Commit, CommitStats, FileChange, DiffStats, Hunk, LineChange};
use std::path::Path;

pub struct GitModel {
    repo: Repository,
}

impl GitModel {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path).context("Failed to discover git repository")?;
        Ok(Self { repo })
    }

    pub fn history(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history_internal(limit, false)
    }

    pub fn history_with_diffs(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history_internal(limit, true)
    }

    fn history_internal(&self, limit: usize, include_hunks: bool) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk().context("Failed to create revwalker")?;
        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        let mut count = 0;

        for oid in revwalk {
            if count >= limit {
                break;
            }
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            let hash = oid.to_string();
            let short_hash = hash.chars().take(7).collect();
            let author = commit.author().name().unwrap_or("Unknown").to_string();
            let message = commit.message().unwrap_or("").trim().to_string();
            let timestamp = Utc.timestamp_opt(commit.time().seconds(), 0).unwrap();

            let parents: Vec<String> = commit.parents().map(|p| p.id().to_string()).collect();

            // Stats
            let (stats, files) = self.get_commit_diff(&commit, include_hunks).unwrap_or_default();

            commits.push(Commit {
                hash,
                short_hash,
                author,
                message,
                timestamp,
                parents,
                stats: Some(stats),
                files,
            });

            count += 1;
        }

        Ok(commits)
    }

    pub fn crawl_graph(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history(limit)
    }

    fn get_commit_diff(&self, commit: &git2::Commit, include_hunks: bool) -> Result<(CommitStats, Vec<FileChange>)> {
        let tree = commit.tree()?;
        let parent = commit.parent(0).ok();
        let parent_tree = parent.as_ref().and_then(|p| p.tree().ok());

        let diff = self.repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        let mut files = Vec::new();
        let mut total_insertions = 0;
        let mut total_deletions = 0;

        // Populate diff internal structure (optional for stats but needed for hunks sometimes)
        // diff.print(git2::DiffFormat::Patch, |_delta, _hunk, _line| true)?;

        for i in 0..diff.deltas().len() {
            if let Ok(patch) = git2::Patch::from_diff(&diff, i) {
                if let Some(patch) = patch {
                     let delta = patch.delta();
                    let path = delta.new_file().path().or(delta.old_file().path())
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    let extension = Path::new(&path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_string();

                    let stats = patch.line_stats().unwrap_or((0, 0, 0));
                    let insertions = stats.1;
                    let deletions = stats.2;

                    total_insertions += insertions;
                    total_deletions += deletions;

                    let mut hunks = Vec::new();
                    if include_hunks {
                         for h_idx in 0..patch.num_hunks() {
                            if let Ok((hunk_info, lines_count)) = patch.hunk(h_idx) {
                                 let mut hunk_lines = Vec::new();
                                 for l_idx in 0..lines_count {
                                     if let Ok(line) = patch.line_in_hunk(h_idx, l_idx) {
                                         let content = std::str::from_utf8(line.content()).unwrap_or("").to_string();
                                         match line.origin() {
                                             '+' => hunk_lines.push(LineChange::Added(content)),
                                             '-' => hunk_lines.push(LineChange::Removed(content)),
                                             ' ' => hunk_lines.push(LineChange::Context(content)),
                                             _ => {},
                                         }
                                     }
                                 }
                                 hunks.push(Hunk {
                                     header: std::str::from_utf8(hunk_info.header()).unwrap_or("").to_string(),
                                     lines: hunk_lines,
                                 });
                            }
                        }
                    }

                    files.push(FileChange {
                        path,
                        extension,
                        insertions,
                        deletions,
                        is_binary: delta.flags().contains(DiffFlags::BINARY),
                        hunks,
                    });
                }
            }
        }

        Ok((CommitStats {
            insertions: total_insertions,
            deletions: total_deletions,
            files_changed: files.len(),
        }, files))
    }

    pub fn diff_workdir(&self) -> Result<DiffStats> {
        let mut diff_opts = git2::DiffOptions::new();
        diff_opts.include_untracked(true);

        let head = self.repo.head().ok();
        let tree = if let Some(h) = head {
            if let Ok(peel) = h.peel_to_tree() {
                Some(peel)
            } else {
                None
            }
        } else {
            None
        };

        let diff = self.repo.diff_tree_to_workdir_with_index(tree.as_ref(), Some(&mut diff_opts))?;

        let mut files = Vec::new();
        let mut total_added = 0;
        let mut total_removed = 0;

        for i in 0..diff.deltas().len() {
             if let Ok(patch) = git2::Patch::from_diff(&diff, i) {
                if let Some(patch) = patch {
                    let delta = patch.delta();
                    let path = delta.new_file().path().or(delta.old_file().path())
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    let extension = Path::new(&path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_string();

                    let stats = patch.line_stats().unwrap_or((0, 0, 0));
                    let added = stats.1;
                    let removed = stats.2;

                    total_added += added;
                    total_removed += removed;

                    // Extract Hunks (Always include for workdir diff as it is usually small)
                    let mut hunks = Vec::new();
                    for h_idx in 0..patch.num_hunks() {
                        if let Ok((hunk_info, lines_count)) = patch.hunk(h_idx) {
                             let mut hunk_lines = Vec::new();
                             for l_idx in 0..lines_count {
                                 if let Ok(line) = patch.line_in_hunk(h_idx, l_idx) {
                                     let content = std::str::from_utf8(line.content()).unwrap_or("").to_string();
                                     match line.origin() {
                                         '+' => hunk_lines.push(LineChange::Added(content)),
                                         '-' => hunk_lines.push(LineChange::Removed(content)),
                                         ' ' => hunk_lines.push(LineChange::Context(content)),
                                         _ => {},
                                     }
                                 }
                             }
                             hunks.push(Hunk {
                                 header: std::str::from_utf8(hunk_info.header()).unwrap_or("").to_string(),
                                 lines: hunk_lines,
                             });
                        }
                    }

                    files.push(FileChange {
                        path,
                        extension,
                        insertions: added,
                        deletions: removed,
                        is_binary: delta.flags().contains(DiffFlags::BINARY),
                        hunks,
                    });
                }
            }
        }

        Ok(DiffStats {
            files,
            total_added,
            total_removed,
        })
    }
}
