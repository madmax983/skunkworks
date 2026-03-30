use digital_sediment::git;

#[test]
#[should_panic(expected = "Failed to open repository")]
fn test_havoc_git_panic() {
    let _ = git::open_repo("/path/to/nowhere/that/does/not/exist").unwrap();
}

#[test]
#[should_panic]
fn test_havoc_unwrap_panic() {
    let temp_dir = tempfile::tempdir().unwrap();
    git2::Repository::init(temp_dir.path()).unwrap();

    // 👺 Havoc: Using an empty repository without checking length causes
    // unwrap to panic, mirroring main.rs behavior before commits are present
    let repo = git::open_repo(temp_dir.path()).unwrap();
    let commits = git::list_commits(&repo, 50).unwrap();
    let _ = commits[0].id;
}

#[test]
#[should_panic(expected = "unable to parse OID")]
fn test_havoc_git_invalid_oid_panic() {
    let temp_dir = tempfile::tempdir().unwrap();
    git2::Repository::init(temp_dir.path()).unwrap();

    // 👺 Havoc: Proving that passing an invalid OID string to `git::get_file_content`
    // combined with `.unwrap()` in the calling code causes a panic.
    // The `Oid::from_str` call will fail for strings that are not valid hex.
    // This replicates the vulnerability where a bad commit ID crashes the application.
    let repo = git::open_repo(temp_dir.path()).unwrap();
    let _ = git::get_file_content(&repo, "NotAValidOid", "README.md").unwrap();
}

use digital_sediment::recovery::RecoveryEngine;
use proptest::prelude::*;

proptest! {
    /// 👺 Havoc: Proving that `RecoveryEngine::recover` is fragile and loops infinitely
    /// or uses excessive memory when given specific inputs, due to the `nom` many0
    /// combinator being used with a parser that might succeed but consume 0 bytes
    /// (the fallback parses anychar, but what if input is empty or has weird unicode?).
    #[test]
    #[ignore = "Might burn CPU quota, uncomment for true chaos"]
    fn test_havoc_recovery_engine(s in "\\PC*") {
        let _ = RecoveryEngine::recover(&s);
    }
}
