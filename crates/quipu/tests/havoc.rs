use proptest::prelude::*;
use quipu::Cord;

proptest! {
    #[test]
    fn test_quipu_checked_subtraction(a in any::<u64>(), b in any::<u64>()) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);

        let result = c1.checked_sub(&c2);

        if a < b {
            assert!(result.is_none(), "Expected underflow to return None");
        } else {
            assert_eq!(result.unwrap().value(), a - b);
        }
    }

    /// 👺 Havoc: Proving `Cord::add` overflow is now handled safely.
    ///
    /// The unsafe `Add` and `Sub` trait implementations have been removed,
    /// so this test now confirms that `checked_add` correctly returns None
    /// instead of panicking or wrapping on overflow.
    #[test]
    fn test_quipu_checked_addition_overflow(
        a in (u64::MAX / 2 + 1)..u64::MAX,
        b in (u64::MAX / 2 + 1)..u64::MAX
    ) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);

        let result = c1.checked_add(&c2);
        assert!(result.is_none(), "Expected overflow to return None");
    }
}

// 👺 Havoc: Prove that checking Cord equality might fail unexpectedly or panic under some
// extremely large cord sizes due to the Vec equality.
// A real system might run out of memory or stack overflow.
// We use a custom runner thread to isolate the overflow so the whole test suite doesn't abort.
#[test]
fn havoc_quipu_huge_cord_equality() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_quipu_huge_cord_equality_inner")
        .arg("--nocapture")
        .status();

    if let Ok(status) = status {
        // If it exited with signal (e.g. SIGABRT from stack overflow), we proved fragility!
        assert!(status.success(), "👺 Havoc: System did not safely handle 100k depth PartialEq! Our chaos hunt proved successful, but now Sentry fixed it!");
    }
}

#[test]
fn havoc_quipu_huge_cord_equality_inner() {
    // Only run this test if explicitly requested, as it is designed to abort the process.
    if std::env::args().any(|arg| arg == "havoc_quipu_huge_cord_equality_inner") {
        let mut cord1 = Cord::from(10);
        let mut cord2 = Cord::from(10);

        // Create an absurdly deep nested cord structure to blow the stack on PartialEq
        for _ in 0..100000 {
            let mut child = Cord::from(1);
            child.subsidiaries.push(cord1);
            cord1 = child;

            let mut child2 = Cord::from(1);
            child2.subsidiaries.push(cord2);
            cord2 = child2;
        }

        // This will stack overflow if PartialEq is not iterative, or take forever.
        assert_eq!(cord1, cord2);

        // Return 0 if successful
        std::process::exit(0);
    }
}
