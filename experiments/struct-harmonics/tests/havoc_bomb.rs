use struct_harmonics::parser::scan_workspace;
use std::fs;

#[test]
fn havoc_bomb_test() {
    let mut bomb_code = String::from("struct Bomb {\n    a: ");
    for _ in 0..5000 {
        bomb_code.push_str("Vec<");
    }
    bomb_code.push_str("String");
    for _ in 0..5000 {
        bomb_code.push_str(">");
    }
    bomb_code.push_str(",\n}");

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("bomb4.rs");
    fs::write(&temp_file, bomb_code).unwrap();
    let _ = scan_workspace(temp_dir.path());
}