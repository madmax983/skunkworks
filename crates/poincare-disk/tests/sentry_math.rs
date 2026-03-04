use poincare_disk::{mobius_add, mobius_sub, Point};

#[test]
fn test_mobius_add_invalid_a() {
    let z = Point::new(0.0, 0.0);
    let a_invalid = Point::new(1.0, 0.0); // Modulus >= 1.0 is invalid
    let result = mobius_add(z, a_invalid);

    assert_eq!(result, z);
}

#[test]
fn test_mobius_sub_invalid_a() {
    let z = Point::new(0.0, 0.0);
    let a_invalid = Point::new(1.0, 0.0); // Modulus >= 1.0 is invalid
    let result = mobius_sub(z, a_invalid);

    assert_eq!(result, z);
}
