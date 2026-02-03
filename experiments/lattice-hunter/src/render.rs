use nalgebra::{Point3, Rotation3, Vector3};

pub struct Camera {
    pub position: Point3<f64>,
    pub target: Point3<f64>,
    pub fov: f64,
    pub aspect: f64,
    pub rotation: Rotation3<f64>,
    pub zoom: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, -50.0),
            target: Point3::origin(),
            fov: 60.0f64.to_radians(),
            aspect: 2.0,
            rotation: Rotation3::identity(),
            zoom: 1.0,
        }
    }

    pub fn project(&self, point: &Point3<f64>, width: f64, height: f64) -> Option<(f64, f64, f64)> {
        // Rotate the world (inverse camera rotation)
        let rotated = self.rotation * (point - self.target.coords) + self.target.coords;

        // Vector from camera to point (Camera is at Z = -50 looking at +Z)
        // Wait, if camera is at -50, and looking at 0, points at 0 are at distance 50.
        // Let's standardise: Camera at Origin, world transformed.

        // Actually, let's keep it simple:
        // 1. Rotate point around target (0,0,0).
        // 2. Translate point by (0, 0, distance_from_camera).

        // Let's assume camera looks down +Z.
        // Points are around 0,0,0.
        // We move points by +Z so they are in front of camera.

        let dist = 60.0; // Distance from camera to center of rotation
        let v = rotated;

        // Apply perspective
        // z_camera = v.z + dist
        let z_cam = v.z + dist;

        if z_cam <= 1.0 {
            return None;
        } // Near clip

        let scale = self.zoom * 100.0;
        let x = v.x / z_cam * scale * self.aspect;
        let y = -v.y / z_cam * scale; // Flip Y

        let screen_x = x + width / 2.0;
        let screen_y = y + height / 2.0;

        Some((screen_x, screen_y, z_cam))
    }

    pub fn rotate_x(&mut self, angle: f64) {
        let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), angle);
        self.rotation = rot * self.rotation;
    }

    pub fn rotate_y(&mut self, angle: f64) {
        let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), angle);
        self.rotation = rot * self.rotation;
    }
}
