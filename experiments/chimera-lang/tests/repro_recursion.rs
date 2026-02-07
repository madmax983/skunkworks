use chimera_lang::compiler::compile;
use std::fs;

#[test]
fn test_recursive_include_crash() {
    let temp_dir_path = std::env::temp_dir().join("chimera_havoc_repro");
    if temp_dir_path.exists() {
        fs::remove_dir_all(&temp_dir_path).unwrap();
    }
    fs::create_dir(&temp_dir_path).unwrap();

    let file_a = temp_dir_path.join("a.chs");
    let file_b = temp_dir_path.join("b.chs");

    fs::write(&file_a, "include \"b.chs\"\n").unwrap();
    fs::write(&file_b, "include \"a.chs\"\n").unwrap();

    println!("Created recursive include files at {:?}", temp_dir_path);

    // This should crash with a stack overflow
    let result = compile("include \"a.chs\"", Some(&temp_dir_path));

    // Cleanup (unlikely to be reached if it crashes)
    let _ = fs::remove_dir_all(&temp_dir_path);

    assert!(result.is_err(), "Expected compilation to fail, but it succeeded!");
}
