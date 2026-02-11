use macroquad::prelude::*;
use ::rand::Rng;

const GRID_WIDTH: usize = 400;
const GRID_HEIGHT: usize = 300;
const SENSOR_ANGLE: f32 = std::f32::consts::PI / 4.0;
const SENSOR_DIST: f32 = 9.0;
const TURN_SPEED: f32 = 0.2;
const MOVE_SPEED: f32 = 1.0;
const EVAPORATION_RATE: f32 = 0.95;
const DIFFUSION_RATE: f32 = 0.1;

pub struct ChemoGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
}

impl ChemoGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0.0; width * height],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        (y % self.height) * self.width + (x % self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.cells[self.index(x, y)]
    }

    pub fn deposit(&mut self, x: usize, y: usize, amount: f32) {
        let idx = self.index(x, y);
        self.cells[idx] = (self.cells[idx] + amount).min(1.0);
    }

    pub fn update(&mut self) {
        let mut next_cells = self.cells.clone();

        // Diffuse
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                        sum += self.cells[ny * self.width + nx];
                    }
                }
                let avg = sum / 9.0;
                let idx = y * self.width + x;
                next_cells[idx] = self.cells[idx] * (1.0 - DIFFUSION_RATE) + avg * DIFFUSION_RATE;
            }
        }

        // Decay
        for cell in next_cells.iter_mut() {
            *cell *= EVAPORATION_RATE;
        }

        self.cells = next_cells;
    }
}

pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
}

impl Agent {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            pos: vec2(x, y),
            angle: rng.gen::<f32>() * std::f32::consts::PI * 2.0,
        }
    }

    pub fn sense(&self, grid: &ChemoGrid) -> (f32, f32, f32) {
        let sensor_l = self.get_sensor_pos(self.angle - SENSOR_ANGLE);
        let sensor_c = self.get_sensor_pos(self.angle);
        let sensor_r = self.get_sensor_pos(self.angle + SENSOR_ANGLE);

        (
            grid.get(sensor_l.x as usize, sensor_l.y as usize),
            grid.get(sensor_c.x as usize, sensor_c.y as usize),
            grid.get(sensor_r.x as usize, sensor_r.y as usize),
        )
    }

    fn get_sensor_pos(&self, angle: f32) -> Vec2 {
        let offset = vec2(angle.cos(), angle.sin()) * SENSOR_DIST;
        let pos = self.pos + offset;
        // Wrap coordinates
        vec2(
            pos.x.rem_euclid(GRID_WIDTH as f32),
            pos.y.rem_euclid(GRID_HEIGHT as f32),
        )
    }

    pub fn rotate(&mut self, sensors: (f32, f32, f32)) {
        let (l, c, r) = sensors;
        let mut rng = ::rand::thread_rng();

        if c > l && c > r {
            // Stay course
        } else if c < l && c < r {
            // Random turn
            self.angle += (rng.gen::<f32>() - 0.5) * 2.0 * TURN_SPEED;
        } else if l > r {
            self.angle -= TURN_SPEED;
        } else if r > l {
            self.angle += TURN_SPEED;
        }
    }

    pub fn move_forward(&mut self) {
        let vel = vec2(self.angle.cos(), self.angle.sin()) * MOVE_SPEED;
        self.pos += vel;
        self.pos.x = self.pos.x.rem_euclid(GRID_WIDTH as f32);
        self.pos.y = self.pos.y.rem_euclid(GRID_HEIGHT as f32);
    }
}

#[macroquad::main("Slime Train")]
async fn main() {
    let mut grid = ChemoGrid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut agents: Vec<Agent> = (0..2000)
        .map(|_| Agent::new(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0))
        .collect();

    // Add some cities
    let cities = vec![
        (100, 100),
        (300, 200),
        (200, 50),
        (50, 250),
        (350, 50),
        (150, 250),
    ];

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        // Update Agents
        for agent in agents.iter_mut() {
            let sensors = agent.sense(&grid);
            agent.rotate(sensors);
            agent.move_forward();
            grid.deposit(agent.pos.x as usize, agent.pos.y as usize, 0.5);
        }

        // Update Grid
        grid.update();

        // City attraction (simplistic: just high pheromone)
        for (cx, cy) in &cities {
             grid.deposit(*cx, *cy, 1.0);
        }


        // Render to texture (optimized direct byte access)
        for (i, cell) in grid.cells.iter().enumerate() {
            let val = *cell;
            let (r, g, b) = if val > 0.01 {
                // Color based on intensity: Blue -> Green -> Yellow -> White
                let r = (val * 2.0 - 1.0).max(0.0);
                let g = val.min(1.0);
                let b = (1.0 - val * 2.0).max(0.0) * 0.5;
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            } else {
                (0, 0, 0)
            };

            let idx = i * 4;
            image.bytes[idx] = r;
            image.bytes[idx + 1] = g;
            image.bytes[idx + 2] = b;
            image.bytes[idx + 3] = 255;
        }

        // Draw cities on top of texture
        for (cx, cy) in &cities {
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let px = (*cx as isize + dx).clamp(0, GRID_WIDTH as isize - 1) as usize;
                    let py = (*cy as isize + dy).clamp(0, GRID_HEIGHT as isize - 1) as usize;
                    let idx = (py * GRID_WIDTH + px) * 4;
                    image.bytes[idx] = 255;
                    image.bytes[idx + 1] = 255;
                    image.bytes[idx + 2] = 255;
                    image.bytes[idx + 3] = 255;
                }
            }
        }

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        draw_text("Slime Train: Mycelial Transit Network", 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_hypha_follows_gradient() {
        let mut grid = ChemoGrid::new(GRID_WIDTH, GRID_HEIGHT);
        let mut agent = Agent {
            pos: vec2(100.0, 100.0),
            angle: 0.0, // Facing East
        };

        // Place food to the South (Right of the agent)
        // Sensor Right is at angle + PI/4
        // Agent is at 100, 100 facing 0.
        // Left sensor: -PI/4 (North East)
        // Center sensor: 0 (East)
        // Right sensor: +PI/4 (South East)

        // Let's put a strong signal where the Right sensor would fall
        let r_sensor_pos = agent.get_sensor_pos(agent.angle + SENSOR_ANGLE);
        grid.deposit(r_sensor_pos.x as usize, r_sensor_pos.y as usize, 1.0);

        // Sense
        let sensors = agent.sense(&grid);
        let (l, c, r) = sensors;

        // Verify sensors picked it up
        assert!(r > c && r > l, "Right sensor should detect highest concentration");

        // Rotate
        let initial_angle = agent.angle;
        agent.rotate(sensors);

        // Should have turned Right (positive angle increase)
        assert!(agent.angle > initial_angle, "Agent should turn towards the right (South-East)");
    }
}
