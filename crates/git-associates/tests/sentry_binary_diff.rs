use git2::{Repository, Signature};
use git_associates::GitModel;
use std::fs;
use std::path::Path;

#[test]
fn test_binary_file_and_eof_newline_diffs() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir().join("git-associates-binary-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)?;
    let repo = Repository::init(&temp_dir)?;

    // 1. Create a binary file
    let bin_path = temp_dir.join("data.bin");
    fs::write(&bin_path, [0u8, 1, 2, 3, 0, 4, 5])?;

    // 2. Create a text file with NO newline at end of file
    let no_nl_path = temp_dir.join("no_newline.txt");
    fs::write(&no_nl_path, "line 1\nline 2")?;

    let mut index = repo.index()?;
    index.add_path(Path::new("data.bin"))?;
    index.add_path(Path::new("no_newline.txt"))?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let sig = Signature::now("Test", "test@example.com")?;

    let parent_commit = repo
        .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    // Modify the binary file
    fs::write(&bin_path, [0u8, 1, 2, 3, 0, 4, 5, 6, 7])?;

    // Modify the text file, still no newline
    fs::write(&no_nl_path, "line 1\nline 2 changed")?;

    index.add_path(Path::new("data.bin"))?;
    index.add_path(Path::new("no_newline.txt"))?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let parent = repo.find_commit(parent_commit)?;

    repo.commit(Some("HEAD"), &sig, &sig, "Second commit", &tree, &[&parent])?;

    let model = GitModel::open(temp_dir)?;
    let history = model.history_with_diffs(1)?;

    let commit = &history[0];
    assert_eq!(commit.files.len(), 2);

    let bin_file = commit.files.iter().find(|f| f.path == "data.bin").unwrap();
    assert!(bin_file.is_binary);
    // Binary files usually don't have detailed line hunks extracted
    assert!(bin_file.hunks.is_empty());

    let txt_file = commit.files.iter().find(|f| f.path == "no_newline.txt").unwrap();
    assert!(!txt_file.is_binary);
    assert!(!txt_file.hunks.is_empty());

    // We expect the hunks to hit the `_ => {}` branch due to the `\ No newline at end of file`
    // which git2 represents with origin `>` or `<` or `\`.

    Ok(())
}
