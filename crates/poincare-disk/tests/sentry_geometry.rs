use poincare_disk::{Geodesic, Point};

#[test]
fn test_geodesic_euclidean_circle() {
    let p1 = Point::new(0.5, 0.0);
    let p2 = Point::new(0.0, 0.5);
    let geo = Geodesic::new(p1, p2);

    let circle = geo.euclidean_circle();
    assert!(circle.is_some());
    let (center, _radius) = circle.unwrap();

    assert!((center.re - 1.25).abs() < 1e-9);
    assert!((center.im - 1.25).abs() < 1e-9);
}

#[test]
fn test_geodesic_collinear() {
    let p1 = Point::new(0.2, 0.2);
    let p2 = Point::new(0.4, 0.4);
    let geo = Geodesic::new(p1, p2);

    let circle = geo.euclidean_circle();
    assert!(circle.is_none());
}
