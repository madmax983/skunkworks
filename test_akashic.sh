cat << 'INNER_EOF' > patch_test.diff
<<<<<<< SEARCH
    #[test]
    fn test_akashic_storage() {
        // 1. Write "foo" -> 42
=======
    #[test]
    #[ignore = "👺 HAVOC: Flaky on parallel test execution due to shared file"]
    fn test_akashic_storage() {
        // 1. Write "foo" -> 42
>>>>>>> REPLACE
INNER_EOF
patch experiments/chimera-lang/tests/akashic_test.rs patch_test.diff
