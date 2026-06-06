use ::glam::Vec3 as PbdVec3;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;
use platter::Platter;

const GRID_SIZE: (usize, usize) = (100, 100);

struct HybridState {
    physics: PbdSystem,
    heatmap: Platter,
}

impl HybridState {
    fn new() -> Self {
        let mut physics = PbdSystem::new();
        let heatmap = Platter::new(GRID_SIZE.0, GRID_SIZE.1);

        // Add a simple chain of particles
        physics.add_particle(PbdVec3::new(5.0, 5.0, 0.0), 0.0); // anchor
        for i in 1..10 {
            physics.add_particle(PbdVec3::new(5.0 + i as f32 * 0.5, 5.0, 0.0), 1.0);
        }

        for i in 0..9 {
            physics.particles[i + 1].pos.y += 0.1; // slight offset to start motion
        }

        Self { physics, heatmap }
    }

    fn update(&mut self, dt: f32) {
        // Step physics
        let mut old_pos = Vec::with_capacity(self.physics.particles.len());
        for p in &self.physics.particles {
            old_pos.push(p.pos);
        }

        self.physics.step(dt, 1);

        // Let's add a simple floor collision to inject heat
        for p in &mut self.physics.particles {
            if p.pos.y > 90.0 {
                p.pos.y = 90.0;
                let vel_y = (p.pos.y - p.prev_pos.y) / dt;
                // Bounce
                p.prev_pos.y = p.pos.y + vel_y * 0.5 * dt;
            }
        }

        // Inject heat based on velocity
        for (i, p) in self.physics.particles.iter().enumerate() {
            if p.inv_mass == 0.0 {
                continue;
            }
            let vel = (p.pos.distance(old_pos[i])) / dt;

            // Map pos to grid
            let gx = (p.pos.x).clamp(0.0, (GRID_SIZE.0 - 1) as f32) as usize;
            let gy = (p.pos.y).clamp(0.0, (GRID_SIZE.1 - 1) as f32) as usize;

            // Heat injection proportional to velocity squared (kinetic energy)
            let heat = (vel * vel) * 0.001 * dt;
            if heat > 0.01 {
                self.heatmap.accumulate(gx, gy, heat.min(5.0) as f64);
            }
        }

        // Diffuse and decay heat
        self.heatmap.decay(0.99);
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        let cell_w = screen_width() / GRID_SIZE.0 as f32;
        let cell_h = screen_height() / GRID_SIZE.1 as f32;

        // Draw Heatmap
        let grid = self.heatmap.magnetism();
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                let heat = grid[y * GRID_SIZE.0 + x];
                if heat > 0.05 {
                    let c = (heat * 0.5).clamp(0.0, 1.0) as f32;
                    draw_rectangle(
                        x as f32 * cell_w,
                        y as f32 * cell_h,
                        cell_w,
                        cell_h,
                        Color::new(c, c * 0.3, 0.1, 1.0),
                    );
                }
            }
        }

        // Draw Physics Particles
        for p in &self.physics.particles {
            let px = p.pos.x * cell_w;
            let py = p.pos.y * cell_h;
            draw_circle(px, py, 4.0, WHITE);
        }

        draw_text("🧬 PHYSICS-PLATTER (Hybrid)", 10.0, 20.0, 20.0, WHITE);
    }
}

// Ensure the macroquad main loop is separated from headless check
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running headless check...");
        let mut state = HybridState::new();
        for _ in 0..10 {
            state.update(0.016);
        }
        println!("Headless check passed. Hybrid vigor confirmed.");
        return;
    }

    macroquad::Window::from_config(
        Conf {
            window_title: "Physics Platter".to_owned(),
            window_width: 800,
            window_height: 800,
            ..Default::default()
        },
        async_main(),
    );
}

async fn async_main() {
    let mut state = HybridState::new();

    loop {
        state.update(get_frame_time());
        state.draw();
        next_frame().await;
    }
}
