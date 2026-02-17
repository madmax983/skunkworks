use macroquad::prelude::Vec3;

/// Performs Möbius addition: u (+) v
/// This represents the translation of point u by vector v in hyperbolic space.
/// Formula: ( (1 + 2<u,v> + |v|^2)u + (1 - |u|^2)v ) / (1 + 2<u,v> + |u|^2|v|^2)
pub fn mobius_add(u: Vec3, v: Vec3) -> Vec3 {
    let u2 = u.length_squared();
    let v2 = v.length_squared();
    let uv = u.dot(v);

    let num = (1.0 + 2.0 * uv + v2) * u + (1.0 - u2) * v;
    let den = 1.0 + 2.0 * uv + u2 * v2;

    num / den
}

/// Performs Möbius subtraction: u (-) v
/// This is equivalent to u (+) (-v).
#[allow(dead_code)]
pub fn mobius_sub(u: Vec3, v: Vec3) -> Vec3 {
    mobius_add(u, -v)
}

/// Calculates the hyperbolic distance between two points u and v in the Poincaré ball.
/// Formula: acosh(1 + 2|u-v|^2 / ((1-|u|^2)(1-|v|^2)))
pub fn hyperbolic_dist(u: Vec3, v: Vec3) -> f32 {
    let dist_sq = (u - v).length_squared();
    let u2 = u.length_squared();
    let v2 = v.length_squared();

    let num = 2.0 * dist_sq;
    let den = (1.0 - u2) * (1.0 - v2);

    let arg = 1.0 + num / den;
    arg.acosh()
}

/// Computes the scaling factor for a sphere at position p with hyperbolic radius r_hyp.
/// The Euclidean radius r_euc is approximately r_hyp * (1 - |p|^2) / 2?
/// Actually, the conformal factor is 2 / (1 - |p|^2).
/// So an object of size L at p appears to have size L * (1 - |p|^2) / 2 in Euclidean space?
/// Let's use the formula: Euclidean Metric = Hyperbolic Metric * (1 - r^2) / 2.
/// So Euclidean Length = Hyperbolic Length * (1 - r^2) / 2.
#[allow(dead_code)]
pub fn conformal_scale(p: Vec3) -> f32 {
    let r2 = p.length_squared();
    (1.0 - r2).max(0.0001) // Clamp to avoid zero/negative
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_identity() {
        let u = Vec3::new(0.5, 0.0, 0.0);
        let zero = Vec3::ZERO;

        assert_eq!(mobius_add(u, zero), u);
        assert_eq!(mobius_add(zero, u), u);
    }

    #[test]
    fn test_mobius_inverse() {
        let u = Vec3::new(0.3, 0.4, 0.0);
        // u (+) (-u) should be 0
        let res = mobius_add(u, -u);
        assert!(res.length() < 1e-6);
    }

    #[test]
    fn test_mobius_associativity_check() {
        // Möbius addition is NOT associative generally.
        // We check if (a + b) + c != a + (b + c).
        let a = Vec3::new(0.8, 0.0, 0.0);
        let b = Vec3::new(0.0, 0.8, 0.0);
        let c = Vec3::new(0.0, 0.0, 0.8);

        let lhs = mobius_add(mobius_add(a, b), c);
        let rhs = mobius_add(a, mobius_add(b, c));

        println!("LHS: {:?}", lhs);
        println!("RHS: {:?}", rhs);
        println!("Diff: {:?}", (lhs - rhs).length());

        // Assert they are different enough
        assert!(
            (lhs - rhs).length() > 1e-8,
            "Möbius addition appeared associative! Diff: {}",
            (lhs - rhs).length()
        );
    }

    #[test]
    fn test_dist_symmetry() {
        let a = Vec3::new(0.2, -0.3, 0.1);
        let b = Vec3::new(-0.1, 0.5, 0.0);

        let d1 = hyperbolic_dist(a, b);
        let d2 = hyperbolic_dist(b, a);

        assert!((d1 - d2).abs() < 1e-6);
    }

    #[test]
    fn test_dist_zero() {
        let a = Vec3::new(0.5, 0.5, 0.5); // Norm sqrt(0.75) < 1.0
        assert!(hyperbolic_dist(a, a).abs() < 1e-6);
    }
}
