use crate::physics::ShallowWater;

pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub symbol: char,
    pub alive: bool,
}

pub struct Game {
    pub sim: ShallowWater,
    pub units: Vec<Unit>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = ShallowWater::new(width, height);
        // Initialize simple terrain: A valley
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - width as f32 / 2.0;
                let dy = y as f32 - height as f32 / 2.0;
                let dist = (dx * dx + dy * dy).sqrt();

                // Noise would be better, but simple function for now
                // High edges, low center
                let terrain = (dist / 10.0 - 2.0).max(0.0);
                sim.b.set(x, y, terrain);

                // Water in the "lake"
                if dist < 20.0 {
                    sim.h.set(x, y, (3.0 - dist / 10.0).max(0.0));
                }
            }
        }

        Self {
            sim,
            units: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.sim.update();

        // Kill units if water is too deep (flood)
        for unit in &mut self.units {
            if unit.alive {
                let ix = unit.x.round() as usize;
                let iy = unit.y.round() as usize;
                let depth = self.sim.h.get(ix, iy);
                // Velocity also kills?
                let u = self.sim.u.get(ix, iy);
                let v = self.sim.v.get(ix, iy);
                let speed = (u * u + v * v).sqrt();

                if depth > 0.5 || speed > 0.5 {
                    unit.alive = false;
                }
            }
        }
    }

    pub fn spawn_unit(&mut self, x: f32, y: f32) {
        self.units.push(Unit {
            x,
            y,
            symbol: 'U',
            alive: true,
        });
    }

    pub fn terraform(&mut self, x: usize, y: usize, amount: f32) {
        let r = 2;
        for dy in -r..=r {
            for dx in -r..=r {
                // Check bounds
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < self.sim.width as i32 && ny >= 0 && ny < self.sim.height as i32 {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    let current = self.sim.b.get(ux, uy);
                    self.sim.b.set(ux, uy, (current + amount).max(0.0));
                }
            }
        }
    }

    pub fn rain(&mut self, x: usize, y: usize) {
        let r = 3;
        for dy in -r..=r {
            for dx in -r..=r {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.sim.width as i32 && ny >= 0 && ny < self.sim.height as i32 {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    let current = self.sim.h.get(ux, uy);
                    self.sim.h.set(ux, uy, current + 0.5);
                }
            }
        }
    }
}
