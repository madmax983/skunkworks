use crate::physics::CosmicString;
use crate::growth::Vine;
use glam::Vec3;
use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line},
};

pub struct Camera {
    pub azimuth: f32,
    pub elevation: f32,
    pub zoom: f32,
    pub offset: (f64, f64),
}

impl Camera {
    pub fn new() -> Self {
        Self {
            azimuth: 0.0,
            elevation: 0.0,
            zoom: 1.0,
            offset: (0.0, 0.0),
        }
    }

    /// Project 3D point to 2D canvas coordinates
    pub fn project(&self, p: Vec3) -> (f64, f64) {
        // Rotate around Y axis (Azimuth)
        let x1 = p.x * self.azimuth.cos() - p.z * self.azimuth.sin();
        let z1 = p.x * self.azimuth.sin() + p.z * self.azimuth.cos();
        let y1 = p.y;

        // Rotate around X axis (Elevation)
        let y2 = y1 * self.elevation.cos() - z1 * self.elevation.sin();
        let _z2 = y1 * self.elevation.sin() + z1 * self.elevation.cos();
        let x2 = x1;

        // Apply Zoom and Offset
        // Canvas usually has center at (0,0) or we configure it.
        // We'll assume canvas is centered.
        let u = x2 as f64 * self.zoom as f64 + self.offset.0;
        let v = y2 as f64 * self.zoom as f64 + self.offset.1;

        (u, v)
    }
}

pub fn draw_cosmic_string(ctx: &mut Context, string: &CosmicString, camera: &Camera) {
    if string.nodes.is_empty() {
        return;
    }

    let mut points_2d = Vec::with_capacity(string.nodes.len());
    for node in &string.nodes {
        points_2d.push(camera.project(node.pos));
    }

    // Draw segments
    for i in 0..points_2d.len() - 1 {
        let (x1, y1) = points_2d[i];
        let (x2, y2) = points_2d[i + 1];

        // Color could be based on tension or velocity?
        // Let's use Cyan for now.
        let color = Color::Cyan;

        ctx.draw(&Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    }
}

pub fn draw_vine(ctx: &mut Context, vine: &Vine, camera: &Camera) {
    for vein in &vine.veins {
        if let Some(parent_idx) = vein.parent_idx {
            let parent = &vine.veins[parent_idx];
            let (x1, y1) = camera.project(parent.pos);
            let (x2, y2) = camera.project(vein.pos);

            // Color gradient based on thickness (which usually implies age/main branch)
            let color = if vein.thickness > 2.0 {
                Color::Rgb(34, 139, 34) // Forest Green for thick branches
            } else if vein.thickness > 1.0 {
                Color::Rgb(50, 205, 50) // Lime Green
            } else {
                Color::Rgb(144, 238, 144) // Light Green for tips
            };

            ctx.draw(&Line {
                x1,
                y1,
                x2,
                y2,
                color,
            });
        }
    }

    // Draw active attractors (debug/visual flair)
    /*
    for att in &vine.attractors {
        if att.active {
            let (x, y) = camera.project(att.pos);
            ctx.print(x, y, ".".to_string());
        }
    }
    */
}
