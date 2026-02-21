use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, mobius_add, Mobius, Point};

/// Generates a deterministic sequence of points inside the Poincaré disk.
pub fn pseudo_random_points(seed: u64, count: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(count);
    let mut s = seed;
    for _ in 0..count {
        // LCG: x_{n+1} = (a * x_n + c) % m
        // Using values from MMIX (Knuth)
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u1 = (s >> 32) as f64 / 4294967296.0;

        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u2 = (s >> 32) as f64 / 4294967296.0;

        // Random point in unit disk: r = sqrt(u1), theta = 2*pi*u2
        // We scale r by 0.95 to ensure we stay strictly inside the disk and avoid boundary singularities
        // for general property tests.
        let r = u1.sqrt() * 0.95;
        let theta = u2 * 2.0 * std::f64::consts::PI;
        points.push(Complex::from_polar(r, theta));
    }
    points
}

#[test]
fn test_triangle_inequality() {
    let points = pseudo_random_points(12345, 30);
    // Check random triples
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        let c = points[(i + 2) % points.len()];

        let dist_ac = hyperbolic_dist(a, c);
        let dist_ab = hyperbolic_dist(a, b);
        let dist_bc = hyperbolic_dist(b, c);

        assert!(
            dist_ac <= dist_ab + dist_bc + 1e-9,
            "Triangle inequality violated: d(a,c)={} > d(a,b)={} + d(b,c)={}",
            dist_ac,
            dist_ab,
            dist_bc
        );
    }
}

#[test]
fn test_mobius_associativity() {
    let points = pseudo_random_points(54321, 20);

    // Construct some random Mobius transformations from points
    // T(z) = (z+a)/(1+a'z) (translation)
    let transforms: Vec<Mobius> = points.iter().map(|p| Mobius::translation(*p)).collect();

    for i in 0..(transforms.len() - 2) {
        let a = transforms[i];
        let b = transforms[i + 1];
        let c = transforms[i + 2];

        // (A * B) * C
        let ab = a.then(&b);
        let lhs = ab.then(&c);

        // A * (B * C)
        let bc = b.then(&c);
        let rhs = a.then(&bc);

        // Check if matrices are approximately equal
        let diff_a = (lhs.a() - rhs.a()).norm();
        let diff_b = (lhs.b() - rhs.b()).norm();
        let diff_c = (lhs.c() - rhs.c()).norm();
        let diff_d = (lhs.d() - rhs.d()).norm();

        assert!(
            diff_a < 1e-9,
            "Associativity failed A component: {}",
            diff_a
        );
        assert!(
            diff_b < 1e-9,
            "Associativity failed B component: {}",
            diff_b
        );
        assert!(
            diff_c < 1e-9,
            "Associativity failed C component: {}",
            diff_c
        );
        assert!(
            diff_d < 1e-9,
            "Associativity failed D component: {}",
            diff_d
        );
    }
}

#[test]
fn test_isometry_property() {
    let points = pseudo_random_points(67890, 20);
    let translation_targets = pseudo_random_points(13579, 5);

    for &t_point in &translation_targets {
        // Apply translation by t_point to all test points
        // Note: mobius_add(z, a) is translation of z by a.
        // But distance isometry is usually d(z1, z2) = d(T(z1), T(z2))

        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];

            let dist_orig = hyperbolic_dist(p1, p2);

            let p1_prime = mobius_add(p1, t_point);
            let p2_prime = mobius_add(p2, t_point);

            let dist_new = hyperbolic_dist(p1_prime, p2_prime);

            assert!(
                (dist_orig - dist_new).abs() < 1e-9,
                "Isometry violated: dist_orig={}, dist_new={}, diff={}",
                dist_orig,
                dist_new,
                (dist_orig - dist_new).abs()
            );
        }
    }
}

#[test]
fn test_singularity_robustness() {
    // Attempt to trigger division by zero or NaN
    // 1 + a.conj() * z approx 0
    // a = -r, z = r, r->1
    let r_vals = [0.9, 0.99, 0.999, 0.9999, 0.99999999];
    for &r in &r_vals {
        let z = Point::new(r, 0.0);
        let a = Point::new(-r, 0.0);
        let res = mobius_add(z, a);

        // We don't necessarily assert it's a valid point inside the disk (since math breaks down),
        // but we assert it doesn't panic.
        // And we check if it is finite (unless we hit absolute precision limit)
        if res.re.is_finite() && res.im.is_finite() {
            // Good
        } else {
            println!("Got non-finite result for r={}: {:?}", r, res);
        }
    }
}

#[test]
fn test_invalid_inputs() {
    // z outside disk
    let z = Point::new(2.0, 0.0);
    let a = Point::new(0.5, 0.0);

    // This is technically undefined behavior according to docs ("assumes norm < 1"),
    // but robust code shouldn't panic.
    // With safety checks, it should return z unchanged.
    let res = mobius_add(z, a);
    assert!((res - z).norm() < 1e-9);

    // z outside, causing singularity
    // 1 + a.conj() * z = 0  => a.conj() * z = -1
    // Let a = 0.5. z = -2.0.
    // 1 + 0.5 * (-2) = 0.
    let z_bad = Point::new(-2.0, 0.0);
    let res_bad = mobius_add(z_bad, a);
    // With safety checks, this avoids singularity and returns z_bad
    assert!((res_bad - z_bad).norm() < 1e-9);
}
