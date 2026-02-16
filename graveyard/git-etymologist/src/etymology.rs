use crate::phonology;
use anyhow::{Context, Result};
use git2::{Blob, Commit, DiffOptions, Patch, Repository};
use std::path::Path;

pub struct LineHistory {
    pub lines: Vec<String>,
    pub mapping: Vec<Option<usize>>,
}

pub struct CodeRiver {
    pub layers: Vec<LineHistory>,
    pub commits: Vec<String>,
    pub authors: Vec<String>,
    #[allow(dead_code)]
    pub messages: Vec<String>,
}

pub fn trace_history(repo_path: &str, file_path: &str) -> Result<CodeRiver> {
    let repo = Repository::open(repo_path).context("Failed to open repo")?;
    let path = Path::new(file_path);

    // 1. Walk history from HEAD
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits = Vec::new();
    for oid in revwalk.take(50) {
        commits.push(oid?);
    }

    let mut layers = Vec::new();
    let mut commit_ids = Vec::new();
    let mut authors = Vec::new();
    let mut messages = Vec::new();

    // First, load the HEAD state (Child 0)
    if let Some(head_oid) = commits.first() {
        let commit = repo.find_commit(*head_oid)?;
        let blob = get_file_blob(&repo, &commit, path)?;
        let content = String::from_utf8_lossy(blob.content()).to_string();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        layers.push(LineHistory {
            lines,
            mapping: Vec::new(),
        });
        commit_ids.push(head_oid.to_string());
        authors.push(commit.author().name().unwrap_or("Unknown").to_string());
        messages.push(commit.summary().unwrap_or("").to_string());
    }

    for i in 0..commits.len() - 1 {
        let child_oid = commits[i];
        let parent_oid = commits[i + 1];

        let child_commit = repo.find_commit(child_oid)?;
        let parent_commit = repo.find_commit(parent_oid)?;

        commit_ids.push(parent_oid.to_string());
        authors.push(
            parent_commit
                .author()
                .name()
                .unwrap_or("Unknown")
                .to_string(),
        );
        messages.push(parent_commit.summary().unwrap_or("").to_string());

        // Get blobs for content
        let parent_blob = get_file_blob(&repo, &parent_commit, path)?;

        // Parent lines
        let p_content = String::from_utf8_lossy(parent_blob.content()).to_string();
        let p_lines: Vec<String> = p_content.lines().map(|s| s.to_string()).collect();

        // Calculate mapping Child -> Parent using Tree Diff
        let child_tree = child_commit.tree()?;
        let parent_tree = parent_commit.tree()?;

        let mut mapping = compute_mapping(&repo, &parent_tree, &child_tree, file_path)?;

        // Handle identity case (no diff)
        if let Some(child_layer) = layers.last_mut() {
            if mapping.is_empty() && !child_layer.lines.is_empty() {
                // If diff is empty, assume identity mapping if file exists
                // Note: If file was added in child, parent blob might be empty?
                // If parent blob is empty, p_lines is empty.
                // But if diff is empty, blobs are identical.
                // So p_lines len == child_lines len.
                mapping = (0..child_layer.lines.len()).map(Some).collect();
            }
            child_layer.mapping = mapping;
        }

        layers.push(LineHistory {
            lines: p_lines,
            mapping: Vec::new(),
        });
    }

    Ok(CodeRiver {
        layers,
        commits: commit_ids,
        authors,
        messages,
    })
}

fn get_file_blob<'a>(repo: &'a Repository, commit: &Commit, path: &Path) -> Result<Blob<'a>> {
    let tree = commit.tree()?;
    match tree.get_path(path) {
        Ok(entry) => {
            let object = entry.to_object(repo)?;
            let blob = object
                .into_blob()
                .map_err(|_| anyhow::anyhow!("Not a blob"))?;
            Ok(blob)
        }
        Err(_) => {
            // File might not exist in this commit (added later)
            // Return empty blob? Or error?
            // If error, trace_history loop will fail.
            // We should handle "File creation" event.
            // But for simplicity, we trace back until file disappears.
            // If get_path fails, we stop?
            // trace_history calls get_file_blob for parent.
            // If parent doesn't have file, we should probably stop or return empty blob.
            // Let's return error and handle it in loop?
            // Actually, better to error and let trace_history handle it if we want to stop.
            Err(anyhow::anyhow!("File not found"))
        }
    }
}

fn compute_mapping(
    repo: &Repository,
    old_tree: &git2::Tree,
    new_tree: &git2::Tree,
    path: &str,
) -> Result<Vec<Option<usize>>> {
    let mut opts = DiffOptions::new();
    opts.context_lines(u32::MAX);
    opts.pathspec(path);

    let diff = repo.diff_tree_to_tree(Some(old_tree), Some(new_tree), Some(&mut opts))?;

    if diff.deltas().len() == 0 {
        return Ok(Vec::new());
    }

    let mut mapping = Vec::new();
    let mut deleted_buffer: Vec<(usize, String)> = Vec::new();
    let mut added_buffer: Vec<(usize, String)> = Vec::new();

    let patch = Patch::from_diff(&diff, 0)?;
    if let Some(patch) = patch {
        for h in 0..patch.num_hunks() {
            let (_hunk, _) = patch.hunk(h)?;
            for l in 0..patch.num_lines_in_hunk(h)? {
                let line = patch.line_in_hunk(h, l)?;
                let origin = line.origin();

                match origin {
                    ' ' => {
                        match_buffers(&mut mapping, &mut deleted_buffer, &mut added_buffer);
                        if let (Some(old), Some(new)) = (line.old_lineno(), line.new_lineno()) {
                            ensure_size(&mut mapping, new as usize);
                            mapping[new as usize - 1] = Some(old as usize - 1);
                        }
                    }
                    '-' => {
                        if let Some(old) = line.old_lineno() {
                            let content = String::from_utf8_lossy(line.content())
                                .trim_end()
                                .to_string();
                            deleted_buffer.push((old as usize - 1, content));
                        }
                    }
                    '+' => {
                        if let Some(new) = line.new_lineno() {
                            let content = String::from_utf8_lossy(line.content())
                                .trim_end()
                                .to_string();
                            added_buffer.push((new as usize - 1, content));
                        }
                    }
                    _ => {}
                }
            }
            match_buffers(&mut mapping, &mut deleted_buffer, &mut added_buffer);
        }
    }

    Ok(mapping)
}

fn ensure_size(vec: &mut Vec<Option<usize>>, size: usize) {
    if vec.len() < size {
        vec.resize(size, None);
    }
}

fn match_buffers(
    mapping: &mut Vec<Option<usize>>,
    deleted: &mut Vec<(usize, String)>,
    added: &mut Vec<(usize, String)>,
) {
    if deleted.is_empty() || added.is_empty() {
        deleted.clear();
        added.clear();
        return;
    }

    for (new_idx, new_text) in added.iter() {
        ensure_size(mapping, *new_idx + 1);

        let mut best_old = None;
        let mut min_dist = f32::MAX;

        for (old_idx, old_text) in deleted.iter() {
            let norm_dist = phonology::distance(new_text, old_text);

            if norm_dist < 0.6 && norm_dist < min_dist {
                min_dist = norm_dist;
                best_old = Some(*old_idx);
            }
        }

        if let Some(old_idx) = best_old {
            mapping[*new_idx] = Some(old_idx);
        }
    }

    deleted.clear();
    added.clear();
}
