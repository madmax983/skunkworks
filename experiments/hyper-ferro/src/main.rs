use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;
use ::rand::Rng;
use macroquad::prelude::*;

const GRID_SIZE: usize = 4; // 4x4x4x4 = 256 nodes

struct HyperLattice {
    spins: Vec<i8>, // +1 or -1
    size: usize,
}

impl HyperLattice {
    fn new(size: usize) -> Self {
        let mut rng = ::rand::thread_rng();
        let total = size * size * size * size;
        let mut spins = Vec::with_capacity(total);
        for _ in 0..total {
            spins.push(if rng.gen_bool(0.5) { 1 } else { -1 });
        }
        Self { spins, size }
    }

    fn index(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        x + self.size * (y + self.size * (z + self.size * w))
    }

    fn coords(&self, idx: usize) -> (usize, usize, usize, usize) {
        let x = idx % self.size;
        let y = (idx / self.size) % self.size;
        let z = (idx / (self.size * self.size)) % self.size;
        let w = (idx / (self.size * self.size * self.size)) % self.size;
        (x, y, z, w)
    }

    fn update(&mut self, temp: f32) {
        let mut rng = ::rand::thread_rng();
        let size = self.size as isize; // Use isize for wrapping logic

        // Metropolis update - iterate N times
        let n_updates = self.spins.len();

        for _ in 0..n_updates {
            let idx = rng.gen_range(0..self.spins.len());
            let (x, y, z, w) = self.coords(idx);
            let s = self.spins[idx];

            // Sum of neighbors
            let mut neighbors_sum = 0;

            let dirs = [
                (1, 0, 0, 0),
                (-1, 0, 0, 0),
                (0, 1, 0, 0),
                (0, -1, 0, 0),
                (0, 0, 1, 0),
                (0, 0, -1, 0),
                (0, 0, 0, 1),
                (0, 0, 0, -1),
            ];

            for (dx, dy, dz, dw) in dirs {
                let nx = (x as isize + dx).rem_euclid(size) as usize;
                let ny = (y as isize + dy).rem_euclid(size) as usize;
                let nz = (z as isize + dz).rem_euclid(size) as usize;
                let nw = (w as isize + dw).rem_euclid(size) as usize;
                let n_idx = self.index(nx, ny, nz, nw);
                neighbors_sum += self.spins[n_idx] as i32;
            }

            let delta_e = 2.0 * (s as f32) * (neighbors_sum as f32); // dE = 2 * s * H_local

            if delta_e <= 0.0 || rng.gen::<f32>() < (-delta_e / temp.max(0.001)).exp() {
                self.spins[idx] = -s; // Flip
            }
        }
    }
}

#[macroquad::main("Hyper Ferro")]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let mut lattice = HyperLattice::new(GRID_SIZE);

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    // User controlled temp offset
    let mut temp_offset = 0.0f32;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Rotation driven by Load + Time
        let rot_speed = 0.1 + monitor.swap_usage;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * rot_speed * 0.2;

        // Input Camera
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

        // Temp control
        if is_key_down(KeyCode::Equal) {
            temp_offset += dt * 5.0;
        }
        if is_key_down(KeyCode::Minus) {
            temp_offset -= dt * 5.0;
        }

        // Ising Update
        // Temp base = 2.0 + CPU usage * 10.0 + offset
        // 4D Ising Critical Temp is roughly 6.68, so we center around there
        let temp = 4.0 + (monitor.cpu_usage * 10.0) + temp_offset;
        lattice.update(temp.max(0.0));

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

        // Draw Lattice
        // Map grid [0..size] to [-1..1]
        // Scale dimensions based on metrics

        // Breathing
        let breath = (get_time() as f32 * (1.0 + monitor.load_avg)).sin() * 0.1;
        let scale_w = 1.0 + breath + monitor.mem_usage;

        let size_f = GRID_SIZE as f32;
        let offset = size_f / 2.0 - 0.5;

        let transform = |idx: usize| -> Vec3 {
            let (x, y, z, w) = lattice.coords(idx);
            let vx = (x as f32 - offset) / (size_f * 0.5);
            let vy = (y as f32 - offset) / (size_f * 0.5);
            let vz = (z as f32 - offset) / (size_f * 0.5);
            let vw = (w as f32 - offset) / (size_f * 0.5);

            let v = Vec4::new(vx, vy, vz, vw).scale_dim(1.0, 1.0, 1.0, scale_w);
            let v = v
                .rotate_xw(angle_xw)
                .rotate_yw(angle_yw)
                .rotate_zw(angle_zw);
            v.project_to_3d(4.0)
        };

        // Draw Nodes
        for i in 0..lattice.spins.len() {
            let p = transform(i);
            let s = lattice.spins[i];

            // Color: Red = Up, Blue = Down
            let color = if s > 0 {
                Color::new(1.0, 0.2, 0.2, 1.0)
            } else {
                Color::new(0.2, 0.2, 1.0, 1.0)
            };

            draw_sphere(p, 0.08, None, color);
        }

        // Draw Edges (Only along positive axes to avoid doubles)
        let dirs = [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)];
        for i in 0..lattice.spins.len() {
            let (x, y, z, w) = lattice.coords(i);

            for (dx, dy, dz, dw) in dirs {
                if x + dx < GRID_SIZE
                    && y + dy < GRID_SIZE
                    && z + dz < GRID_SIZE
                    && w + dw < GRID_SIZE
                {
                    let j = lattice.index(x + dx, y + dy, z + dz, w + dw);
                    let p1 = transform(i);
                    let p2 = transform(j);

                    let aligned = lattice.spins[i] == lattice.spins[j];
                    let col = if aligned {
                        Color::new(0.5, 0.5, 0.5, 0.1)
                    } else {
                        Color::new(1.0, 1.0, 1.0, 0.3)
                    };
                    draw_line_3d(p1, p2, col);
                }
            }
        }

        set_default_camera();

        draw_text("HYPER-FERRO", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Temp: {:.2} (CPU: {:.0}%)", temp, monitor.cpu_usage * 100.0),
            10.0,
            50.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("Lattice Scale (Mem): {:.2}", scale_w),
            10.0,
            70.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Rotation (Swap/Load): {:.2}", rot_speed),
            10.0,
            90.0,
            20.0,
            GREEN,
        );
        draw_text("Controls: +/- Temp, Arrows Camera", 10.0, 110.0, 20.0, GRAY);

        next_frame().await
    }
}
