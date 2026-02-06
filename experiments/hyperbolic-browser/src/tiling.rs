use num_complex::Complex;
use poincare_disk::{neighbor_transform_a, Mobius, TilingConsts};
use std::f64::consts::PI;

pub struct Tiler {
    pub consts: TilingConsts,
}

impl Tiler {
    pub fn new() -> Self {
        Self {
            consts: TilingConsts::new_4_5(),
        }
    }

    /// Returns the transformation from Child Space to Parent Space.
    /// In Child Space:
    /// - Center (0,0) is the Child.
    /// - Neighbor 2 (Left) is the Parent.
    /// In Parent Space:
    /// - Center (0,0) is the Parent.
    /// - Neighbor `direction` is the Child.
    ///
    /// Result maps Child(0,0) -> Parent(neighbor_pos).
    /// Result maps Child(neighbor_2_pos) -> Parent(0,0).
    pub fn get_neighbor_transform(&self, direction: usize) -> Mobius {
        let a = neighbor_transform_a(direction, &self.consts);

        // We want to rotate Child Space so that its Neighbor 2 (Left) aligns with the direction towards Parent.
        // In Parent Space, Child is at `direction` angle.
        // So Parent is at `direction + 180` (Backwards) relative to Child Center?
        // No, in hyperbolic space, direction is preserved along geodesic.
        // If we move East (0), we look West (2) to see origin.
        // So relative to Child, Parent is West (2).
        // My `fs_map` assumes Parent is ALWAYS West (2).
        // So Child's West should point to Parent.
        // If Child is North (1) of Parent.
        // Relative to Child, Parent is South (3).
        // But we want Parent to be West (2).
        // So we need to rotate Child Space: West (2) -> South (3).
        // Rotate -90 deg.
        // `direction` = 1. `rotation` = -1 * 90.
        // West (-1,0) -> South (0,-1). Correct.

        let rotation_angle = -(direction as f64) * PI / 2.0;

        let translation = Mobius::translation(a);

        // Rotation Mobius: z -> z * e^(i*theta)
        // Matrix: [e^(i*t/2) 0; 0 e^(-i*t/2)]
        let half_angle = rotation_angle / 2.0;
        let rotation = Mobius {
            a: Complex::from_polar(1.0, half_angle),
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::from_polar(1.0, -half_angle),
        };

        // Apply rotation first (in local frame), then translation.
        translation.then(&rotation)
    }
}
