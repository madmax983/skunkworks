use klein_fs::fs_scanner;

#[test]
fn test_scanner() {
    let nodes = fs_scanner::scan_directory(".", 2);
    assert!(!nodes.is_empty());
    assert_eq!(nodes[0].path, ".");

    // Check if Cargo.toml is found (assuming we run from crate root)
    let found = nodes.iter().any(|n| n.name == "Cargo.toml");
    // Wait, if running from repo root, "." is repo root. Cargo.toml is there.
    // If running from experiments/klein-fs, Cargo.toml is there.
    // Yes.
    assert!(found, "Cargo.toml not found in scan");
}
