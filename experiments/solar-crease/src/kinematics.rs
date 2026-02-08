use macroquad::prelude::*;

pub fn calculate_miura_vertices(
    rows: usize,
    cols: usize,
    a: f32,
    b: f32,
    alpha: f32,
    fold_factor: f32, // 0.0 (folded) to 1.0 (flat)
) -> Vec<Vec3> {
    let mut vertices = Vec::with_capacity((rows + 1) * (cols + 1));

    // Map fold_factor to an angle rho (dihedral angle of the zig-zag)
    // Flat: rho = PI. Folded: rho -> 0.
    // Let's constrain rho to [0.1, PI] to avoid singularities.
    let rho = fold_factor * (std::f32::consts::PI - 0.1) + 0.1;

    // Derived parameters
    // We assume edge 'a' maintains length and angle 'alpha' is constant.
    // The projection of 'a' on the horizontal plane:
    let dx_a = a * alpha.sin() * (rho / 2.0).sin();
    let dy_a = a * alpha.cos();
    let dz_a = a * alpha.sin() * (rho / 2.0).cos();

    // Now for edge 'b'.
    // We use a simplified parameterization for 'b' that ensures the mesh doesn't tear.
    // The angle zeta of edge b with the horizontal plane.
    let sin_zeta = alpha.sin() * (rho / 2.0).cos();
    // Clamp to [-1, 1] just in case
    let zeta = sin_zeta.clamp(-1.0, 1.0).asin();

    let dy_b = b * zeta.cos();
    let dz_b = b * zeta.sin();

    // Now construct vertices.
    // Center the mesh later.

    for r in 0..=rows {
        for c in 0..=cols {
            // X position: simply accumulates dx_a
            let px = (c as f32) * dx_a;

            // Y position involves 'b' steps and the 'a' zig-zag
            // Base Y from 'b' edges:
            let mut py = (r as f32) * dy_b;

            // Add 'a' offset to Y based on column parity
            if c % 2 == 1 {
                py += dy_a;
            }

            // Z position
            // Z = (c % 2) * dz_a + (r % 2) * dz_b * (something)
            // Based on parity logic derived previously:
            // Column 0 (even): oscillates 0 -> dz_b -> 0 -> dz_b
            // Column 1 (odd):  oscillates dz_a -> dz_a - dz_b -> dz_a -> ...

            let mut pz = if c % 2 == 1 { dz_a } else { 0.0 };

            if c % 2 == 0 {
                // Valley column
                 if r % 2 == 1 { pz += dz_b; }
            } else {
                // Mountain column
                 if r % 2 == 1 { pz -= dz_b; }
            }

            vertices.push(vec3(px, py, pz));
        }
    }

    // Center the mesh
    let center_x = (cols as f32 * dx_a) / 2.0;
    let center_y = (rows as f32 * dy_b) / 2.0;
    let center_z = 0.0;

    for v in &mut vertices {
        v.x -= center_x;
        v.y -= center_y;
        v.z -= center_z;
    }

    vertices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_state() {
        // Flat state: fold_factor = 1.0 -> rho = PI
        // dz_a should be 0.
        // dz_b should be 0.
        let vertices = calculate_miura_vertices(2, 2, 1.0, 1.0, 60.0f32.to_radians(), 1.0);
        for v in vertices {
            assert!(v.z.abs() < 1e-4, "Z should be near 0 in flat state, got {}", v.z);
        }
    }
}
