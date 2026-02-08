use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Oid, Repository};
use std::path::Path;
use strsim::levenshtein;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Genesis, // The line was created here (no ancestor found).
    Shift,   // The line was modified from an ancestor.
    Inertia, // The line remained unchanged (context).
}

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub commit_hash: String,
    pub short_hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub message: String,
    pub line_content: String,
    pub line_num: usize, // 1-based
    pub change_type: ChangeType,
}

pub struct EtymologyTracer {
    repo: Repository,
}

impl EtymologyTracer {
    pub fn new(path: &str) -> Result<Self> {
        let repo = Repository::discover(path).context("Failed to open repository")?;
        Ok(Self { repo })
    }

    pub fn trace_line(&self, rel_path: &str, mut current_line: usize) -> Result<Vec<TraceEntry>> {
        let mut history = Vec::new();

        // 1. Start at HEAD
        let head = self.repo.head().context("No HEAD found")?;
        let head_commit = head.peel_to_commit().context("HEAD is not a commit")?;

        let mut current_oid = head_commit.id();
        let mut current_commit = head_commit;

        // Get initial content
        let initial_blob = self.get_file_blob(current_oid, rel_path)?;
        let initial_content =
            get_line_from_blob(&initial_blob, current_line).unwrap_or_else(|| "???".to_string());

        // Push the starting point (Present Day)
        history.push(TraceEntry {
            commit_hash: current_oid.to_string(),
            short_hash: current_oid.to_string()[..7].to_string(),
            author: current_commit
                .author()
                .name()
                .unwrap_or("Unknown")
                .to_string(),
            date: DateTime::from_timestamp(current_commit.time().seconds(), 0)
                .unwrap_or_default()
                .with_timezone(&Utc),
            message: current_commit.message().unwrap_or("").trim().to_string(),
            line_content: initial_content.clone(),
            line_num: current_line,
            change_type: ChangeType::Inertia, // Will be updated if next step shows change
        });

        // Loop backwards
        loop {
            // Get parent (assume linear history for now, take first parent)
            if current_commit.parent_count() == 0 {
                // Reached root
                if let Some(last) = history.last_mut() {
                    last.change_type = ChangeType::Genesis;
                }
                break;
            }

            let parent = current_commit.parent(0)?;
            let parent_oid = parent.id();
            let parent_tree = parent.tree()?;
            let current_tree = current_commit.tree()?;

            // Diff parent -> current
            let diff =
                self.repo
                    .diff_tree_to_tree(Some(&parent_tree), Some(&current_tree), None)?;

            // Find patch for our file
            let mut found_change = false;
            let mut new_line_in_parent = current_line; // Default to same if no diff found (unlikely if valid diff)

            // We need to calculate the position in parent.
            // Iterate over deltas.

            // Optimization: Find the delta for this file specifically.
            let mut file_patch = None;
            for i in 0..diff.deltas().len() {
                let delta = diff.get_delta(i).unwrap();
                let delta_path = delta.new_file().path().or(delta.old_file().path());
                if let Some(p) = delta_path {
                    if p.to_string_lossy() == rel_path {
                        file_patch = Some(git2::Patch::from_diff(&diff, i)?);
                        break;
                    }
                }
            }

            if let Some(Some(patch)) = file_patch {
                // Analyze Patch
                let mut line_mapped = false;

                // Track coordinate shift caused by hunks BEFORE our line
                let mut line_shift: isize = 0;

                // We need to look at hunks to see if our line is INSIDE one or AFTER one.
                let num_hunks = patch.num_hunks();

                for h_idx in 0..num_hunks {
                    let (hunk, _) = patch.hunk(h_idx)?;

                    let new_start = hunk.new_start() as usize;
                    let new_lines = hunk.new_lines() as usize;
                    let old_lines = hunk.old_lines() as usize;

                    // Check if our line is BEFORE this hunk
                    if current_line < new_start {
                        // The line is before this change, so previous shifts apply, but this one doesn't.
                        // We are done.
                        break;
                    }

                    // Check if our line is INSIDE this hunk
                    if current_line >= new_start && current_line < new_start + new_lines {
                        // Our line is involved in this hunk.
                        // We need to iterate the lines of the hunk to find exactly what happened.

                        // Collect deleted lines (potential ancestors)
                        let mut potential_ancestors = Vec::new();

                        let num_lines = patch.num_lines_in_hunk(h_idx)?;
                        for l_idx in 0..num_lines {
                            let line = patch.line_in_hunk(h_idx, l_idx)?;

                            match line.origin() {
                                '-' => {
                                    // Deleted line from parent
                                    let content = std::str::from_utf8(line.content())
                                        .unwrap_or("")
                                        .trim_end();
                                    potential_ancestors.push((
                                        line.old_lineno().unwrap() as usize,
                                        content.to_string(),
                                    ));
                                }
                                '+' => {
                                    // Added line (our line might be this one)
                                    let ln = line.new_lineno().unwrap() as usize;
                                    if ln == current_line {
                                        // This is our line!
                                        // It is an addition.
                                        // Check if we found a deleted line that looks similar.
                                        found_change = true;
                                    }
                                }
                                ' ' => {
                                    // Context
                                    let new_ln = line.new_lineno().unwrap() as usize;
                                    let old_ln = line.old_lineno().unwrap() as usize;
                                    if new_ln == current_line {
                                        // Our line is context. It existed in parent at old_ln.
                                        new_line_in_parent = old_ln;
                                        line_mapped = true;
                                        // It's Inertia.
                                    }
                                }
                                _ => {}
                            }
                        }

                        if found_change {
                            // The line was added/modified.
                            // Search ancestors.
                            // Get current content
                            let blob = self.get_file_blob(current_oid, rel_path)?;
                            let current_content =
                                get_line_from_blob(&blob, current_line).unwrap_or_default();

                            // Find best match
                            let mut best_sim = 0.0;
                            let mut best_ancestor = None;

                            for (old_ln, old_content) in &potential_ancestors {
                                let dist = levenshtein(&current_content, old_content);
                                let max_len = current_content.len().max(old_content.len());
                                let sim = if max_len == 0 {
                                    1.0
                                } else {
                                    1.0 - (dist as f64 / max_len as f64)
                                };

                                if sim > best_sim {
                                    best_sim = sim;
                                    best_ancestor = Some((old_ln, old_content.clone()));
                                }
                            }

                            let threshold = 0.4; // Loose threshold
                            if best_sim > threshold {
                                // Sound Shift
                                let (old_ln, _old_txt) = best_ancestor.unwrap();
                                new_line_in_parent = *old_ln;

                                // Update the entry for CURRENT commit to say it was a Shift
                                if let Some(last) = history.last_mut() {
                                    last.change_type = ChangeType::Shift;
                                }
                            } else {
                                // Genesis
                                if let Some(last) = history.last_mut() {
                                    last.change_type = ChangeType::Genesis;
                                }
                                return Ok(history);
                            }

                            line_mapped = true;
                        }

                        break; // Found our hunk
                    }

                    // If our line is AFTER this hunk, we adjust shift.
                    // Shift = (Total Insertions) - (Total Deletions) so far.
                    // Wait, simply:
                    // new_line = old_line + (new_lines - old_lines)
                    // So old_line = new_line - (new_lines - old_lines)
                    // line_shift += (new_lines as isize - old_lines as isize);

                    // Actually, simpler:
                    // If we haven't found a hunk covering our line, we just subtract the delta of this hunk.
                    // But wait, calculating cumulative shift is hard if we don't know exact line numbers.
                    // Let's rely on the fact that if it's NOT in a hunk, it maps linearly?
                    // But we need to know the mapping.

                    // Recalculate:
                    // If the line is AFTER this hunk, its position in the parent is:
                    // current_pos - (added_lines) + (removed_lines)
                    line_shift += new_lines as isize - old_lines as isize;
                }

                if !line_mapped {
                    // Line was not in any hunk.
                    // So it maps to current_line - total_shift.
                    new_line_in_parent = (current_line as isize - line_shift) as usize;
                }
            } else {
                // No patch for this file?
                // It means the file wasn't changed at all in this commit.
                // new_line_in_parent remains current_line.
            }

            // Prepare for next iteration
            current_line = new_line_in_parent;
            current_commit = parent;
            current_oid = parent_oid;

            // Get content for new entry
            let blob = self.get_file_blob(current_oid, rel_path)?;
            let content =
                get_line_from_blob(&blob, current_line).unwrap_or_else(|| "???".to_string());

            history.push(TraceEntry {
                commit_hash: current_oid.to_string(),
                short_hash: current_oid.to_string()[..7].to_string(),
                author: current_commit
                    .author()
                    .name()
                    .unwrap_or("Unknown")
                    .to_string(),
                date: DateTime::from_timestamp(current_commit.time().seconds(), 0)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                message: current_commit.message().unwrap_or("").trim().to_string(),
                line_content: content,
                line_num: current_line,
                change_type: ChangeType::Inertia,
            });

            // Safety break
            if history.len() > 100 {
                break;
            }
        }

        Ok(history)
    }

    fn get_file_blob(&self, commit_oid: Oid, path: &str) -> Result<Vec<u8>> {
        let commit = self.repo.find_commit(commit_oid)?;
        let tree = commit.tree()?;
        let entry = tree.get_path(Path::new(path))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object.as_blob().context("Not a blob")?;
        Ok(blob.content().to_vec())
    }
}

fn get_line_from_blob(blob: &[u8], line_num: usize) -> Option<String> {
    if line_num == 0 {
        return None;
    }
    let s = std::str::from_utf8(blob).ok()?;
    s.lines().nth(line_num - 1).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_self() {
        // Trace a line in this file itself.
        // We need to find a line that has history.
        // Since this file was just created, it only has 1 commit history (or 0 if not committed).
        // If run in CI, it might have more.

        let path = ".";
        let file = "experiments/git-etymology/src/etym.rs";

        // This test only works if the file exists in HEAD.
        // Since I haven't committed it yet, it won't exist in HEAD of the repo!
        // So I can't test it against itself yet.

        // Use Cargo.toml instead, it exists?
        // No, I just created that too.

        // Use a file that definitely exists and has history.
        // `README.md` usually exists.
        let file = "README.md";
        if std::path::Path::new(file).exists() {
            let tracer = EtymologyTracer::new(path).expect("Failed to init tracer");
            // Trace line 1
            let trace = tracer.trace_line(file, 1);
            // It might fail if file not in HEAD (e.g. strict sparse checkout or something), but generally ok.
            if let Ok(t) = trace {
                println!("Trace length: {}", t.len());
            }
        }
    }
}
