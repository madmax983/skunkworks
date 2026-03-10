use git_associates::GitModel;
use git2::{Repository, Signature};
use std::fs;
use std::path::Path;

#[test]
fn test_git_associates_coverage() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();

    // Initialize a new repo
    let repo = Repository::init(repo_path).unwrap();

    let sig = Signature::now("Test User", "test@example.com").unwrap();

    // Create a file and commit it
    let file_path = repo_path.join("test.txt");
    fs::write(&file_path, "initial content\n").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let commit_id1 = repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();

    // Modify the file and commit again
    fs::write(&file_path, "initial content\nadded content\n").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let parent_commit = repo.find_commit(commit_id1).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Second commit", &tree, &[&parent_commit]).unwrap();

    // Modify the file in workdir (uncommitted change)
    fs::write(&file_path, "initial content\nadded content\nmore uncommitted content\n").unwrap();

    // Create an untracked file
    let untracked_path = repo_path.join("untracked.rs");
    fs::write(&untracked_path, "fn main() {}\n").unwrap();

    // Now test the GitModel API
    let model = GitModel::open(repo_path).unwrap();

    // Test diff_workdir
    let diff_stats = model.diff_workdir().unwrap();
    println!("{:?}", diff_stats);

    // Test history
    let history = model.history(10).unwrap();
    assert_eq!(history.len(), 2);
    assert!(history[0].stats.is_none());

    // Test history_with_diffs
    let history_diff = model.history_with_diffs(10).unwrap();
    assert_eq!(history_diff.len(), 2);
    assert!(history_diff[0].stats.is_some());
    assert!(!history_diff[0].files.is_empty());

    // Check hunk extraction
    let modified_file = history_diff[0].files.iter().find(|f| f.path == "test.txt").unwrap();
    assert!(!modified_file.hunks.is_empty());

    let has_added_line = modified_file.hunks.iter().any(|h| {
        h.lines.iter().any(|l| matches!(l, git_associates::LineChange::Added(_)))
    });
    assert!(has_added_line);
}

#[test]
fn test_git_associates_edge_cases() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();

    let sig = Signature::now("Test User", "test@example.com").unwrap();

    // 1. Commit a binary file to test `is_binary` flag
    let file_path = repo_path.join("binary_file.bin");
    fs::write(&file_path, &[0, 159, 146, 150]).unwrap(); // Invalid UTF-8

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("binary_file.bin")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let commit_id1 = repo.commit(Some("HEAD"), &sig, &sig, "Add binary file", &tree, &[]).unwrap();

    // Modify binary file
    fs::write(&file_path, &[0, 159, 146, 151]).unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("binary_file.bin")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let parent_commit = repo.find_commit(commit_id1).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Modify binary file", &tree, &[&parent_commit]).unwrap();

    // Now test the GitModel API
    let model = GitModel::open(repo_path).unwrap();

    let history_diff = model.history_with_diffs(10).unwrap();

    let modified_file = history_diff[0].files.iter().find(|f| f.path == "binary_file.bin").unwrap();
    assert!(modified_file.is_binary);
}
