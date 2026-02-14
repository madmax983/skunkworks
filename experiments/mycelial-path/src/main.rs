use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 150;
const CELL_SIZE: f32 = 4.0;

// ----------------------------------------------------------------------------
// Substrate
// ----------------------------------------------------------------------------

struct Substrate {
    width: usize,
    height: usize,
    density: Vec<f32>,
}

impl Substrate {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let mut density = vec![0.0; width * height];

        for y in 0..height {
            for x in 0..width {
                let fx = x as f32;
                let fy = y as f32;

                let v1 = (fx * 0.05).sin();
                let v2 = (fy * 0.05).cos();
                let v3 = ((fx + fy) * 0.02).sin();
                let v4: f32 = rng.gen();

                let val = (v1 + v2 + v3 * 0.5 + v4 * 0.2) / 2.7;
                let normalized = (val + 1.0) / 2.0;

                density[y * width + x] = normalized.clamp(0.0, 1.0);
            }
        }

        Self {
            width,
            height,
            density,
        }
    }

    fn get_density(&self, pos: IVec2) -> f32 {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return 1.0;
        }
        self.density[(pos.y as usize) * self.width + (pos.x as usize)]
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let d = self.density[idx];

                let brightness = 0.05 + (1.0 - d) * 0.15;
                let color = Color::new(brightness, brightness * 0.8, brightness * 0.6, 1.0);

                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Pathfinding Structures
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
struct Node {
    pos: IVec2,
    f_score: f32, // g + h
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for Min-Heap behavior in BinaryHeap
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct HyphaeNetwork {
    open_set: BinaryHeap<Node>,
    came_from: Vec<Option<IVec2>>,
    g_score: Vec<f32>,
    active_tips: Vec<IVec2>,
    width: usize,
    height: usize,
    target: Option<IVec2>,
    found: bool,
    start: IVec2,
}

impl HyphaeNetwork {
    fn new(width: usize, height: usize, start: IVec2) -> Self {
        let mut open_set = BinaryHeap::new();
        let came_from = vec![None; width * height];
        let mut g_score = vec![f32::INFINITY; width * height];

        // Initialize start
        let start_idx = (start.y as usize) * width + (start.x as usize);
        g_score[start_idx] = 0.0;
        open_set.push(Node {
            pos: start,
            f_score: 0.0,
        });

        Self {
            open_set,
            came_from,
            g_score,
            active_tips: Vec::new(),
            width,
            height,
            target: None,
            found: false,
            start,
        }
    }

    fn set_target(&mut self, target: IVec2) {
        self.target = Some(target);
        // Reset search but keep start
        self.found = false;
        self.open_set.clear();
        self.came_from.fill(None);
        self.g_score.fill(f32::INFINITY);

        let start_idx = (self.start.y as usize) * self.width + (self.start.x as usize);
        self.g_score[start_idx] = 0.0;

        // Initial heuristic
        let h = ((self.start.x - target.x).abs() + (self.start.y - target.y).abs()) as f32;
        self.open_set.push(Node {
            pos: self.start,
            f_score: h,
        });
    }

    fn update(&mut self, substrate: &Substrate, steps: usize) {
        if self.found {
            return;
        }
        if self.target.is_none() {
            return;
        }

        let target = self.target.unwrap();

        self.active_tips.clear();

        for _ in 0..steps {
            if let Some(current) = self.open_set.pop() {
                self.active_tips.push(current.pos);

                if current.pos == target {
                    self.found = true;
                    return;
                }

                // Neighbors (8-way)
                let neighbors = [
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(1, 1),
                    IVec2::new(1, -1),
                    IVec2::new(-1, 1),
                    IVec2::new(-1, -1),
                ];

                for &offset in &neighbors {
                    let neighbor = current.pos + offset;

                    if neighbor.x < 0
                        || neighbor.y < 0
                        || neighbor.x >= self.width as i32
                        || neighbor.y >= self.height as i32
                    {
                        continue;
                    }

                    let idx = (neighbor.y as usize) * self.width + (neighbor.x as usize);
                    let curr_idx = (current.pos.y as usize) * self.width + (current.pos.x as usize);

                    // Cost function: Distance * Density Penalty
                    let dist = if offset.x != 0 && offset.y != 0 {
                        1.414
                    } else {
                        1.0
                    };
                    let density = substrate.get_density(neighbor);
                    // Higher density = higher cost.
                    let move_cost = dist * (1.0 + density * 20.0); // Increased penalty to make it avoid rocks more

                    let tentative_g = self.g_score[curr_idx] + move_cost;

                    if tentative_g < self.g_score[idx] {
                        self.came_from[idx] = Some(current.pos);
                        self.g_score[idx] = tentative_g;

                        // Heuristic: Euclidean * weight
                        // A higher weight (e.g. 2.0 or 5.0) makes it greedier (faster, but maybe not optimal path)
                        // A weight of 1.0 is standard A* (guarantees shortest path)
                        // Let's use 1.2 for a slight greediness to look more organic
                        let h = neighbor.as_vec2().distance(target.as_vec2());

                        self.open_set.push(Node {
                            pos: neighbor,
                            f_score: tentative_g + h * 1.5,
                        });
                    }
                }
            } else {
                break;
            }
        }
    }

    fn draw(&self) {
        // Draw established paths (Hyphae)
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if let Some(parent) = self.came_from[idx] {
                    let p1 = Vec2::new(parent.x as f32, parent.y as f32) * CELL_SIZE
                        + Vec2::splat(CELL_SIZE / 2.0);
                    let p2 =
                        Vec2::new(x as f32, y as f32) * CELL_SIZE + Vec2::splat(CELL_SIZE / 2.0);

                    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, Color::new(1.0, 1.0, 1.0, 0.2));
                }
            }
        }

        // Draw active tips
        for tip in &self.active_tips {
            draw_rectangle(
                tip.x as f32 * CELL_SIZE,
                tip.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                GREEN,
            );
        }

        // Draw Path if found
        if self.found {
            if let Some(target) = self.target {
                let mut curr = target;
                while let Some(parent) =
                    self.came_from[(curr.y as usize) * self.width + (curr.x as usize)]
                {
                    let p1 = Vec2::new(parent.x as f32, parent.y as f32) * CELL_SIZE
                        + Vec2::splat(CELL_SIZE / 2.0);
                    let p2 = Vec2::new(curr.x as f32, curr.y as f32) * CELL_SIZE
                        + Vec2::splat(CELL_SIZE / 2.0);

                    // Thicker, pulsating artery
                    let time = get_time() as f32;
                    let thickness = 2.0 + (time * 5.0).sin() * 0.5;
                    let pulse_color = Color::new(1.0, 0.2, 0.2, 0.8);

                    draw_line(p1.x, p1.y, p2.x, p2.y, thickness, pulse_color);
                    curr = parent;
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Main
// ----------------------------------------------------------------------------

#[macroquad::main("Mycelial Path")]
async fn main() {
    let mut substrate = Substrate::new(GRID_WIDTH, GRID_HEIGHT);
    let start = IVec2::new(GRID_WIDTH as i32 / 2, GRID_HEIGHT as i32 / 2);
    let mut fungus = HyphaeNetwork::new(GRID_WIDTH, GRID_HEIGHT, start);

    // Set a random target initially
    let mut rng = thread_rng();
    let mut target = IVec2::new(
        rng.gen_range(10..GRID_WIDTH as i32 - 10),
        rng.gen_range(10..GRID_HEIGHT as i32 - 10),
    );
    fungus.set_target(target);

    loop {
        // Input Handling
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let grid_x = (mpos.0 / CELL_SIZE) as i32;
            let grid_y = (mpos.1 / CELL_SIZE) as i32;

            if grid_x >= 0
                && grid_x < GRID_WIDTH as i32
                && grid_y >= 0
                && grid_y < GRID_HEIGHT as i32
            {
                target = IVec2::new(grid_x, grid_y);
                fungus.set_target(target);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            substrate = Substrate::new(GRID_WIDTH, GRID_HEIGHT);
            let mut rng = thread_rng();
            target = IVec2::new(
                rng.gen_range(10..GRID_WIDTH as i32 - 10),
                rng.gen_range(10..GRID_HEIGHT as i32 - 10),
            );
            fungus.set_target(target);
        }

        clear_background(BLACK);

        // Update
        fungus.update(&substrate, 100); // Speed up even more

        // Draw
        substrate.draw();
        fungus.draw();

        // Draw target
        draw_circle(
            target.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            target.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            CELL_SIZE * 2.0,
            YELLOW,
        );

        // UI
        draw_text("Mycelial Path", 20.0, 30.0, 30.0, WHITE);
        draw_text("Left Click: Set Food Target", 20.0, 50.0, 20.0, WHITE);
        draw_text("Space: New Soil", 20.0, 70.0, 20.0, WHITE);

        next_frame().await
    }
}
