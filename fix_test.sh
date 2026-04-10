cat << 'INNER_EOF' > patch_akashic.diff
<<<<<<< SEARCH
    #[test]
    fn test_akashic_storage() {
        // Cleanup
        let _ = fs::remove_file(".chimera_akashic.json");

        // 1. Write "foo" -> 42
=======
    #[test]
    fn test_akashic_storage() {
        // 1. Write "foo" -> 42
>>>>>>> REPLACE
<<<<<<< SEARCH
        assert_eq!(vm_read.stack.pop(), Some(Value::Int(42)));
        let _ = fs::remove_file(&vm_write.akashic.file_path);
    }

    #[test]
    fn test_karma_miracle() {
        // Cleanup
        let _ = fs::remove_file(".chimera_akashic.json");

        // 1. Gain Karma
=======
        assert_eq!(vm_read.stack.pop(), Some(Value::Int(42)));
        let _ = fs::remove_file(&vm_write.akashic.file_path);
    }

    #[test]
    fn test_karma_miracle() {
        // 1. Gain Karma
>>>>>>> REPLACE
INNER_EOF
patch experiments/chimera-lang/tests/akashic_test.rs patch_akashic.diff
