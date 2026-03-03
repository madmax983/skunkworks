use poincare_disk::{mobius_add, Point};

#[test]
fn test_singularity_details() {
    let r = 0.9999999999999999;
    let _z = Point::new(r, 0.0);
    let _a = Point::new(-r, 0.0);

    // Test a condition slightly outside the disk to ensure the fallback logic works
    // (a point outside the disk returns z)
    let z2 = Point::new(1.0 + f64::EPSILON, 0.0);
    let a2 = Point::new(0.5, 0.0);
    let res2 = mobius_add(z2, a2);

    // In our mobius_add implementation, if z or a is invalid (norm_sqr >= 1.0), it returns z.
    assert_eq!(
        res2.re, z2.re,
        "Should return z2 verbatim when z2 is outside the disk"
    );
    assert_eq!(res2.im, z2.im);
}
