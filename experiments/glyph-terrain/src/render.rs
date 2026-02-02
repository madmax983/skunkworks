use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use noise::{NoiseFn, Perlin};
use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line},
};
use crate::sdf::Grid;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32, // in radians
}

impl Camera {
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos,
            yaw: -std::f32::consts::FRAC_PI_2, // Look -Z
            pitch: 0.0,
            fov: 60.0_f32.to_radians(),
        }
    }

    pub fn forward(&self) -> Vec3 {
        let (y_sin, y_cos) = self.yaw.sin_cos();
        let (p_sin, p_cos) = self.pitch.sin_cos();
        Vec3::new(y_cos * p_cos, p_sin, y_sin * p_cos).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    pub fn view_projection(&self, aspect_ratio: f32) -> Mat4 {
        let view = Mat4::look_at_rh(self.pos, self.pos + self.forward(), Vec3::Y);
        let projection = Mat4::perspective_rh(self.fov, aspect_ratio, 0.1, 1000.0);
        projection * view
    }
}

pub struct Terrain {
    pub sdf: Grid,
    pub scale: f32,      // World units per grid unit
    pub height_mult: f32,// Vertical scaling
    perlin: Perlin,
}

impl Terrain {
    pub fn new(sdf: Grid) -> Self {
        Self {
            sdf,
            scale: 0.5,
            height_mult: 2.0,
            perlin: Perlin::new(42),
        }
    }

    pub fn get_height(&self, x: usize, z: usize) -> f32 {
        let sdf_val = self.sdf.get(x, z);

        // Base height: only positive SDF (inside glyph) contributes to height
        let base = if sdf_val > 0.0 { sdf_val } else { 0.0 };

        // Noise
        let nx = x as f64 * 0.1;
        let nz = z as f64 * 0.1;
        let noise = self.perlin.get([nx, nz]) as f32;

        // Modulate noise by base height (smooth transition)
        // Add noise only if base > 0 to keep ground flat, or allow rugged ground
        // Let's have rugged ground but mountains are higher.

        let ground_noise = noise * 0.2; // Small bumps everywhere
        let mountain_noise = noise * 0.5; // Bigger bumps on mountains

        if base > 0.0 {
            (base + mountain_noise) * self.height_mult
        } else {
            ground_noise * 0.5 // Subtle ground
        }
    }
}

pub fn draw_terrain(ctx: &mut Context, terrain: &Terrain, camera: &Camera, screen_size: (f32, f32)) {
    let aspect = screen_size.0 / screen_size.1;
    // TUI chars are roughly 1:2 aspect ratio, so we adjust aspect ratio passed to projection
    // Actually screen_size is in "points" of the canvas. If canvas is 100x100, aspect is 1.
    // But physically it depends. Let's assume 1.0 for now.

    let vp = camera.view_projection(aspect);

    let width = terrain.sdf.width;
    let height = terrain.sdf.height; // mapped to Z

    // Resolution step to avoid drawing too many lines
    let step = 1;

    // Helper to project point
    let project = |x: usize, z: usize| -> Option<(f64, f64)> {
        let h = terrain.get_height(x, z);
        // World coordinates
        // Center the text roughly?
        // Let's place 0,0 at corner for now.
        let wx = x as f32 * terrain.scale;
        let wy = h;

        // Z is negative in OpenGL RH convention usually?
        // Wait, look_at_rh: +Y up, -Z forward.
        // So let's map grid Y (which I call Z) to world -Z?
        // Or just map to +Z and camera looks at +Z?
        // Let's map to -Z for standard "forward into screen".
        // So world z = -z_index * scale.

        let p = Vec4::new(wx, wy, -(z as f32 * terrain.scale), 1.0);

        let clip = vp * p;

        if clip.w <= 0.0 { return None; } // Behind camera

        // NDC
        let ndc = clip.xyz() / clip.w;

        // Check bounds -1 to 1 (roughly)
        // If far out, don't draw?

        // Map NDC (-1..1) to Canvas bounds.
        // Let's assume Canvas bounds are 0..100, 0..100
        // x: -1 -> 0, 1 -> 100
        let sx = (ndc.x + 1.0) * 0.5 * 100.0;
        let sy = (ndc.y + 1.0) * 0.5 * 100.0;

        Some((sx as f64, sy as f64))
    };

    // Draw lines along X
    for z in (0..height).step_by(step) {
        for x in (0..width - step).step_by(step) {
            if let (Some(p1), Some(p2)) = (project(x, z), project(x + step, z)) {
                 // Color based on height?
                 let h = terrain.get_height(x, z);
                 let color = if h > 2.0 { Color::White }
                             else if h > 0.5 { Color::Red }
                             else { Color::DarkGray };

                 ctx.draw(&Line {
                     x1: p1.0, y1: p1.1,
                     x2: p2.0, y2: p2.1,
                     color,
                 });
            }
        }
    }

    // Draw lines along Z
    for x in (0..width).step_by(step) {
        for z in (0..height - step).step_by(step) {
            if let (Some(p1), Some(p2)) = (project(x, z), project(x, z + step)) {
                 let h = terrain.get_height(x, z);
                 let color = if h > 2.0 { Color::White }
                             else if h > 0.5 { Color::Red }
                             else { Color::DarkGray };

                 ctx.draw(&Line {
                     x1: p1.0, y1: p1.1,
                     x2: p2.0, y2: p2.1,
                     color,
                 });
            }
        }
    }
}
