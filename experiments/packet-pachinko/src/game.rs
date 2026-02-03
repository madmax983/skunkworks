use crate::physics::{resolve_collision, PacketKind, Particle, Pin, PinKind, Vec2};
use rand::Rng;

pub struct GameState {
    pub particles: Vec<Particle>,
    pub pins: Vec<Pin>,
    pub score: i32,
    pub width: f64,
    pub height: f64,
    pub game_over: bool,
    pub gravity: Vec2,
    pub cursor_pos: Vec2,
    pub tool_kind: PinKind,
    pub spawned_count: usize,
}

impl GameState {
    pub fn new(width: f64, height: f64) -> Self {
        let mut pins = Vec::new();

        // Generate a grid of pins (pegboard)
        let rows = 10;
        let cols = 15;
        let spacing_x = width / cols as f64;
        let spacing_y = (height * 0.6) / rows as f64;
        let start_y = height * 0.2;

        for r in 0..rows {
            let offset = if r % 2 == 0 { spacing_x / 2.0 } else { 0.0 };
            for c in 0..cols {
                let x = (c as f64 * spacing_x) + offset + spacing_x * 0.5;
                let y = start_y + (r as f64 * spacing_y);

                // Keep pins somewhat centered
                if x > 5.0 && x < width - 5.0 {
                    pins.push(Pin::new(x, y, PinKind::Bumper));
                }
            }
        }

        Self {
            particles: Vec::new(),
            pins,
            score: 0,
            width,
            height,
            game_over: false,
            gravity: Vec2::new(0.0, 30.0), // Positive Y is down
            cursor_pos: Vec2::new(width / 2.0, height / 2.0),
            tool_kind: PinKind::Blocker,
            spawned_count: 0,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.game_over {
            return;
        }

        // Spawn packets
        // Every 60 ticks approx (assuming 60fps) -> 1 sec
        // Let's use random chance
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.05) {
            // 5% chance per tick
            self.spawn_packet();
        }

        // Update particles
        for p in &mut self.particles {
            p.update(dt, self.gravity);
        }

        // Collisions
        for p in &mut self.particles {
            if !p.active {
                continue;
            }

            for pin in &self.pins {
                resolve_collision(p, pin);
            }

            // Wall collisions
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x *= -0.5;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x *= -0.5;
            }
        }

        // Check bins
        self.check_bins();

        // Cleanup inactive
        self.particles.retain(|p| p.active);
    }

    fn spawn_packet(&mut self) {
        let mut rng = rand::thread_rng();
        let x = self.width / 2.0 + rng.gen_range(-5.0..5.0);

        let kind = match rng.gen_range(0..10) {
            0..=1 => PacketKind::Malware, // 20%
            2..=5 => PacketKind::Ssh,     // 40%
            _ => PacketKind::Http,        // 40%
        };

        let mut p = Particle::new(x, 0.0, kind);
        p.vel = Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(5.0..10.0)); // Initial push down
        self.particles.push(p);
        self.spawned_count += 1;
    }

    fn check_bins(&mut self) {
        let bin_y = self.height - 2.0; // Bottom area
        let bin_width = self.width / 3.0;

        for p in &mut self.particles {
            if !p.active {
                continue;
            }

            if p.pos.y > bin_y {
                p.active = false; // It landed

                // Determine bin index
                // 0: DROP (Left), 1: SSH (Middle), 2: HTTP (Right)
                let bin_idx = (p.pos.x / bin_width).floor() as i32;

                let points = match (p.kind, bin_idx) {
                    (PacketKind::Malware, 0) => 50,   // Malware caught in Drop
                    (PacketKind::Malware, _) => -100, // Malware leaked!

                    (PacketKind::Ssh, 1) => 20,  // SSH in SSH bin
                    (PacketKind::Ssh, 0) => -5,  // SSH dropped
                    (PacketKind::Ssh, _) => -10, // SSH wrong port

                    (PacketKind::Http, 2) => 10, // HTTP in HTTP bin
                    (PacketKind::Http, 0) => -2, // HTTP dropped (minor loss)
                    (PacketKind::Http, _) => -5, // HTTP wrong port
                };

                self.score += points;
            }
        }
    }

    pub fn place_pin(&mut self) {
        // Check if pin already exists at cursor (collision with cursor radius)
        let cursor_pin = Pin::new(self.cursor_pos.x, self.cursor_pos.y, self.tool_kind);

        // Remove existing pin if close
        if let Some(idx) = self
            .pins
            .iter()
            .position(|p| (p.pos - self.cursor_pos).length() < (p.radius + cursor_pin.radius))
        {
            self.pins.remove(idx);
        } else {
            // Add new pin
            self.pins.push(cursor_pin);
        }
    }

    pub fn move_cursor(&mut self, dx: f64, dy: f64) {
        self.cursor_pos.x = (self.cursor_pos.x + dx).clamp(0.0, self.width);
        self.cursor_pos.y = (self.cursor_pos.y + dy).clamp(0.0, self.height);
    }
}
