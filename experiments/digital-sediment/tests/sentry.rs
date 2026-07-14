#[path = "../src/git.rs"]
#[allow(dead_code)]
mod git;

use git2::{Repository, Signature};
use std::env;
use std::fs;

// 🛡️ Sentry: Prove that `unwrap_or(Some("")).unwrap()` panic is fixed.
#[test]
fn test_commit_summary_unwrap() {
    let temp_dir = env::temp_dir().join("sentry_git_test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let repo = Repository::init(&temp_dir).unwrap();
    let sig = Signature::now("Test", "test@test.com").unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    // Create an empty commit message, git2 might return None for summary() or ""
    let _commit_id = repo
        .commit(Some("HEAD"), &sig, &sig, "", &tree, &[])
        .unwrap();

    // Test that list_commits works without panicking!
    let list = git::list_commits(&repo, 10).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].message, "");
}
