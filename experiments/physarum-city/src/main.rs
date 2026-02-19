use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_AGENTS: usize = 50000;
const SENSOR_ANGLE: f32 = std::f32::consts::PI / 4.0;
const SENSOR_DIST: f32 = 9.0;
const TURN_SPEED: f32 = 0.2; // Radians per tick
const MOVE_SPEED: f32 = 1.0;
const DECAY_RATE: f32 = 0.95;
const DIFFUSE_RATE: f32 = 0.9;
const DEPOSIT_AMOUNT: f32 = 1.0;

#[derive(Clone, Copy)]
struct Agent {
    pos: Vec2,
    angle: f32, // Radians
    species: u8, // For future use (multiple colors)
}

struct World {
    trail_map: Vec<f32>,
    trail_map_next: Vec<f32>, // Double buffer
    width: usize,
    height: usize,
    food_map: Vec<Vec2>, // List of food positions
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        Self {
            trail_map: vec![0.0; width * height],
            trail_map_next: vec![0.0; width * height],
            width,
            height,
            food_map: Vec::new(),
        }
    }

    fn get_trail(&self, x: f32, y: f32) -> f32 {
        let ix = x.round() as i32;
        let iy = y.round() as i32;

        // Wrap around logic
        let w = self.width as i32;
        let h = self.height as i32;

        let ix = (ix % w + w) % w;
        let iy = (iy % h + h) % h;

        self.trail_map[(iy as usize) * self.width + (ix as usize)]
    }

    fn add_trail(&mut self, x: f32, y: f32, amount: f32) {
        let ix = x.round() as i32;
        let iy = y.round() as i32;

        let w = self.width as i32;
        let h = self.height as i32;

        let ix = (ix % w + w) % w;
        let iy = (iy % h + h) % h;

        let idx = (iy as usize) * self.width + (ix as usize);
        self.trail_map[idx] = (self.trail_map[idx] + amount).min(10.0); // Clamp max
    }
}

impl Agent {
    fn new(pos: Vec2, angle: f32) -> Self {
        Self { pos, angle, species: 0 }
    }

    fn update(&mut self, world: &World) {
        let mut rng = ::rand::thread_rng();

        // Sensory stage
        // Right is +angle (Clockwise in screen coords)
        let right_angle = self.angle + SENSOR_ANGLE;
        // Left is -angle (Counter-Clockwise)
        let left_angle = self.angle - SENSOR_ANGLE;
        let front_angle = self.angle;

        let sensor_r_pos = self.pos + Vec2::new(right_angle.cos(), right_angle.sin()) * SENSOR_DIST;
        let sensor_l_pos = self.pos + Vec2::new(left_angle.cos(), left_angle.sin()) * SENSOR_DIST;
        let sensor_f_pos = self.pos + Vec2::new(front_angle.cos(), front_angle.sin()) * SENSOR_DIST;

        let v_r = world.get_trail(sensor_r_pos.x, sensor_r_pos.y);
        let v_l = world.get_trail(sensor_l_pos.x, sensor_l_pos.y);
        let v_f = world.get_trail(sensor_f_pos.x, sensor_f_pos.y);

        // Turn logic
        if v_f > v_l && v_f > v_r {
            // Keep direction, maybe wiggle slightly
        } else if v_f < v_l && v_f < v_r {
            // Random turn
            let turn = (rng.gen::<f32>() - 0.5) * 2.0 * TURN_SPEED;
            self.angle += turn;
        } else if v_l > v_r {
            // Turn Left (-angle)
            self.angle -= TURN_SPEED;
        } else if v_r > v_l {
            // Turn Right (+angle)
            self.angle += TURN_SPEED;
        }

        // Move
        let move_vec = Vec2::new(self.angle.cos(), self.angle.sin()) * MOVE_SPEED;
        self.pos += move_vec;

        // Wrap
        let w = world.width as f32;
        let h = world.height as f32;
        self.pos.x = (self.pos.x % w + w) % w;
        self.pos.y = (self.pos.y % h + h) % h;
    }
}

fn diffuse_and_decay(world: &mut World) {
    // Zero-allocation implementation with double buffering
    let w = world.width;
    let h = world.height;

    // We can't borrow trail_map and trail_map_next mutably from World at same time easily due to borrow checker
    // But we can destructure or use split_at_mut if they were in same Vec (they are not).
    // Or we can use unsafe, or just indices if we iterate over range and access via slice?
    // Wait, Rayon needs references.
    // Easiest way:
    // Take `trail_map` as read-only slice.
    // Take `trail_map_next` as mutable slice.
    // This requires passing them explicitly or splitting the borrow.

    let (src, dest) = (&world.trail_map, &mut world.trail_map_next);

    // Use Rayon zip/enumerate/chunks?
    // Dest is mutable. Src is immutable.
    // We can iterate over dest mutable chunks (rows?) and compute from src.

    dest.par_iter_mut().enumerate().for_each(|(idx, val)| {
        let x = idx % w;
        let y = idx / w;
        let mut sum = 0.0;

        // 3x3 kernel
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = ((x as i32 + dx + w as i32) % w as i32) as usize;
                let ny = ((y as i32 + dy + h as i32) % h as i32) as usize;
                sum += src[ny * w + nx];
            }
        }

        let avg = sum / 9.0;
        *val = avg * DIFFUSE_RATE * DECAY_RATE;
    });

    // Swap buffers
    std::mem::swap(&mut world.trail_map, &mut world.trail_map_next);
}

#[macroquad::main("Physarum City")]
async fn main() {
    let mut world = World::new(WIDTH, HEIGHT);
    let mut agents: Vec<Agent> = Vec::with_capacity(NUM_AGENTS);
    let mut rng = ::rand::thread_rng();

    // Initialize agents randomly (circle or random)
    for _ in 0..NUM_AGENTS {
        let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
        // Random position
        let pos = vec2(rng.gen::<f32>() * WIDTH as f32, rng.gen::<f32>() * HEIGHT as f32);

        agents.push(Agent::new(pos, angle));
    }

    // Add some initial food/cities
    world.food_map.push(vec2(WIDTH as f32 * 0.2, HEIGHT as f32 * 0.2));
    world.food_map.push(vec2(WIDTH as f32 * 0.8, HEIGHT as f32 * 0.8));
    world.food_map.push(vec2(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5));

    // Texture for trails
    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        // Input: Add food
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            world.food_map.push(vec2(mx, my));
        }

        // Add food pheromones (continuous)
        let foods = world.food_map.clone();
        for food in foods {
            // Add a blob of pheromone around food
            // Radius 5
            for dy in -5..=5 {
                for dx in -5..=5 {
                    if dx*dx + dy*dy <= 25 {
                        world.add_trail(food.x + dx as f32, food.y + dy as f32, 2.0);
                    }
                }
            }
        }

        // Update agents
        agents.par_iter_mut().for_each(|agent| {
            agent.update(&world);
        });

        // Sequential deposit
        for agent in &agents {
            world.add_trail(agent.pos.x, agent.pos.y, DEPOSIT_AMOUNT);
        }

        // Diffuse and Decay
        diffuse_and_decay(&mut world);

        // Update texture
        // Zero-allocation: Write directly to image bytes via parallel iterator
        // Image bytes are [R, G, B, A, R, G, B, A, ...]
        // We process chunks of 4.

        let trail_map = &world.trail_map;
        image.bytes.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            let val = trail_map[i];
            let brightness = (val * 255.0).min(255.0) as u8;
            // Color: Cyan/Green biological look
            pixel[0] = 0;           // R
            pixel[1] = brightness;  // G
            pixel[2] = brightness / 2; // B
            pixel[3] = 255;         // A
        });

        texture.update(&image);

        // Draw
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Draw food markers
        for food in &world.food_map {
            draw_circle(food.x, food.y, 5.0, RED);
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 30.0, WHITE);
        draw_text(format!("Agents: {}", NUM_AGENTS).as_str(), 10.0, 50.0, 30.0, WHITE);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_steering() {
        let width = 100;
        let height = 100;
        let mut world = World::new(width, height);

        // Agent at (50, 50), facing 0 (Right)
        let mut agent = Agent::new(vec2(50.0, 50.0), 0.0);

        // Sensor Right pos: (50, 50) + (cos(45)*9, sin(45)*9) ~ (56.36, 56.36)
        // Sensor Left pos: (50, 50) + (cos(-45)*9, sin(-45)*9) ~ (56.36, 43.64)

        // Place trail at Right sensor location
        let target_x = 56;
        let target_y = 56; // High Y is "Right" in terms of rotation from 0 to 90 deg
        world.add_trail(target_x as f32, target_y as f32, 10.0);

        agent.update(&world);

        // Expect agent to turn Right (Positive Angle)
        assert!(agent.angle > 0.0, "Agent should turn right (positive angle) towards pheromone. Angle: {}", agent.angle);

        // Reset agent
        agent.pos = vec2(50.0, 50.0);
        agent.angle = 0.0;

        // Place trail at Left sensor location
        world = World::new(width, height); // Clear world
        let target_x_l = 56;
        let target_y_l = 44;
        world.add_trail(target_x_l as f32, target_y_l as f32, 10.0);

        agent.update(&world);

        // Expect agent to turn Left (Negative Angle)
        assert!(agent.angle < 0.0, "Agent should turn left (negative angle) towards pheromone. Angle: {}", agent.angle);
    }
}
