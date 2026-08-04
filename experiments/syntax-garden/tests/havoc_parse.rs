#[path = "../src/parser.rs"]
pub(crate) mod parser;

use std::env;
use std::process::Command;

#[test]
fn havoc_test_parse_bomb() {
    let status = Command::new(env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_parse_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    if status.success() {
        println!("👺 Havoc SUCCESS: parser survived deeply nested AST!");
    } else {
        panic!(
            "Havoc failed: parser crashed on deeply nested AST! Exit code: {:?}",
            status.code()
        );
    }
}

#[test]
#[ignore]
fn havoc_test_parse_inner() {
    if env::args().any(|arg| arg == "havoc_test_parse_inner") {
        let temp_dir = env::temp_dir().join("havoc_syntax_garden");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let file_path = temp_dir.join("dos.rs");

        let mut content = String::new();
        for _ in 0..20000 {
            content.push('{');
        }
        for _ in 0..20000 {
            content.push('}');
        }
        std::fs::write(&file_path, content).unwrap();

        let _ = parser::parse_directory(temp_dir.to_str().unwrap());
    }
}
