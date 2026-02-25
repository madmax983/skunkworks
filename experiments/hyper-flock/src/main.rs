use ::rand::Rng;
use hyper_system::math::{HyperVector, Vec4};
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;

// --- Boid Logic ---

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f32,
    pub max_force: f32,
    pub view_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub color: Color,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.02..0.05),
            max_force: rng.gen_range(0.001..0.005),
            view_radius: rng.gen_range(0.5..1.0), // In unit hypercube
            separation_weight: rng.gen_range(1.2..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid4D {
    pub position: Vec4,
    pub velocity: Vec4,
    pub acceleration: Vec4,
    pub dna: Dna,
}

impl Default for Boid4D {
    fn default() -> Self {
        Self::new()
    }
}

impl Boid4D {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        // Spawn inside unit tesseract (-1 to 1)
        let pos = Vec4::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        let vel = Vec4::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize_or_zero()
        .scale(0.03);

        Self {
            position: pos,
            velocity: vel,
            acceleration: Vec4::zero(),
            dna: Dna::random(),
        }
    }

    pub fn update(&mut self, bounds: Vec4) {
        // bounds defines the scale of the tesseract
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec4::zero();

        // Bounce off walls (Hypercube Bounds)
        // X
        if self.position.x > bounds.x {
            self.position.x = bounds.x;
            self.velocity.x *= -1.0;
        } else if self.position.x < -bounds.x {
            self.position.x = -bounds.x;
            self.velocity.x *= -1.0;
        }
        // Y
        if self.position.y > bounds.y {
            self.position.y = bounds.y;
            self.velocity.y *= -1.0;
        } else if self.position.y < -bounds.y {
            self.position.y = -bounds.y;
            self.velocity.y *= -1.0;
        }
        // Z
        if self.position.z > bounds.z {
            self.position.z = bounds.z;
            self.velocity.z *= -1.0;
        } else if self.position.z < -bounds.z {
            self.position.z = -bounds.z;
            self.velocity.z *= -1.0;
        }
        // W
        if self.position.w > bounds.w {
            self.position.w = bounds.w;
            self.velocity.w *= -1.0;
        } else if self.position.w < -bounds.w {
            self.position.w = -bounds.w;
            self.velocity.w *= -1.0;
        }
    }

    pub fn flock(&mut self, boids: &[Boid4D]) {
        let separation = self.separation(boids).scale(self.dna.separation_weight);
        let alignment = self.alignment(boids).scale(self.dna.alignment_weight);
        let cohesion = self.cohesion(boids).scale(self.dna.cohesion_weight);

        self.acceleration += separation;
        self.acceleration += alignment;
        self.acceleration += cohesion;
    }

    fn separation(&self, boids: &[Boid4D]) -> Vec4 {
        let mut steer = Vec4::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                let diff = (self.position - other.position).normalize_or_zero();
                let diff = diff / d_sq.sqrt(); // Weight by distance
                steer += diff;
                count += 1;
            }
        }
        if count > 0 {
            steer = steer / (count as f32);
            if steer.length_squared() > 0.0 {
                steer = steer.normalize_or_zero().scale(self.dna.max_speed);
                steer = steer - self.velocity;
                steer = steer.limit(self.dna.max_force);
            }
        }
        steer
    }

    fn alignment(&self, boids: &[Boid4D]) -> Vec4 {
        let mut sum = Vec4::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                sum += other.velocity;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum / (count as f32);
            sum = sum.normalize_or_zero().scale(self.dna.max_speed);
            let steer = sum - self.velocity;
            return steer.limit(self.dna.max_force);
        }
        Vec4::zero()
    }

    fn cohesion(&self, boids: &[Boid4D]) -> Vec4 {
        let mut sum = Vec4::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                sum += other.position;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum / (count as f32);
            return self.seek(sum);
        }
        Vec4::zero()
    }

    fn seek(&self, target: Vec4) -> Vec4 {
        let desired = (target - self.position)
            .normalize_or_zero()
            .scale(self.dna.max_speed);
        let steer = desired - self.velocity;
        steer.limit(self.dna.max_force)
    }
}

// --- Main ---

#[macroquad::main("Hyper-Flock")]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let mut boids: Vec<Boid4D> = (0..100).map(|_| Boid4D::new()).collect();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let angle_zw: f32 = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Rotation driven by Load
        let speed_mult = 1.0 + monitor.load_avg * 2.0;
        angle_xw += dt * 0.2 * speed_mult;
        angle_yw += dt * 0.1 * speed_mult;

        // Update Bounds based on System Metrics
        // X = CPU, Y = RAM, Z = Swap, W = Load
        let bounds = Vec4::new(
            1.0 + monitor.cpu_usage,
            1.0 + monitor.mem_usage,
            1.0 + monitor.swap_usage,
            1.0 + monitor.load_avg * 2.0, // W expands with load
        );

        // Update Boids
        // We need to clone boids for read access during update
        let old_boids = boids.clone();
        for boid in boids.iter_mut() {
            boid.flock(&old_boids);
            boid.update(bounds);
        }

        // Camera Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Precompute sine/cosine for this frame's rotations
        let sin_xw = angle_xw.sin();
        let cos_xw = angle_xw.cos();
        let sin_yw = angle_yw.sin();
        let cos_yw = angle_yw.cos();
        let sin_zw = angle_zw.sin();
        let cos_zw = angle_zw.cos();

        // Draw Boids
        for boid in &boids {
            // Transform 4D -> 3D
            let mut p = boid.position;
            // Rotate in 4D first using fast methods
            p = p.rotate_xw_fast(sin_xw, cos_xw);
            p = p.rotate_yw_fast(sin_yw, cos_yw);
            p = p.rotate_zw_fast(sin_zw, cos_zw);
            // Project
            let p3 = p.project_to_3d(3.0);

            // Color based on W-depth and DNA
            let w_factor = (boid.position.w + bounds.w) / (2.0 * bounds.w + 0.001);
            let color = Color::new(
                boid.dna.color.r * w_factor,
                boid.dna.color.g * w_factor,
                boid.dna.color.b * w_factor,
                1.0,
            );

            draw_sphere(p3.into(), 0.05, None, color);
        }

        // Draw Tesseract Bounds (Wireframe)
        // Similar to tesseract-ops but scaled by bounds
        draw_tesseract_wireframe(bounds, sin_xw, cos_xw, sin_yw, cos_yw, sin_zw, cos_zw);

        set_default_camera();
        draw_text(
            "Hyper-Flock: 4D Boids driven by System Load",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(&format!("Boids: {}", boids.len()), 10.0, 50.0, 20.0, WHITE);
        draw_text(
            &format!("CPU(X): {:.0}%", monitor.cpu_usage * 100.0),
            10.0,
            70.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM(Y): {:.0}%", monitor.mem_usage * 100.0),
            10.0,
            90.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("SWP(Z): {:.0}%", monitor.swap_usage * 100.0),
            10.0,
            110.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!("LOD(W): {:.2}", monitor.load_avg),
            10.0,
            130.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}

fn draw_tesseract_wireframe(
    bounds: Vec4,
    sin_xw: f32,
    cos_xw: f32,
    sin_yw: f32,
    cos_yw: f32,
    sin_zw: f32,
    cos_zw: f32,
) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { bounds.x } else { -bounds.x };
        let y = if i & 2 != 0 { bounds.y } else { -bounds.y };
        let z = if i & 4 != 0 { bounds.z } else { -bounds.z };
        let w = if i & 8 != 0 { bounds.w } else { -bounds.w };
        verts.push(Vec4::new(x, y, z, w));
    }

    let transform = |v: Vec4| -> Vec3 {
        let mut v = v; // Already scaled by bounds
        v = v.rotate_xw_fast(sin_xw, cos_xw);
        v = v.rotate_yw_fast(sin_yw, cos_yw);
        v = v.rotate_zw_fast(sin_zw, cos_zw);
        v.project_to_3d(3.0).into()
    };

    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                let p1 = transform(verts[i]);
                let p2 = transform(verts[j]);
                draw_line_3d(p1, p2, Color::new(0.5, 0.5, 0.5, 0.3));
            }
        }
    }
}
