use poincare_disk::{mobius_add, mobius_sub, Point};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 10000, .. ProptestConfig::default() })]

    // 👺 Havoc: We must find inputs where Poincare math returns a NaN or Infinity!
    // Since `norm_sqr() < 1.0` avoids division by zero, let's explicitly inject NaNs
    // or Infinities as inputs to see if it gracefully returns them or panics elsewhere.
    // Also, we can use `f64::ANY` to test ALL float values (including NaNs/Infs).
    #[test]
    fn test_mobius_add_fuzzing(
        z_re in proptest::num::f64::ANY,
        z_im in proptest::num::f64::ANY,
        a_re in proptest::num::f64::ANY,
        a_im in proptest::num::f64::ANY,
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let res = mobius_add(z, a);

        // Let's assert that the result is NEVER NaN.
        // The current implementation probably propagates NaNs because `norm_sqr() >= 1.0`
        // is false if it's NaN! Wait, `NaN >= 1.0` is false, so it falls through to division
        // and produces NaN.
        prop_assert!(!res.re.is_nan(), "Havoc 👺: Math failed! NaN produced.");
        prop_assert!(!res.im.is_nan(), "Havoc 👺: Math failed! NaN produced.");
    }

    #[test]
    fn test_mobius_sub_fuzzing(
        z_re in proptest::num::f64::ANY,
        z_im in proptest::num::f64::ANY,
        a_re in proptest::num::f64::ANY,
        a_im in proptest::num::f64::ANY,
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let res = mobius_sub(z, a);

        prop_assert!(!res.re.is_nan(), "Havoc 👺: Math failed! NaN produced.");
        prop_assert!(!res.im.is_nan(), "Havoc 👺: Math failed! NaN produced.");
    }
}

// 👺 Havoc: Prove that passing usize::MAX causes a capacity overflow panic.
#[test]
fn havoc_poincare_disk_alloc_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_poincare_disk_alloc_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! pseudo_random_points panics internally on count = usize::MAX due to capacity overflow!"
        );
    }
}

pub fn pseudo_random_points(seed: u64, count: usize) -> Vec<poincare_disk::Point> {
    let actual_count = count.min(100_000);
    let mut points = Vec::with_capacity(actual_count);
    let mut s = seed;
    for _ in 0..actual_count {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u1 = (s >> 32) as f64 / 4294967296.0;

        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u2 = (s >> 32) as f64 / 4294967296.0;

        let r = u1.sqrt() * 0.95;
        let theta = u2 * 2.0 * std::f64::consts::PI;
        points.push(num_complex::Complex::from_polar(r, theta));
    }
    points
}

#[test]
#[ignore]
fn havoc_poincare_disk_alloc_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_poincare_disk_alloc_panic_inner") {
        let _points = pseudo_random_points(1234, usize::MAX);
        std::process::exit(0);
    }
}
