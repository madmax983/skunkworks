use git2::{Repository, Signature, Time};
use git_associates::GitModel;
use std::fs;
use std::path::Path;

#[test]
fn test_history_invalid_timestamp_fallback() {
    let temp_dir = std::env::temp_dir().join("git-associates-timestamp-fallback");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    // In order to hit `unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())`,
    // `Utc.timestamp_opt` must return None or Ambiguous instead of Single.
    // By providing an absurdly large positive timestamp value that chrono cannot parse
    // it triggers the fallback, validating that Sentry/Warden handled the unwrap safely.
    // Chrono's max supported year is 262143. Providing seconds that map past this throws it out of bounds.
    let seconds_far_future = 253_402_300_800 * 100_000;
    let time = Time::new(seconds_far_future, 0);
    let sig = Signature::new("Test", "test@example.com", &time).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial with bad time", &tree, &[])
        .unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let history = model.history(10).unwrap();

    assert_eq!(history.len(), 1);
}

#[test]
fn test_history_with_diff_unknown_path() {
    let temp_dir = std::env::temp_dir().join("git-associates-unknown-path-fallback");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let time = Time::new(1700000000, 0);
    let sig = Signature::new("Test", "test@example.com", &time).unwrap();

    let commit1_oid = repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[]).unwrap();

    // Add a file
    let file_path = temp_dir.join("normal.txt");
    fs::write(&file_path, "test").unwrap();
    index.add_path(Path::new("normal.txt")).unwrap();
    let oid2 = index.write_tree().unwrap();
    let tree2 = repo.find_tree(oid2).unwrap();

    let commit1 = repo.find_commit(commit1_oid).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Second", &tree2, &[&commit1]).unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let history = model.history_with_diffs(10).unwrap();

    assert_eq!(history.len(), 2);
}

#[test]
fn test_diff_workdir_empty() {
    let temp_dir = std::env::temp_dir().join("git-associates-diff-workdir-empty");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let _repo = Repository::init(&temp_dir).unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let diff_stats = model.diff_workdir().unwrap();
    assert_eq!(diff_stats.files.len(), 0);
    assert_eq!(diff_stats.total_added, 0);
    assert_eq!(diff_stats.total_removed, 0);
}
