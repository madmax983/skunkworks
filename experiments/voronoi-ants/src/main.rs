mod scan;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use scan::FileNode;
use std::time::{Duration, Instant};
use tui_shared::Tui;

// --- Physics & Simulation Types ---

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        let l = self.length();
        if l == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / l,
                y: self.y / l,
            }
        }
    }
}

#[derive(Clone)]
struct Ant {
    pos: Vec2,
    vel: Vec2,
    has_food: bool,
    color: Color,
}

struct FoodSource {
    pos: Vec2,
    amount: usize,
    #[allow(dead_code)]
    max_amount: usize,
}

struct PheromoneGrid {
    width: usize,
    height: usize,
    food_scent: Vec<f32>,
    home_scent: Vec<f32>,
}

impl PheromoneGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            food_scent: vec![0.0; width * height],
            home_scent: vec![0.0; width * height],
        }
    }

    fn get_index(&self, x: f64, y: f64) -> Option<usize> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let ix = x as usize;
        let iy = y as usize;
        if ix >= self.width || iy >= self.height {
            return None;
        }
        Some(iy * self.width + ix)
    }

    fn add_scent(&mut self, x: f64, y: f64, food: bool, amount: f32) {
        if let Some(idx) = self.get_index(x, y) {
            if food {
                self.food_scent[idx] = (self.food_scent[idx] + amount).min(100.0);
            } else {
                self.home_scent[idx] = (self.home_scent[idx] + amount).min(100.0);
            }
        }
    }

    fn sample_gradient(&self, x: f64, y: f64, food_scent: bool) -> Vec2 {
        let mut grad = Vec2::new(0.0, 0.0);
        let idx_c = match self.get_index(x, y) {
            Some(i) => i,
            None => return grad,
        };

        let grid = if food_scent {
            &self.food_scent
        } else {
            &self.home_scent
        };

        // Simple 4-neighbor gradient
        let c_val = grid[idx_c];

        if let Some(idx_r) = self.get_index(x + 1.0, y) {
            grad.x += grid[idx_r] as f64 - c_val as f64;
        }
        if let Some(idx_l) = self.get_index(x - 1.0, y) {
            grad.x -= grid[idx_l] as f64 - c_val as f64;
        }
        if let Some(idx_d) = self.get_index(x, y + 1.0) {
            grad.y += grid[idx_d] as f64 - c_val as f64;
        }
        if let Some(idx_u) = self.get_index(x, y - 1.0) {
            grad.y -= grid[idx_u] as f64 - c_val as f64;
        }

        grad
    }

    fn evaporate(&mut self) {
        for v in &mut self.food_scent {
            *v *= 0.98;
            if *v < 0.1 {
                *v = 0.0;
            }
        }
        for v in &mut self.home_scent {
            *v *= 0.98;
            if *v < 0.1 {
                *v = 0.0;
            }
        }
    }
}

struct World {
    width: f64,
    height: f64,
    ants: Vec<Ant>,
    foods: Vec<FoodSource>,
    pheromones: PheromoneGrid,
    total_collected: usize,
}

impl World {
    fn new(files: Vec<FileNode>, width: f64, height: f64) -> Self {
        let mut rng = rand::thread_rng();

        // Map files to food sources
        let mut foods = Vec::new();
        for file in files {
            if file.todo_count > 0 {
                foods.push(FoodSource {
                    pos: Vec2::new(
                        rng.gen_range(5.0..width - 5.0),
                        rng.gen_range(5.0..height - 5.0),
                    ),
                    amount: file.todo_count * 10, // amplify for gameplay
                    max_amount: file.todo_count * 10,
                });
            }
        }

        // Add random food if empty
        if foods.is_empty() {
            for _ in 0..5 {
                foods.push(FoodSource {
                    pos: Vec2::new(
                        rng.gen_range(5.0..width - 5.0),
                        rng.gen_range(5.0..height - 5.0),
                    ),
                    amount: 50,
                    max_amount: 50,
                });
            }
        }

        let ants = (0..60)
            .map(|_| {
                let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                Ant {
                    pos: Vec2::new(width / 2.0, height / 2.0),
                    vel: Vec2::new(angle.cos(), angle.sin()),
                    has_food: false,
                    color: Color::White,
                }
            })
            .collect();

        Self {
            width,
            height,
            ants,
            foods,
            pheromones: PheromoneGrid::new(width as usize, height as usize),
            total_collected: 0,
        }
    }

    fn update(&mut self) {
        let home_pos = Vec2::new(self.width / 2.0, self.height / 2.0);

        // Update Grid (add constant home scent at center)
        self.pheromones
            .add_scent(home_pos.x, home_pos.y, false, 50.0);

        // Update Ants
        let mut rng = rand::thread_rng();

        for ant in &mut self.ants {
            // 1. Behavior & Steering
            let mut desired_vel = ant.vel;

            if ant.has_food {
                // Seek Home
                ant.color = Color::Red; // Carrying food

                // Pheromone steering (Home Scent)
                let grad = self.pheromones.sample_gradient(ant.pos.x, ant.pos.y, false);
                if grad.length() > 0.1 {
                    desired_vel.x += grad.x * 2.0;
                    desired_vel.y += grad.y * 2.0;
                } else {
                    // Approximate direction to center if no scent
                    let dx = home_pos.x - ant.pos.x;
                    let dy = home_pos.y - ant.pos.y;
                    desired_vel.x += dx * 0.05;
                    desired_vel.y += dy * 0.05;
                }

                // Drop Food Pheromone
                self.pheromones.add_scent(ant.pos.x, ant.pos.y, true, 5.0);

                // Check Home Arrival
                let d = (ant.pos.x - home_pos.x).hypot(ant.pos.y - home_pos.y);
                if d < 2.0 {
                    ant.has_food = false;
                    self.total_collected += 1;
                    ant.vel.x *= -1.0;
                    ant.vel.y *= -1.0;
                }
            } else {
                // Seek Food
                ant.color = Color::Blue; // Scouting

                // Pheromone steering (Food Scent)
                let grad = self.pheromones.sample_gradient(ant.pos.x, ant.pos.y, true);
                if grad.length() > 0.1 {
                    desired_vel.x += grad.x * 2.0;
                    desired_vel.y += grad.y * 2.0;
                }

                // Drop Home Pheromone
                self.pheromones.add_scent(ant.pos.x, ant.pos.y, false, 2.0);

                // Check Food Arrival
                for food in &mut self.foods {
                    if food.amount > 0 {
                        let d = (ant.pos.x - food.pos.x).hypot(ant.pos.y - food.pos.y);
                        if d < 2.0 {
                            ant.has_food = true;
                            food.amount -= 1;
                            ant.vel.x *= -1.0;
                            ant.vel.y *= -1.0;
                            break;
                        }
                    }
                }
            }

            // Random wander
            desired_vel.x += rng.gen_range(-0.5..0.5);
            desired_vel.y += rng.gen_range(-0.5..0.5);

            // Normalize and Apply
            desired_vel = desired_vel.normalize();
            ant.vel = desired_vel;
            ant.pos.x += ant.vel.x * 1.0; // Speed
            ant.pos.y += ant.vel.y * 1.0;

            // Bounds (Bounce)
            if ant.pos.x <= 0.0 || ant.pos.x >= self.width {
                ant.vel.x *= -1.0;
                ant.pos.x = ant.pos.x.clamp(0.0, self.width - 0.1);
            }
            if ant.pos.y <= 0.0 || ant.pos.y >= self.height {
                ant.vel.y *= -1.0;
                ant.pos.y = ant.pos.y.clamp(0.0, self.height - 0.1);
            }
        }

        self.pheromones.evaporate();
    }
}

// --- TUI Logic ---

struct App {
    world: World,
    should_quit: bool,
}

impl App {
    fn new(files: Vec<FileNode>) -> Self {
        Self {
            world: World::new(files, 100.0, 100.0), // Init size, updated on render
            should_quit: false,
        }
    }
}

fn main() -> Result<()> {
    // 1. Scan
    let root = std::env::current_dir()?;
    let files = scan::scan_codebase(&root).unwrap_or_default();

    // 2. Init TUI
    let mut tui = Tui::init()?;
    let mut app = App::new(files);

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        app.should_quit = true;
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.world.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update World Size to match View
    let area = chunks[0];
    if area.width as f64 != app.world.width || area.height as f64 != app.world.height {
        // Resizing grid is expensive, so we just clamp boundaries or re-init?
        // For now, let's just update the bounds variables so ants bounce correctly.
        // (A real implementation would resize the grid vectors too)
        if area.width as usize != app.world.pheromones.width
            || area.height as usize != app.world.pheromones.height
        {
            app.world.pheromones = PheromoneGrid::new(area.width as usize, area.height as usize);
        }
        app.world.width = area.width as f64;
        app.world.height = area.height as f64;
    }

    let voronoi_widget = VoronoiWidget { world: &app.world };
    f.render_widget(voronoi_widget, area);

    let status_text = vec![Line::from(vec![
        " [Q] ".yellow().bold(),
        "Quit ".into(),
        format!(" | Collected: {} ", app.world.total_collected).into(),
        format!(" | Ants: {} ", app.world.ants.len()).into(),
        format!(" | Food Sources: {} ", app.world.foods.len()).into(),
    ])];

    let status = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}

struct VoronoiWidget<'a> {
    world: &'a World,
}

impl<'a> Widget for VoronoiWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Optimizing Nearest Neighbor Search:
        // Hoisting invariants to avoid repeated calculations in the inner loop.
        // Uses forward differencing for dx^2 and pre-calculated dy^2.

        // State: dx_sq, delta, dy_sq, color
        let mut row_state: Vec<(f64, f64, f64, Color)> = Vec::with_capacity(self.world.ants.len());

        for y in 0..area.height {
            let py = y as f64;

            // Pre-calculate Y-invariants and init X-state for this row
            row_state.clear();
            for ant in &self.world.ants {
                let dy = (py - ant.pos.y) * 2.0;
                let dy_sq = dy * dy;

                // Initial x=0 state
                // dx = 0.0 - ant.pos.x = -ant.pos.x
                // dx^2 = (-ant.pos.x)^2
                // delta = 2(0 - ant.pos.x) + 1 = 1 - 2*ant.pos.x
                let dx_sq = ant.pos.x * ant.pos.x;
                let delta = 1.0 - 2.0 * ant.pos.x;

                row_state.push((dx_sq, delta, dy_sq, ant.color));
            }

            for x in 0..area.width {
                // Voronoi Logic
                let mut min_dist_sq = f64::MAX;
                let mut nearest_color = Color::Reset;

                // We iterate mutably to update dx_sq and delta
                for state in &mut row_state {
                    let dist_sq = state.0 + state.2; // dx_sq + dy_sq

                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest_color = state.3;
                    }

                    // Forward differencing for next x:
                    // dx^2 += delta
                    // delta += 2.0
                    state.0 += state.1;
                    state.1 += 2.0;
                }

                // Draw Cell Background
                if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                    // Dim the color for background
                    // Since we can't easily "dim" ANSI colors programmatically without full RGB logic,
                    // we map Blue -> LightBlue etc or just use the color.
                    // Or maybe use a char to represent texture.

                    // Let's set bg to nearest color.
                    cell.set_bg(nearest_color);

                    // Check for Food Source
                    for food in &self.world.foods {
                        if food.amount > 0 {
                            let fx = food.pos.x.round() as u16;
                            let fy = food.pos.y.round() as u16;
                            if x == fx && y == fy {
                                cell.set_fg(Color::Green);
                                cell.set_char('☘');
                            }
                        }
                    }

                    // Check for Home
                    let hx = (self.world.width / 2.0).round() as u16;
                    let hy = (self.world.height / 2.0).round() as u16;
                    if x == hx && y == hy {
                        cell.set_fg(Color::Yellow);
                        cell.set_char('⌂');
                    }
                }
            }
        }

        // Draw Ants on top?
        // Actually Voronoi implies the region *belongs* to the ant.
        // So the ant is implicitly at the center of the cell (or seed point).
        // Let's draw the ant character to verify.
        for ant in &self.world.ants {
            let ax = ant.pos.x.round() as u16;
            let ay = ant.pos.y.round() as u16;
            if ax < area.width && ay < area.height {
                if let Some(cell) = buf.cell_mut((area.x + ax, area.y + ay)) {
                    cell.set_fg(Color::Black); // Contrast against BG
                    cell.set_char('●');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;
    use std::time::Instant;

    #[test]
    fn test_render_voronoi() {
        let ants = vec![
            Ant {
                pos: Vec2::new(2.0, 2.0),
                vel: Vec2::new(0.0, 0.0),
                has_food: false,
                color: Color::Red,
            },
            Ant {
                pos: Vec2::new(8.0, 2.0),
                vel: Vec2::new(0.0, 0.0),
                has_food: false,
                color: Color::Blue,
            },
        ];

        let world = World {
            width: 10.0,
            height: 10.0,
            ants,
            foods: vec![],
            pheromones: PheromoneGrid::new(10, 10),
            total_collected: 0,
        };

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let area = Rect::new(0, 0, 10, 10);

        // Verify correctness (small grid)
        let w = VoronoiWidget { world: &world };
        w.render(area, &mut buffer);

        // (2,2) -> Red
        let cell_2_2 = buffer.cell((2, 2)).unwrap();
        assert_eq!(cell_2_2.bg, Color::Red);

        // (8,2) -> Blue
        let cell_8_2 = buffer.cell((8, 2)).unwrap();
        assert_eq!(cell_8_2.bg, Color::Blue);

        // (5,2) -> Red (tie breaker first)
        let cell_5_2 = buffer.cell((5, 2)).unwrap();
        assert_eq!(cell_5_2.bg, Color::Red);

        // (6,2) -> Blue
        let cell_6_2 = buffer.cell((6, 2)).unwrap();
        assert_eq!(cell_6_2.bg, Color::Blue);

        // Benchmark (Large Grid)
        let large_area = Rect::new(0, 0, 100, 100);
        let mut large_buffer = Buffer::empty(large_area);
        // We reuse the same world (ants are at 2.0, 2.0 and 8.0, 2.0)

        let start = Instant::now();
        let iterations = 100;
        for _ in 0..iterations {
            let w = VoronoiWidget { world: &world };
            w.render(large_area, &mut large_buffer);
        }
        println!(
            "Time per {} iters (100x100): {:?}",
            iterations,
            start.elapsed()
        );
    }
}
