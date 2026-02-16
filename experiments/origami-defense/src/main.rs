use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

mod geo;
mod model;
use geo::MiuraPattern;
use model::{World, Tower, Enemy, Projectile, ROWS, COLS};
use nalgebra::{Point3, Rotation3, Vector3};

struct App {
    world: World,
    rotation: (f64, f64), // (pitch, yaw)
    cursor: (usize, usize),
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(),
            rotation: (0.5, 0.5),
            cursor: (ROWS / 2, COLS / 2),
            should_quit: false,
        }
    }

    fn update(&mut self) {
        // Physics: Smooth Rho Transition
        let diff = self.world.target_rho - self.world.rho;
        self.world.rho += diff * 0.1;

        // Income Logic: More income when FLAT (rho near 1.0)
        // Base income per tick
        if self.world.ticks % 10 == 0 {
            let income = 0.5 * self.world.rho;
            self.world.resources += income;
        }

        // Compute current vertices for collision checks
        let vertices = self.world.pattern.compute_vertices(self.world.rho);

        // Spawn Enemies
        self.world.spawn_timer += 1;
        let spawn_rate = (100 - self.world.wave * 5).max(20);
        if self.world.spawn_timer >= spawn_rate {
            self.world.spawn_timer = 0;
            self.world.enemies.push(Enemy {
                r: 0,
                c: 0,
                hp: 10.0 + (self.world.wave as f64 * 2.0),
                id: self.world.ticks,
                move_timer: 0,
                max_hp: 10.0 + (self.world.wave as f64 * 2.0),
            });
        }

        // Wave progression
        if self.world.ticks % 1000 == 0 {
            self.world.wave += 1;
        }

        // Move Enemies
        let move_delay = 10; // Ticks per step
        let mut reached_end_indices = Vec::new();

        for (i, enemy) in self.world.enemies.iter_mut().enumerate() {
            enemy.move_timer += 1;
            if enemy.move_timer >= move_delay {
                enemy.move_timer = 0;
                // Pathfinding: Go to (ROWS-1, COLS-1)
                // Prefer changing C (moving right) then R (moving down)
                if enemy.c < COLS - 2 {
                    enemy.c += 1;
                } else if enemy.r < ROWS - 2 {
                    enemy.r += 1;
                } else {
                    // Reached end
                    reached_end_indices.push(i);
                }
            }
        }

        // Remove enemies that reached the end (Damage Player?)
        for i in reached_end_indices.iter().rev() {
            self.world.resources -= 10.0; // Penalty
            self.world.enemies.remove(*i);
        }

        // Update Towers
        for tower in &mut self.world.towers {
            if tower.cooldown > 0 {
                tower.cooldown -= 1;
            }

            // Find nearest enemy in 3D SPACE
            let tower_pos = self.world.pattern.get_face_center(&vertices, tower.r, tower.c).unwrap_or(Point3::origin());

            let mut best_dist = f64::MAX;
            let mut best_enemy_idx = None;

            for (idx, enemy) in self.world.enemies.iter().enumerate() {
                if let Some(enemy_pos) = self.world.pattern.get_face_center(&vertices, enemy.r, enemy.c) {
                    let dist = (tower_pos - enemy_pos).norm();
                    if dist < best_dist {
                        best_dist = dist;
                        best_enemy_idx = Some(idx);
                    }
                }
            }

            // Range check (Range is roughly 2.0 * a * range_mult)
            let range = 15.0 * tower.range_mult;
            let sensor_info = if best_dist < range {
                Some((best_dist, 0.0)) // Angle not used yet
            } else {
                None
            };

            // Tower Logic (VM)
            if let Some(should_fire) = tower.tick(sensor_info) {
                if should_fire && tower.cooldown == 0 {
                    if let Some(idx) = best_enemy_idx {
                        // Fire projectile
                        // Calculate lead? No, homing for now or instant hit?
                        // Let's spawn a projectile at tower pos aimed at enemy
                        let target_pos = self.world.pattern.get_face_center(&vertices, self.world.enemies[idx].r, self.world.enemies[idx].c).unwrap();
                        let dir = (target_pos - tower_pos).normalize();

                        self.world.projectiles.push(Projectile {
                            pos: tower_pos,
                            velocity: dir * 2.0, // Speed
                            life: 20,
                        });
                        tower.cooldown = 10;
                    }
                }
            }
        }

        // Update Projectiles
        let mut hit_indices = Vec::new();
        let mut dead_proj_indices = Vec::new();

        for (p_idx, proj) in self.world.projectiles.iter_mut().enumerate() {
            proj.pos += proj.velocity;
            proj.life -= 1;

            if proj.life == 0 {
                dead_proj_indices.push(p_idx);
                continue;
            }

            // Check collision with enemies (in 3D!)
            for (e_idx, enemy) in self.world.enemies.iter().enumerate() {
                if let Some(enemy_pos) = self.world.pattern.get_face_center(&vertices, enemy.r, enemy.c) {
                    let dist = (proj.pos - enemy_pos).norm();
                    if dist < 3.0 { // Hit radius
                        hit_indices.push(e_idx);
                        dead_proj_indices.push(p_idx);
                        break; // Only hit one enemy
                    }
                }
            }
        }

        // Remove dead projectiles
        // Sort and dedup to avoid index issues if logic changes
        dead_proj_indices.sort_unstable();
        dead_proj_indices.dedup();
        for i in dead_proj_indices.iter().rev() {
            if *i < self.world.projectiles.len() {
                self.world.projectiles.remove(*i);
            }
        }

        // Damage enemies
        hit_indices.sort_unstable();
        hit_indices.dedup();
        let mut dead_enemies = Vec::new();
        for i in hit_indices {
            if i < self.world.enemies.len() {
                self.world.enemies[i].hp -= 5.0;
                if self.world.enemies[i].hp <= 0.0 {
                    self.world.resources += 2.0; // Kill reward
                    dead_enemies.push(i);
                }
            }
        }

        // Remove dead enemies
        dead_enemies.sort_unstable();
        dead_enemies.dedup();
        for i in dead_enemies.iter().rev() {
             if *i < self.world.enemies.len() {
                self.world.enemies.remove(*i);
            }
        }

        self.world.ticks += 1;
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Left => {
                    self.world.target_rho = (self.world.target_rho - 0.1).clamp(0.0, 1.0)
                }
                KeyCode::Right => {
                    self.world.target_rho = (self.world.target_rho + 0.1).clamp(0.0, 1.0)
                }
                KeyCode::Char('w') => self.cursor.0 = self.cursor.0.saturating_sub(1),
                KeyCode::Char('s') => self.cursor.0 = (self.cursor.0 + 1).min(ROWS - 2),
                KeyCode::Char('a') => self.cursor.1 = self.cursor.1.saturating_sub(1),
                KeyCode::Char('d') => self.cursor.1 = (self.cursor.1 + 1).min(COLS - 2),
                KeyCode::Char(' ') => {
                    self.world.add_tower(self.cursor.0, self.cursor.1);
                }
                // Rotation
                KeyCode::Char('i') => self.rotation.0 -= 0.1,
                KeyCode::Char('k') => self.rotation.0 += 0.1,
                KeyCode::Char('j') => self.rotation.1 -= 0.1,
                KeyCode::Char('l') => self.rotation.1 += 0.1,
                _ => {}
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    let canvas_area = chunks[0];

    // Compute vertices once for drawing
    let vertices = app.world.pattern.compute_vertices(app.world.rho);

    // Rotation Matrix
    let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), app.rotation.0)
        * Rotation3::from_axis_angle(&Vector3::y_axis(), app.rotation.1);

    // Project Logic
    let projected: Vec<(f64, f64)> = vertices
        .iter()
        .map(|v| {
            let rv = rot * v;
            (rv.x, rv.y)
        })
        .collect();

    // Bounds
    let (min_x, max_x, min_y, max_y) = projected.iter().fold(
        (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
        |(minx, maxx, miny, maxy), (x, y)| (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y)),
    );

    let width = (max_x - min_x).max(1.0) * 1.5;
    let height = (max_y - min_y).max(1.0) * 1.5;
    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;

    let x_bounds = [cx - width / 2.0, cx + width / 2.0];
    let y_bounds = [cy - height / 2.0, cy + height / 2.0];

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Origami Defense "))
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            // Draw Grid Lines
             let rows = app.world.pattern.rows;
            let cols = app.world.pattern.cols;

            for r in 0..rows {
                for c in 0..cols {
                    let idx = r * cols + c;
                    let (x1, y1) = projected[idx];

                    if c + 1 < cols {
                        let (x2, y2) = projected[idx + 1];
                        ctx.draw(&CanvasLine { x1, y1, x2, y2, color: Color::DarkGray });
                    }
                    if r + 1 < rows {
                        let (x2, y2) = projected[idx + cols];
                        ctx.draw(&CanvasLine { x1, y1, x2, y2, color: Color::DarkGray });
                    }
                }
            }

            // Helper to get 2D center of face
            let get_center_2d = |r: usize, c: usize| -> Option<(f64, f64)> {
                let idxs = app.world.pattern.get_quad_indices(r, c)?;
                let mut cx = 0.0; let mut cy = 0.0;
                for &i in &idxs {
                    cx += projected[i].0;
                    cy += projected[i].1;
                }
                Some((cx/4.0, cy/4.0))
            };

            // Draw Towers
            for tower in &app.world.towers {
                if let Some((x, y)) = get_center_2d(tower.r, tower.c) {
                    ctx.print(x, y, "T".cyan().bold());
                }
            }

            // Draw Enemies
            for enemy in &app.world.enemies {
                if let Some((x, y)) = get_center_2d(enemy.r, enemy.c) {
                    ctx.print(x, y, "E".red().bold());
                }
            }

            // Draw Projectiles
            // Need to project projectile 3D pos
            for proj in &app.world.projectiles {
                let rv = rot * proj.pos;
                ctx.print(rv.x, rv.y, ".".yellow());
            }

            // Draw Cursor
             if let Some((x, y)) = get_center_2d(app.cursor.0, app.cursor.1) {
                ctx.print(x, y, "[ ]".yellow());
             }

             // Draw Base/End
             if let Some((x, y)) = get_center_2d(ROWS-2, COLS-2) {
                 ctx.print(x, y, "BASE".green());
             }
        });

    f.render_widget(canvas, canvas_area);

    // Status
    let status_text = format!(
        "Rho: {:.2} | Res: {:.1} | Wave: {} | En: {} | Twr: {} | [Arrows] Fold | [Space] Build | [WASD] Cursor",
        app.world.rho, app.world.resources, app.world.wave, app.world.enemies.len(), app.world.towers.len()
    );
    f.render_widget(
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            app.handle_event(event::read()?);
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
