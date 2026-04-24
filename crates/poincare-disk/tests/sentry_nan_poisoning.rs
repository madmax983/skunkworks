use num_complex::Complex;
use poincare_disk::Mobius;

#[test]
fn test_mobius_nan_poisoning() {
    let a = Complex::new(f64::NAN, 0.0);
    let b = Complex::new(0.0, 0.0);
    let c = Complex::new(0.0, 0.0);
    let d = Complex::new(1.0, 0.0);

    let m_opt = Mobius::new(a, b, c, d);

    // If a is NaN, the determinant is NaN. NaN < 1e-12 is false.
    // So the constructor might incorrectly return Some instead of None!
    assert!(
        m_opt.is_none(),
        "Constructor should reject NaN determinant!"
    );
}
