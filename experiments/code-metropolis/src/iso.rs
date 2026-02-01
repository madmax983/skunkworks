use ratatui::style::Color;
use ratatui::widgets::canvas::Line;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64, // Height
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub scale: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub angle: f64, // Rotation in radians
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            angle: std::f64::consts::FRAC_PI_4, // 45 degrees
        }
    }
}

pub fn project(p: Point3D, camera: &Camera) -> Point2D {
    // Rotate x, z around y axis (center of rotation is 0,0,0 usually, but here we just rotate coords)
    let x_rot = p.x * camera.angle.cos() - p.z * camera.angle.sin();
    let z_rot = p.x * camera.angle.sin() + p.z * camera.angle.cos();

    // Isometric projection for terminal (2:1 aspect ratio approx)
    // x_screen = x - z
    // y_screen = (x + z) / 2 - y

    let iso_x = x_rot - z_rot;
    let iso_y = (x_rot + z_rot) * 0.5 - p.y;

    Point2D {
        x: (iso_x * camera.scale) + camera.offset_x,
        y: (iso_y * camera.scale) + camera.offset_y,
    }
}

pub struct Cube {
    pub origin: Point3D,
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub color: Color,
}

impl Cube {
    pub fn get_lines(&self, camera: &Camera) -> Vec<Line> {
        let x = self.origin.x;
        let y = self.origin.y;
        let z = self.origin.z;
        let w = self.width;
        let h = self.height;
        let d = self.depth;

        let vertices = [
            Point3D { x, y, z },             // 0: bottom-back-left
            Point3D { x: x+w, y, z },        // 1: bottom-back-right
            Point3D { x: x+w, y, z: z+d },   // 2: bottom-front-right
            Point3D { x, y, z: z+d },        // 3: bottom-front-left
            Point3D { x, y: y+h, z },        // 4: top-back-left
            Point3D { x: x+w, y: y+h, z },   // 5: top-back-right
            Point3D { x: x+w, y: y+h, z: z+d }, // 6: top-front-right
            Point3D { x, y: y+h, z: z+d },      // 7: top-front-left
        ];

        let proj: Vec<Point2D> = vertices.iter().map(|v| project(*v, camera)).collect();

        // Define the 12 edges
        let edges = [
            (0,1), (1,2), (2,3), (3,0), // Bottom
            (4,5), (5,6), (6,7), (7,4), // Top
            (0,4), (1,5), (2,6), (3,7)  // Vertical
        ];

        edges.iter().map(|&(start, end)| {
            Line {
                x1: proj[start].x,
                y1: proj[start].y,
                x2: proj[end].x,
                y2: proj[end].y,
                color: self.color,
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_origin() {
        let camera = Camera {
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            angle: 0.0,
        };
        let p = Point3D { x: 0.0, y: 0.0, z: 0.0 };
        let proj = project(p, &camera);
        assert!((proj.x - 0.0).abs() < 1e-6);
        assert!((proj.y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cube_lines() {
        let camera = Camera::default();
        let cube = Cube {
            origin: Point3D { x: 0.0, y: 0.0, z: 0.0 },
            width: 10.0,
            height: 10.0,
            depth: 10.0,
            color: Color::White,
        };
        let lines = cube.get_lines(&camera);
        assert_eq!(lines.len(), 12);
    }
}
