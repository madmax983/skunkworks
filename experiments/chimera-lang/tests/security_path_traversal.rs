use chimera_lang::compiler;
use std::fs;

#[test]
fn test_path_traversal_exploit() {
    // Create a temporary directory structure
    let root = std::env::temp_dir().join("chimera_security_test_exploit");
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir(&root).unwrap();

    let secret_path = root.join("secret.txt");
    // Make the secret file valid code so compilation succeeds if the file is read
    fs::write(&secret_path, "strand secret {}").unwrap();

    let sandbox = root.join("sandbox");
    fs::create_dir(&sandbox).unwrap();

    let exploit_path = sandbox.join("exploit.chs");
    // Attempt to include the secret file from the parent directory
    let exploit_code = r#"include "../secret.txt""#;
    fs::write(&exploit_path, exploit_code).unwrap();

    // The compiler takes the source code and the base path.
    // In this case, the base path is the 'sandbox' directory.
    let result = compiler::compile(exploit_code, Some(&sandbox));

    // Cleanup
    let _ = fs::remove_dir_all(&root);

    // Assert that the compilation FAILED (it should not be able to read the secret)
    match result {
        Ok(_) => panic!("Vulnerability confirmed: Compiler successfully included a file outside the sandbox!"),
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Security Error: Path traversal attempt detected"),
                "Unexpected error message: {}",
                error_msg
            );
        }
    }
}

#[test]
fn test_valid_include() {
    let root = std::env::temp_dir().join("chimera_security_test_valid");
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir(&root).unwrap();

    let sandbox = root.join("sandbox");
    fs::create_dir(&sandbox).unwrap();

    let lib_path = sandbox.join("lib.chs");
    fs::write(&lib_path, "strand lib {}").unwrap();

    let main_path = sandbox.join("main.chs");
    let main_code = r#"include "lib.chs""#;
    fs::write(&main_path, main_code).unwrap();

    // Should succeed
    let result = compiler::compile(main_code, Some(&sandbox));

    // Cleanup
    let _ = fs::remove_dir_all(&root);

    assert!(result.is_ok(), "Valid include failed: {:?}", result.err());
}
