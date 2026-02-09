use nalgebra::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatticeType {
    SimpleCubic,
    BodyCenteredCubic,
    FaceCenteredCubic,
}

pub struct Lattice {
    pub points: Vec<Point3<f64>>,
    pub lattice_type: LatticeType,
}

impl Lattice {
    pub fn new(l_type: LatticeType, size: usize, scale: f64) -> Self {
        let points = match l_type {
            LatticeType::SimpleCubic => generate_sc(size, scale),
            LatticeType::BodyCenteredCubic => generate_bcc(size, scale),
            LatticeType::FaceCenteredCubic => generate_fcc(size, scale),
        };
        Self {
            points,
            lattice_type: l_type,
        }
    }
}

fn generate_sc(size: usize, scale: f64) -> Vec<Point3<f64>> {
    let mut points = Vec::new();
    let range = size as isize;

    // Center at 0,0,0
    let start = -range;
    let end = range;

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                points.push(Point3::new(
                    x as f64 * scale,
                    y as f64 * scale,
                    z as f64 * scale,
                ));
            }
        }
    }
    points
}

fn generate_bcc(size: usize, scale: f64) -> Vec<Point3<f64>> {
    let mut points = generate_sc(size, scale);
    let range = size as isize;
    let start = -range;
    let end = range;

    // Add center points
    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                // Determine if we are within bounds for the center point?
                // Actually, BCC fills space infinitely.
                // The center point is at (x+0.5, y+0.5, z+0.5) * scale
                // But usually crystallographers describe it as corner + center.
                // Let's stick to the cell definition.
                // If we want a "grid" of BCC, we just add the offset points.

                // Note: If we just add +0.5 to everything, we might go slightly outside the "box"
                // defined by SC, but that's fine.

                // Optimization: Don't add if it's strictly outside the visual cube?
                // Nah, let's keep it simple.

                points.push(Point3::new(
                    (x as f64 + 0.5) * scale,
                    (y as f64 + 0.5) * scale,
                    (z as f64 + 0.5) * scale,
                ));
            }
        }
    }
    points
}

fn generate_fcc(size: usize, scale: f64) -> Vec<Point3<f64>> {
    let mut points = generate_sc(size, scale);
    let range = size as isize;
    let start = -range;
    let end = range;

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
                let fx = x as f64;
                let fy = y as f64;
                let fz = z as f64;

                // Face centers:
                // XY face: (x+0.5, y+0.5, z)
                points.push(Point3::new(
                    (fx + 0.5) * scale,
                    (fy + 0.5) * scale,
                    fz * scale,
                ));
                // XZ face: (x+0.5, y, z+0.5)
                points.push(Point3::new(
                    (fx + 0.5) * scale,
                    fy * scale,
                    (fz + 0.5) * scale,
                ));
                // YZ face: (x, y+0.5, z+0.5)
                points.push(Point3::new(
                    fx * scale,
                    (fy + 0.5) * scale,
                    (fz + 0.5) * scale,
                ));
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sc_generation() {
        let size = 1; // Range -1..=1, count 3
        let lattice = Lattice::new(LatticeType::SimpleCubic, size, 1.0);
        // (1*2 + 1)^3 = 3^3 = 27
        assert_eq!(lattice.points.len(), 27);
    }

    #[test]
    fn test_bcc_generation() {
        let size = 1;
        let lattice = Lattice::new(LatticeType::BodyCenteredCubic, size, 1.0);
        // 27 * 2 = 54
        assert_eq!(lattice.points.len(), 54);
    }

    #[test]
    fn test_fcc_generation() {
        let size = 1;
        let lattice = Lattice::new(LatticeType::FaceCenteredCubic, size, 1.0);
        // 27 * 4 = 108
        assert_eq!(lattice.points.len(), 108);
    }
}
