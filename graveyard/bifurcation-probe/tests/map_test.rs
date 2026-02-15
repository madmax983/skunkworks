use bifurcation_probe::map::{ChaoticMap, LogisticMap};

#[test]
fn test_logistic_iteration() {
    let map = LogisticMap::new(3.0);
    // x_{n+1} = r * x_n * (1 - x_n)
    // 3.0 * 0.5 * (1 - 0.5) = 3.0 * 0.5 * 0.5 = 0.75
    let result = map.iterate(0.5);
    assert!((result - 0.75).abs() < 1e-6);
}

#[test]
fn test_lyapunov_stable() {
    // r = 2.0. Stable fixed point at x = 0.5.
    // f'(x) = r(1-2x). At x=0.5, f'(x) = 2(0) = 0. ln(0) is -inf.
    // Let's pick r=2.5. Fixed point x = 1 - 1/r = 1 - 0.4 = 0.6.
    // f'(0.6) = 2.5 * (1 - 1.2) = 2.5 * -0.2 = -0.5.
    // Lyapunov = ln(|-0.5|) = ln(0.5) ≈ -0.693.
    let map = LogisticMap::new(2.5);
    // Run for some steps to reach attractor
    let mut x = 0.1;
    for _ in 0..100 {
        x = map.iterate(x);
    }

    let lambda = bifurcation_probe::map::calculate_lyapunov(&map, x, 100);
    assert!((lambda - (-0.693)).abs() < 0.1);
}

#[test]
fn test_lyapunov_chaotic() {
    // r = 4.0. Fully chaotic. Lyapunov = ln(2) ≈ 0.693.
    let map = LogisticMap::new(4.0);
    let mut x = 0.1;
    // Transient
    for _ in 0..100 {
        x = map.iterate(x);
    }
    let lambda = bifurcation_probe::map::calculate_lyapunov(&map, x, 1000);
    assert!((lambda - 0.693).abs() < 0.1);
}
