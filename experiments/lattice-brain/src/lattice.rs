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

    for x in start..=end {
        for y in start..=end {
            for z in start..=end {
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

                // Face centers
                points.push(Point3::new(
                    (fx + 0.5) * scale,
                    (fy + 0.5) * scale,
                    fz * scale,
                ));
                points.push(Point3::new(
                    (fx + 0.5) * scale,
                    fy * scale,
                    (fz + 0.5) * scale,
                ));
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
