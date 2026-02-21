use crate::types::Point;

/// Performs Möbius addition: $ (z + a) / (1 + \bar{a}z) $.
///
/// This operation represents a hyperbolic translation that moves the origin to `a`.
/// It is non-commutative and non-associative in the Euclidean sense, but forms a group loop (gyroup).
///
/// Geometrically, this maps the origin $0$ to $a$.
///
/// # Panics
///
/// Does not panic, but returns `z` unchanged if `|a| >= 1` (invalid translation)
/// or if `|z| >= 1` (invalid point).
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, mobius_add};
///
/// let z = Point::new(0.0, 0.0); // Origin
/// let a = Point::new(0.5, 0.0); // Target
///
/// // Translating the origin by 'a' moves it to 'a'
/// let result = mobius_add(z, a);
/// assert_eq!(result, a);
/// ```
#[doc(alias = "hyperbolic_translation")]
pub fn mobius_add(z: Point, a: Point) -> Point {
    // If 'a' is outside the disk, this is an invalid translation.
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    // If 'z' is outside the disk, the operation is undefined and could cause division by zero.
    // We return 'z' to propagate the error safely.
    if z.norm_sqr() >= 1.0 {
        return z;
    }
    (z + a) / (1.0 + a.conj() * z)
}

/// Performs Möbius subtraction: $ (z - a) / (1 - \bar{a}z) $.
///
/// This is the inverse of `mobius_add`. It represents a hyperbolic translation that moves `a` to the origin.
///
/// Geometrically, this maps $a$ to $0$.
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, mobius_sub};
///
/// let a = Point::new(0.5, 0.5);
/// // Moving 'a' "back" by 'a' should result in the origin
/// let result = mobius_sub(a, a);
/// assert!(result.norm() < 1e-10);
/// ```
#[doc(alias = "hyperbolic_inverse_translation")]
pub fn mobius_sub(z: Point, a: Point) -> Point {
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    if z.norm_sqr() >= 1.0 {
        return z;
    }
    (z - a) / (1.0 - a.conj() * z)
}

/// Calculates the hyperbolic distance between two points.
///
/// The distance formula is:
/// $$ d(a, b) = 2 \text{tanh}^{-1} \left( \left| \frac{a - b}{1 - \bar{a}b} \right| \right) $$
///
/// As points approach the boundary of the disk (norm -> 1), the distance approaches infinity.
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, hyperbolic_dist};
///
/// let center = Point::new(0.0, 0.0);
/// let p = Point::new(0.5, 0.0);
///
/// // Distance from 0 to 0.5 is 2 * atanh(0.5) ≈ 1.0986
/// let dist = hyperbolic_dist(center, p);
/// assert!((dist - 1.0986).abs() < 0.001);
/// ```
pub fn hyperbolic_dist(a: Point, b: Point) -> f64 {
    let num = a - b;
    let den = 1.0 - a.conj() * b;
    let modulus = (num / den).norm();

    // Propagate NaN to avoid masking errors
    if modulus.is_nan() {
        return f64::NAN;
    }

    // Clamp to avoid atanh(1.0) = infinity/NaN if floating point errors push us over
    let clamped = modulus.min(0.99999999);
    2.0 * clamped.atanh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_inverses() {
        let a = Point::new(0.3, -0.2);
        let z = Point::new(0.1, 0.5);
        let t = mobius_add(z, a);
        let back = mobius_sub(t, a);
        assert!((back - z).norm() < 1e-10);
    }

    #[test]
    fn test_mobius_origin() {
        let a = Point::new(0.5, 0.0);
        // Transforming 'a' by 'a' (via sub) should map it to the origin.
        let result = mobius_sub(a, a);
        assert!(result.norm() < 1e-9);
    }

    #[test]
    fn test_hyperbolic_dist_zero() {
        let a = Point::new(0.2, 0.2);
        assert!(hyperbolic_dist(a, a) < 1e-9);
    }

    #[test]
    fn test_disk_boundary_preservation() {
        // Points inside the disk should stay inside
        let a = Point::new(0.5, 0.0);
        let z = Point::new(0.0, 0.9);
        let t = mobius_sub(z, a);
        assert!(t.norm() < 1.0);
    }

    #[test]
    fn test_havoc_singularity() {
        // 👺 HAVOC: Attempting to create a singularity at the edge of the disk.
        // We want 1 + a.conj() * z to be 0.
        // Let a = -r, z = r.
        // 1 + (-r)(r) = 1 - r^2.
        // If r -> 1, 1 - r^2 -> 0.
        // Result = (r - (-r)) / (1 - r^2) = 2r / (1 - r^2) -> Infinity.

        let r = 0.9999999999999999; // Very close to 1.0
        let z = Point::new(r, 0.0);
        let a = Point::new(-r, 0.0); // -r is a valid point inside disk (barely)

        let res = mobius_add(z, a);
        println!("Havoc Result: {:?}", res);

        // This SHOULD fail if we are properly paranoid, but the current impl doesn't check for Inf/NaN output.
        // We assert that it IS a valid point (norm < 1.0), which it mathematically isn't (it escapes to infinity).
        // If the library claims to implement the Poincaré disk, the result MUST be in the disk.
        assert!(
            res.norm() < 1.0,
            "Singularity breach! Point escaped the disk: {:?}",
            res
        );
    }
}
