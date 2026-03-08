use git2::{Repository, Signature, Time};
use git_associates::GitModel;
use std::fs;
use std::path::Path;

#[test]
fn test_diff_workdir() {
    let temp_dir = std::env::temp_dir().join("git-associates-diff-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    // Create an initial file
    let file_path = temp_dir.join("test.txt");
    fs::write(&file_path, "initial content\n").unwrap();
    let delete_file_path = temp_dir.join("delete.md");
    fs::write(&delete_file_path, "delete me\n").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    index.add_path(Path::new("delete.md")).unwrap();
    index.write().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let time = Time::new(1700000000, 0);
    let sig = Signature::new("Test Author", "test@example.com", &time).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .unwrap();

    // Modify the file
    fs::write(&file_path, "modified content\nadded line\n").unwrap();

    // Add a new file
    let new_file_path = temp_dir.join("new.txt");
    fs::write(&new_file_path, "new file content\n").unwrap();

    // Delete a file
    fs::remove_file(&delete_file_path).unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let diff = model.diff_workdir().unwrap();

    assert_eq!(diff.files.len(), 3);

    // find test.txt
    let test_file = diff.files.iter().find(|f| f.path == "test.txt").unwrap();
    assert_eq!(test_file.extension, "txt");
    assert_eq!(test_file.insertions, 2);
    assert_eq!(test_file.deletions, 1);
    assert_eq!(test_file.hunks.len(), 1);

    // find new.txt
    let new_file = diff.files.iter().find(|f| f.path == "new.txt").unwrap();
    assert_eq!(new_file.extension, "txt");
    assert_eq!(new_file.insertions, 0); // Git2 without include_untracked_content returns 0
    assert_eq!(new_file.deletions, 0);
    assert_eq!(new_file.hunks.len(), 0);

    // find delete.md
    let delete_file = diff.files.iter().find(|f| f.path == "delete.md").unwrap();
    assert_eq!(delete_file.extension, "md");
    assert_eq!(delete_file.insertions, 0);
    assert_eq!(delete_file.deletions, 1);
    assert_eq!(delete_file.hunks.len(), 1);

    // DiffStats total
    assert_eq!(diff.total_added, 2); // test.txt insertions + new.txt insertions
    assert_eq!(diff.total_removed, 2); // test.txt deletions + delete.md deletions
}
