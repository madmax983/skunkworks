use macroquad::prelude::*;
use ::rand::Rng;
use rayon::prelude::*;
use gray_scott::GrayScott;

const WORLD_SIZE: f32 = 1000.0;
const GRID_SIZE: usize = 200; // Resolution of the Gray-Scott grid

pub struct World {
    pub firewalls: Vec<Firewall>,
    pub gs: GrayScott,
}

pub struct Firewall {
    pub position: Vec2,
    pub radius: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            firewalls: Vec::new(),
            gs: GrayScott::new(GRID_SIZE, GRID_SIZE),
        }
    }

    pub fn add_firewall(&mut self, x: f32, y: f32) {
        self.firewalls.push(Firewall {
            position: Vec2::new(x, y),
            radius: 80.0,
        });
    }

    pub fn clear_firewalls(&mut self) {
        self.firewalls.clear();
    }

    pub fn update(&mut self, particles: &mut ParticleSystem, server_pos: Vec2) {
        // Simple absorption logic
        let server_radius = 50.0;
        let mut absorbed = 0;

        // Add chemical V where server is (attractor)
        let sx = ((server_pos.x / WORLD_SIZE) * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;
        let sy = ((server_pos.y / WORLD_SIZE) * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;

        self.gs.add_chemical(sx, sy, 0.5);

        // Remove particles that got too close to the server (successful packets)
        particles.particles.retain(|p| {
            if (p.position - server_pos).length_squared() < server_radius * server_radius {
                absorbed += 1;
                false // removed
            } else {
                true // kept
            }
        });

        // Add chemical V where particles are (DDoS packets act as predator chemical)
        for p in &particles.particles {
            let px = ((p.position.x / WORLD_SIZE) * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;
            let py = ((p.position.y / WORLD_SIZE) * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32) as usize;
            self.gs.add_chemical(px, py, 0.05); // Deposition
        }

        // Apply firewalls as high "kill" regions
        // In this variant, we do a custom update loop or just use standard parameters with localized perturbations.
        // For simplicity, we step the simulation.

        // standard spots: feed=0.03, kill=0.062
        self.gs.update(0.03, 0.062, 1.0);
    }
}

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(20000),
            max_particles: 20000,
        }
    }

    pub fn add_particle(&mut self, position: Vec2) {
        if self.particles.len() < self.max_particles {
            let mut rng = ::rand::thread_rng();
            self.particles.push(Particle {
                position,
                velocity: Vec2::ZERO,
                speed: rng.gen_range(50.0..150.0),
            });
        }
    }

    pub fn update(&mut self, dt: f32, world: &World, server_pos: Vec2) {
        let dt = dt.min(0.05); // cap dt

        // Convert world firewalls to a form easy for parallel iteration
        let firewalls: Vec<(Vec2, f32)> = world.firewalls.iter().map(|f| (f.position, f.radius)).collect();
        let grid_w = world.gs.width();
        let grid_h = world.gs.height();
        let u_chem = world.gs.u().to_vec(); // Copy state for read-only parallel access
        let v_chem = world.gs.v().to_vec();

        // Calculate forces (parallel)
        let forces: Vec<Vec2> = self
            .particles
            .par_iter()
            .map(|p| {
                let mut force = Vec2::ZERO;

                // Gradient ascent on chemical V (attracted to other packets and server)
                // Gradient descent on chemical U (repelled from firewalls/neutral space)
                let px = ((p.position.x / WORLD_SIZE) * grid_w as f32).clamp(0.0, (grid_w - 1) as f32) as isize;
                let py = ((p.position.y / WORLD_SIZE) * grid_h as f32).clamp(0.0, (grid_h - 1) as f32) as isize;

                let get_v = |x: isize, y: isize| -> f32 {
                    let nx = x.clamp(0, (grid_w - 1) as isize) as usize;
                    let ny = y.clamp(0, (grid_h - 1) as isize) as usize;
                    v_chem[ny * grid_w + nx]
                };

                let get_u = |x: isize, y: isize| -> f32 {
                    let nx = x.clamp(0, (grid_w - 1) as isize) as usize;
                    let ny = y.clamp(0, (grid_h - 1) as isize) as usize;
                    u_chem[ny * grid_w + nx]
                };

                let v_dx = get_v(px + 1, py) - get_v(px - 1, py);
                let v_dy = get_v(px, py + 1) - get_v(px, py - 1);

                let u_dx = get_u(px + 1, py) - get_u(px - 1, py);
                let u_dy = get_u(px, py + 1) - get_u(px, py - 1);

                // Attracted to V, Repelled by U
                force += Vec2::new(v_dx, v_dy).normalize_or_zero() * 200.0;
                force -= Vec2::new(u_dx, u_dy).normalize_or_zero() * 100.0;

                // Target Attraction (Server pulling packets in, weak compared to local gradient)
                let dir = server_pos - p.position;
                let dist = dir.length();
                if dist > 0.0 {
                    let server_pull = 50.0;
                    force += (dir / dist) * server_pull;
                }

                // Firewall Repulsion
                for (fw_pos, fw_radius) in &firewalls {
                    let fw_dir = p.position - *fw_pos;
                    let fw_dist = fw_dir.length();
                    if fw_dist < fw_radius * 2.0 {
                        let repulsion = 5000.0 / (fw_dist * fw_dist + 1.0);
                        force += (fw_dir / fw_dist) * repulsion;
                    }
                }

                // Damping
                force -= p.velocity * 0.5;

                force
            })
            .collect();

        // Apply forces and integrate (parallel)
        self.particles
            .par_iter_mut()
            .zip(forces.into_par_iter())
            .for_each(|(p, force)| {
                p.velocity += force * dt;
                p.position += p.velocity * dt;

                // Speed limit
                let current_speed = p.velocity.length();
                if current_speed > p.speed {
                    p.velocity = (p.velocity / current_speed) * p.speed;
                }

                // Keep inside bounds
                p.position.x = p.position.x.clamp(0.0, WORLD_SIZE);
                p.position.y = p.position.y.clamp(0.0, WORLD_SIZE);
            });
    }
}

#[macroquad::main("Morphogenetic Cyberwarfare")]
async fn main() {
    let mut world = World::new();

    // Texture setup
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let mut particles = ParticleSystem::new();

    let server_pos = Vec2::new(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);

    let mut last_update = get_time();

    loop {
        let now = get_time();
        let dt = (now - last_update) as f32;
        last_update = now;

        // Interaction
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let wx = (x / screen_w) * WORLD_SIZE;
            let wy = (y / screen_h) * WORLD_SIZE;
            world.add_firewall(wx, wy);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Spawn attackers (DDoS packets) from edges
        if ::rand::thread_rng().gen_range(0..10) < 8 {
            for _ in 0..10 {
                let spawn_x = if ::rand::thread_rng().gen_range(0..2) == 0 {
                    0.0
                } else {
                    WORLD_SIZE
                };
                let spawn_y = ::rand::thread_rng().gen_range(0.0..WORLD_SIZE);
                particles.add_particle(Vec2::new(spawn_x, spawn_y));
            }
        }

        // Update firewalls effect on chemical grid (Firewalls inject chemical U and kill V)
        for firewall in &world.firewalls {
            let px = ((firewall.position.x / WORLD_SIZE) * GRID_SIZE as f32) as isize;
            let py = ((firewall.position.y / WORLD_SIZE) * GRID_SIZE as f32) as isize;
            let radius = ((firewall.radius / WORLD_SIZE) * GRID_SIZE as f32) as isize;

            for y in -radius..=radius {
                for x in -radius..=radius {
                    if x * x + y * y <= radius * radius {
                        let nx = (px + x).clamp(0, (GRID_SIZE - 1) as isize) as usize;
                        let ny = (py + y).clamp(0, (GRID_SIZE - 1) as isize) as usize;
                        world.gs.u_mut()[ny * GRID_SIZE + nx] = 1.0;
                        world.gs.v_mut()[ny * GRID_SIZE + nx] = 0.0;
                    }
                }
            }
        }

        world.update(&mut particles, server_pos);
        particles.update(dt, &world, server_pos);

        // Render to image
        image.bytes.fill(0);

        // Draw chemical V as background (Reaction-Diffusion trail)
        let v_chem = world.gs.v();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let v = v_chem[y * GRID_SIZE + x];
                if v > 0.05 {
                    let c = Color::new(0.0, v * 0.8, v, 1.0);
                    // Map GRID_SIZE to image coordinates
                    let ix = (x as f32 / GRID_SIZE as f32 * width as f32) as i32;
                    let iy = (y as f32 / GRID_SIZE as f32 * height as f32) as i32;
                    let block_w = (width / GRID_SIZE) as i32;
                    let block_h = (height / GRID_SIZE) as i32;

                    for by in 0..block_h {
                        for bx in 0..block_w {
                            draw_pixel_on_image(&mut image, ix + bx, iy + by, c);
                        }
                    }
                }
            }
        }

        // Draw Firewalls
        for firewall in &world.firewalls {
            let px = (firewall.position.x / WORLD_SIZE * width as f32) as i32;
            let py = (firewall.position.y / WORLD_SIZE * height as f32) as i32;
            let radius = (firewall.radius / WORLD_SIZE * width as f32) as i32;
            draw_circle_on_image(&mut image, px, py, radius, Color::new(1.0, 0.0, 0.0, 0.5));
        }

        // Draw Server
        let px = (server_pos.x / WORLD_SIZE * width as f32) as i32;
        let py = (server_pos.y / WORLD_SIZE * height as f32) as i32;
        draw_circle_on_image(&mut image, px, py, 30, Color::new(0.0, 1.0, 0.0, 1.0));

        // Draw particles
        for p in &particles.particles {
            let px = (p.position.x / WORLD_SIZE * width as f32) as i32;
            let py = (p.position.y / WORLD_SIZE * height as f32) as i32;
            draw_pixel_on_image(&mut image, px, py, WHITE);
        }

        texture.update(&image);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_text("Morphogenetic Cyberwarfare (gray-ddos)", 10.0, 20.0, 20.0, WHITE);
        draw_text("Left Click: Deploy Chemical Firewall", 10.0, 40.0, 20.0, WHITE);
        draw_text("C: Clear Firewalls", 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Packets: {}", particles.particles.len()), 10.0, 80.0, 20.0, WHITE);

        next_frame().await
    }
}

fn draw_pixel_on_image(img: &mut Image, x: i32, y: i32, color: Color) {
    if x >= 0 && x < img.width as i32 && y >= 0 && y < img.height as i32 {
        img.set_pixel(x as u32, y as u32, color);
    }
}

fn draw_circle_on_image(img: &mut Image, cx: i32, cy: i32, r: i32, color: Color) {
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= r * r {
                draw_pixel_on_image(img, cx + x, cy + y, color);
            }
        }
    }
}
