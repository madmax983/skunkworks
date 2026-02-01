use rand::Rng;

#[derive(Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub char: char,
    pub trail: Vec<(f64, f64)>,
    pub age: usize,
}

impl Particle {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, char: char) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            char,
            trail: Vec::with_capacity(8),
            age: 0,
        }
    }
}

pub struct GravityWell {
    pub x: f64,
    pub y: f64,
    pub mass: f64,
    pub char: char,
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub wells: Vec<GravityWell>,
    pub width: f64,
    pub height: f64,
    pub absorbed_count: usize,
    poem: Vec<char>,
    poem_index: usize,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let poem: Vec<char> = "stars fall into the void where light once danced eternal"
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let mut universe = Self {
            particles: Vec::new(),
            wells: Vec::new(),
            width,
            height,
            absorbed_count: 0,
            poem,
            poem_index: 0,
        };

        // Create gravity wells
        universe.spawn_wells();

        // Initial particles
        for _ in 0..60 {
            universe.spawn_particle();
        }

        universe
    }

    fn spawn_wells(&mut self) {
        let mut rng = rand::thread_rng();
        let well_chars = ['@', '*', '#', '%'];

        // Create 3-5 gravity wells
        let count = rng.gen_range(3..=5);
        for i in 0..count {
            let margin = 10.0;
            self.wells.push(GravityWell {
                x: rng.gen_range(margin..self.width - margin),
                y: rng.gen_range(margin..self.height - margin),
                mass: rng.gen_range(80.0..200.0),
                char: well_chars[i % well_chars.len()],
            });
        }
    }

    fn next_poem_char(&mut self) -> char {
        let c = self.poem[self.poem_index];
        self.poem_index = (self.poem_index + 1) % self.poem.len();
        c
    }

    pub fn spawn_particle(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn from edges with tangential velocity
        let (x, y, vx, vy) = match rng.gen_range(0..4) {
            0 => {
                // Top edge
                let x = rng.gen_range(0.0..self.width);
                (x, 0.0, rng.gen_range(-0.3..0.3), rng.gen_range(0.2..0.5))
            }
            1 => {
                // Bottom edge
                let x = rng.gen_range(0.0..self.width);
                (x, self.height, rng.gen_range(-0.3..0.3), rng.gen_range(-0.5..-0.2))
            }
            2 => {
                // Left edge
                let y = rng.gen_range(0.0..self.height);
                (0.0, y, rng.gen_range(0.2..0.5), rng.gen_range(-0.3..0.3))
            }
            _ => {
                // Right edge
                let y = rng.gen_range(0.0..self.height);
                (self.width, y, rng.gen_range(-0.5..-0.2), rng.gen_range(-0.3..0.3))
            }
        };

        let char = self.next_poem_char();
        self.particles.push(Particle::new(x, y, vx, vy, char));
    }

    pub fn tick(&mut self) {
        let dt = 0.5;
        let absorption_radius = 1.5;
        let trail_length = 6;

        // Update particles
        for particle in &mut self.particles {
            // Store trail
            particle.trail.push((particle.x, particle.y));
            if particle.trail.len() > trail_length {
                particle.trail.remove(0);
            }

            // Calculate gravitational acceleration from all wells
            let mut ax = 0.0;
            let mut ay = 0.0;

            for well in &self.wells {
                let dx = well.x - particle.x;
                let dy = well.y - particle.y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt().max(2.0);

                // F = G * m / r^2, but we use a softer falloff
                let force = well.mass / (dist_sq + 10.0);
                ax += force * dx / dist;
                ay += force * dy / dist;
            }

            // Apply acceleration
            particle.vx += ax * dt * 0.01;
            particle.vy += ay * dt * 0.01;

            // Damping (very slight)
            particle.vx *= 0.999;
            particle.vy *= 0.999;

            // Update position
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;

            particle.age += 1;
        }

        // Check for absorption and remove absorbed particles
        let wells = &self.wells;
        let mut absorbed = 0;
        self.particles.retain(|p| {
            for well in wells {
                let dx = well.x - p.x;
                let dy = well.y - p.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < absorption_radius {
                    absorbed += 1;
                    return false;
                }
            }
            // Also remove if too far out of bounds
            if p.x < -20.0 || p.x > self.width + 20.0 || p.y < -20.0 || p.y > self.height + 20.0 {
                return false;
            }
            true
        });
        self.absorbed_count += absorbed;

        // Spawn new particles to maintain population
        while self.particles.len() < 60 {
            self.spawn_particle();
        }
    }

    pub fn reset(&mut self) {
        self.particles.clear();
        self.wells.clear();
        self.absorbed_count = 0;
        self.poem_index = 0;
        self.spawn_wells();
        for _ in 0..60 {
            self.spawn_particle();
        }
    }
}
