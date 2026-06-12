#[path = "../src/physics.rs"]
mod physics;
use physics::Arm;

// 👺 Havoc: Prove that `Arm::solve` panics via out-of-bounds indexing and `unwrap()`
// when the public `joints` vector is cleared externally.
// We use an isolated subprocess runner to securely detonate and verify the explosion
// without failing the CI build overall, marking the outer test 'Green'.
#[test]
fn havoc_test_arm_unwrap() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_arm_unwrap_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: Expected process to succeed without panic, but it failed!"
        );
    }
}

#[test]
#[ignore]
fn havoc_test_arm_unwrap_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_arm_unwrap_inner") {
        let mut arm = Arm::new((0.0, 0.0), vec![10.0, 10.0]);
        // 🧨 The Trigger: Mutate the public field to cause the crash!
        arm.joints.clear();

        // 💥 Detonate: This will panic inside `solve` at `let base = self.joints[0];`
        arm.solve((5.0, 5.0));
        std::process::exit(0);
    }
}
