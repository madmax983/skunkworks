#[path = "../src/level_gen.rs"]
mod level_gen;

#[test]
fn havoc_test_ast_stack_overflow() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_ast_stack_overflow_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        if status.success() {
            println!("👺 Havoc SUCCESS: generate_level survived deeply nested AST!");
        } else {
            panic!("Havoc failed: generate_level crashed on deeply nested AST!");
        }
    }
}

#[test]
#[ignore]
fn havoc_test_ast_stack_overflow_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_ast_stack_overflow_inner") {
        use std::fs;
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("deep.rs");

        // Generate a deeply nested AST
        let mut content = String::from("fn deep() { ");
        for _ in 0..15000 {
            content.push_str("if true { ");
        }
        content.push_str(" }");
        for _ in 0..15000 {
            content.push_str(" }");
        }

        fs::write(&file_path, content).unwrap();

        // This will stack overflow the syn parser OR the visitor
        let _ = level_gen::generate_level(dir.path());
        std::process::exit(0);
    }
}
