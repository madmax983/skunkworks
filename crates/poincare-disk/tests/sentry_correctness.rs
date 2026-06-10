use poincare_disk::{Mobius, Point};
use num_complex::Complex;

#[test]
fn test_geodesic_euclidean_circle() {
    let p1 = Point::new(0.5, 0.0);
    let p2 = Point::new(0.0, 0.5);
    let geo = poincare_disk::Geodesic::new(p1, p2);

    let circle = geo.euclidean_circle();
    assert!(circle.is_some());
    if let Some((center, _radius)) = circle {
        assert!((center.re - 1.25).abs() < 1e-9);
        assert!((center.im - 1.25).abs() < 1e-9);
    }
}
