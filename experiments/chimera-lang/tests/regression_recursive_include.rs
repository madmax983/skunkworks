#[cfg(test)]
mod tests {
    use chimera_lang::compiler::compile;
    use std::fs;

    #[test]
    fn test_recursive_include_cycle() {
        // Use a temporary directory for the test to avoid path issues
        let temp_dir = std::env::temp_dir().join("chimera_recursion_test");
        let _ = fs::remove_dir_all(&temp_dir); // Ensure clean state
        fs::create_dir_all(&temp_dir).unwrap();

        let a_path = temp_dir.join("a.chs");
        let b_path = temp_dir.join("b.chs");

        // a.chs includes b.chs
        fs::write(&a_path, "include \"b.chs\"").unwrap();
        // b.chs includes a.chs
        fs::write(&b_path, "include \"a.chs\"").unwrap();

        // The source we compile just includes a.chs
        let src = "include \"a.chs\"";

        // Pass the temp_dir as the base path so imports resolve relative to it
        let result = compile(src, Some(&temp_dir));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);

        assert!(result.is_err(), "Recursive include should fail");
        let err = result.unwrap_err();
        println!("Error was: {}", err);
        assert!(err.to_string().contains("Recursive include detected"));
    }
}
