use crate::terrain::Terrain;
use rand::Rng;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vy: f64,
    pub alive: bool,
    pub char: char,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub width: usize,
    pub height: usize,
    pub gravity: f64,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            gravity: 50.0,
        }
    }

    pub fn spawn(&mut self, x: usize, count: usize) {
        let mut rng = rand::thread_rng();
        // chars to represent deleted code debris
        let chars = ['-', 'x', 'd', 'e', 'l', ';', '}', '{', '/', '*', '#'];

        for _ in 0..count {
            // Spread slightly around x
            let spread = rng.gen_range(-2.0..2.0);
            let px = (x as f64 + spread).clamp(0.0, self.width as f64 - 1.0);

            self.particles.push(Particle {
                x: px,
                y: 0.0, // Start at top
                vy: rng.gen_range(5.0..15.0), // Initial downward velocity
                alive: true,
                char: chars[rng.gen_range(0..chars.len())],
            });
        }
    }

    pub fn update(&mut self, dt: f64, terrain: &mut Terrain) {
        for p in &mut self.particles {
            if !p.alive { continue; }

            // Gravity
            p.vy += self.gravity * dt;
            p.y += p.vy * dt;

            // Collision with terrain
            // Coordinate system: 0 is Top, Height is Bottom.
            // Terrain grows from Bottom up.
            // Ground Y level = Height - TerrainHeight

            let tx = p.x as usize;
            if tx < terrain.width {
                let h = terrain.get_height(tx);
                let ground_y = self.height as f64 - h;

                if p.y >= ground_y {
                    // Hit ground
                    p.alive = false;
                    // Erode
                    terrain.erode(tx, 1.0);
                    // Maybe splash or slide? For now just disappear and erode.
                }
            }

            // Bounds check
            if p.y > self.height as f64 {
                p.alive = false;
            }
        }

        // Cleanup dead
        self.particles.retain(|p| p.alive);
    }
}
