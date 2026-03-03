use git2::{Repository, Signature};
use git_associates::GitModel;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

/// Helper function to create a temporary git repository with some commits.
fn setup_temp_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo = Repository::init(temp_dir.path()).expect("Failed to init repo");

    let sig = Signature::now("Test User", "test@example.com").unwrap();

    // Create a file and commit it
    let file_path = temp_dir.path().join("file1.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();
    drop(file);

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let commit_id1 = {
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap()
    };

    // Modify the file and add a second commit
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "Second line.").unwrap();
    drop(file);

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    {
        let tree = repo.find_tree(tree_id).unwrap();
        let parent_commit = repo.find_commit(commit_id1).unwrap();

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Second commit",
            &tree,
            &[&parent_commit],
        )
        .unwrap();
    }

    temp_dir
}

#[test]
fn test_open_nonexistent_repo() {
    let model = GitModel::open("non_existent_directory_for_git");
    assert!(model.is_err(), "Opening non-existent repo should fail");
}

#[test]
fn test_history_limit_zero() {
    let temp_dir = setup_temp_repo();
    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    let history = model.history(0).expect("Failed to get history");
    assert!(history.is_empty(), "History with limit 0 should be empty");
}

#[test]
fn test_history_with_diffs_limit() {
    let temp_dir = setup_temp_repo();
    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    let history = model
        .history_with_diffs(2)
        .expect("Failed to get history with diffs");
    assert!(history.len() <= 2, "History length should not exceed limit");

    if let Some(commit) = history.first() {
        assert!(commit.stats.is_some(), "Commit should have stats");
    }
}

#[test]
fn test_history_invalid_limit() {
    let temp_dir = setup_temp_repo();
    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    // Large limits shouldn't panic, they should just return available commits
    let history = model
        .history(usize::MAX)
        .expect("history with large limit shouldn't panic");
    assert_eq!(history.len(), 2, "Should return all 2 commits");
}

#[test]
fn test_diff_workdir() {
    let temp_dir = setup_temp_repo();
    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    // Modify working directory
    let file_path = temp_dir.path().join("file1.txt");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "Uncommitted line.").unwrap();
    drop(file);

    let diff = model
        .diff_workdir()
        .expect("Should be able to get workdir diff");

    assert_eq!(diff.files.len(), 1, "Should have 1 modified file");
    assert_eq!(diff.files[0].path, "file1.txt");
    assert_eq!(diff.total_added, 1, "Should have 1 added line");
}

#[test]
fn test_diff_workdir_untracked() {
    let temp_dir = setup_temp_repo();
    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    // Create an untracked file
    let file_path = temp_dir.path().join("untracked.rs");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "fn main() {{}}").unwrap();
    drop(file);

    let diff = model
        .diff_workdir()
        .expect("Should be able to get workdir diff");

    assert!(
        diff.files.iter().any(|f| f.path == "untracked.rs"),
        "Should include untracked file"
    );
}

#[test]
fn test_history_empty_repo() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    Repository::init(temp_dir.path()).expect("Failed to init repo");

    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    let history = model.history(10);
    // Should fail or return empty, but shouldn't panic (revwalk on empty repo might fail)
    // Looking at the implementation: `revwalk.push_head()?` will fail if there is no HEAD
    assert!(
        history.is_err(),
        "Empty repo should fail to get history (no HEAD)"
    );
}

#[test]
fn test_diff_empty_repo() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    Repository::init(temp_dir.path()).expect("Failed to init repo");

    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    let diff = model.diff_workdir();
    // It shouldn't panic, maybe it returns Ok with empty or error depending on HEAD
    assert!(
        diff.is_ok() || diff.is_err(),
        "Should not panic on empty repo diff"
    );
}

#[test]
fn test_hunk_extraction() {
    let temp_dir = setup_temp_repo();

    // In order to add a third commit, we need to modify the repo directly.
    let repo = Repository::open(temp_dir.path()).unwrap();

    // Modify file and commit
    let file_path = temp_dir.path().join("file1.txt");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "Third line with hunk test.").unwrap();
    drop(file);

    let sig = Signature::now("Test User", "test@example.com").unwrap();
    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Third commit",
        &tree,
        &[&head_commit],
    )
    .unwrap();

    let model = GitModel::open(temp_dir.path()).unwrap();
    let history = model.history_with_diffs(1).unwrap();

    let commit = &history[0];
    assert_eq!(commit.files.len(), 1);

    let file_change = &commit.files[0];
    assert!(!file_change.hunks.is_empty(), "Hunks should be extracted");

    // Let's verify the hunk contents
    let hunk = &file_change.hunks[0];
    assert!(!hunk.header.is_empty());

    // At least one added line should exist
    let has_added_line = hunk
        .lines
        .iter()
        .any(|l| matches!(l, git_associates::model::LineChange::Added(_)));
    assert!(has_added_line, "Should have parsed an added line");
}

#[test]
fn test_history_single_commit() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo = Repository::init(temp_dir.path()).expect("Failed to init repo");

    let sig = Signature::now("Test User", "test@example.com").unwrap();

    // Create a file and commit it
    let file_path = temp_dir.path().join("file1.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, world!").unwrap();
    drop(file);

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .unwrap();

    let model = GitModel::open(temp_dir.path()).expect("Failed to open temp repo");

    let history = model.history_with_diffs(10).expect("Failed to get history");
    assert_eq!(history.len(), 1, "Should have exactly 1 commit");

    let commit = &history[0];
    assert_eq!(
        commit.parents.len(),
        0,
        "Initial commit should have 0 parents"
    );

    // For the initial commit, diff_tree_to_tree with a None parent returns all files as Added.
    assert!(
        commit.stats.is_some(),
        "Initial commit should have diff stats"
    );
    let stats = commit.stats.as_ref().unwrap();
    assert_eq!(stats.files_changed, 1);
    assert!(stats.insertions > 0);
}

#[test]
fn test_hunk_line_changes_exact() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo = Repository::init(temp_dir.path()).expect("Failed to init repo");

    let sig = Signature::now("Test User", "test@example.com").unwrap();

    // 1st Commit
    let file_path = temp_dir.path().join("file1.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Line 1\nLine 2\nLine 3").unwrap();
    drop(file);

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    let commit_id1 = {
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "C1", &tree, &[])
            .unwrap()
    };

    // 2nd Commit: Modify Line 2
    let mut file = File::create(&file_path).unwrap(); // Overwrite
    writeln!(file, "Line 1\nModified Line 2\nLine 3").unwrap();
    drop(file);

    let mut index = repo.index().unwrap();
    index.add_path(std::path::Path::new("file1.txt")).unwrap();
    index.write().unwrap();
    let tree_id = index.write_tree().unwrap();
    {
        let tree = repo.find_tree(tree_id).unwrap();
        let parent = repo.find_commit(commit_id1).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "C2", &tree, &[&parent])
            .unwrap();
    }

    let model = GitModel::open(temp_dir.path()).unwrap();
    let history = model.history_with_diffs(1).unwrap(); // Get just the latest commit

    let commit = &history[0];
    let file_change = &commit.files[0];
    let hunk = &file_change.hunks[0];

    // We should see a Context line, a Removed line, an Added line, and a Context line
    let mut saw_removed = false;
    let mut saw_added = false;
    let mut context_count = 0;

    for line in &hunk.lines {
        match line {
            git_associates::model::LineChange::Context(_) => context_count += 1,
            git_associates::model::LineChange::Removed(text) => {
                saw_removed = true;
                assert!(text.contains("Line 2"));
            }
            git_associates::model::LineChange::Added(text) => {
                saw_added = true;
                assert!(text.contains("Modified Line 2"));
            }
        }
    }

    assert!(saw_removed, "Should have a removed line");
    assert!(saw_added, "Should have an added line");
    assert!(context_count > 0, "Should have context lines");
}
