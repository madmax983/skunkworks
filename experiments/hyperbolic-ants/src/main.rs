mod ant;
mod dungeon;
mod geometry;
mod render;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use rand::rngs::StdRng;
use rand::SeedableRng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use ant::Ant;
use dungeon::{Dungeon, Path};
use geometry::{mobius_add, neighbor_transform_a, Mobius, Point, TilingConsts};

struct App {
    dungeon: Dungeon,
    ants: Vec<Ant>,
    camera_path: Path,
    camera_offset: Point,
    tiling_consts: TilingConsts,
    rng: StdRng,
    last_tick: Instant,
}

impl App {
    fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(42);
        let dungeon = Dungeon::new(12345);
        let consts = TilingConsts::new_4_5();

        let mut ants = Vec::new();
        for _ in 0..50 {
            ants.push(Ant::new(&mut rng));
        }

        Self {
            dungeon,
            ants,
            camera_path: Vec::new(),
            camera_offset: Point::new(0.0, 0.0),
            tiling_consts: consts,
            rng,
            last_tick: Instant::now(),
        }
    }

    fn on_tick(&mut self) {
        // Update Ants
        for ant in &mut self.ants {
            ant.update(&self.dungeon, &self.tiling_consts, &mut self.rng);
        }

        // Decay Pheromones
        self.dungeon.decay_pheromones();
    }

    fn move_camera(&mut self, dx: f64, dy: f64) {
        let delta = Point::new(dx, dy);
        if delta.norm() > 0.2 {
            return;
        }

        let candidate_offset = mobius_add(self.camera_offset, delta);

        // Check transition
        let mut best_neighbor = None;
        let mut best_dist_sq = candidate_offset.norm_sqr();

        for i in 0..4 {
            let neighbor_pos = neighbor_transform_a(i, &self.tiling_consts);
            let t = Mobius::inverse_translation(neighbor_pos);
            let p_in_neighbor = t.apply(candidate_offset);

            if p_in_neighbor.norm_sqr() < best_dist_sq {
                best_dist_sq = p_in_neighbor.norm_sqr();
                best_neighbor = Some((i, p_in_neighbor));
            }
        }

        if let Some((idx, new_pos)) = best_neighbor {
            let next_path = Dungeon::canonicalize_step(self.camera_path.clone(), idx);
            // Camera can move through walls (Spectator mode)
            self.camera_path = next_path;
            self.camera_offset = new_pos;
        } else {
            self.camera_offset = candidate_offset;
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Left | KeyCode::Char('a') => app.move_camera(-0.05, 0.0),
                    KeyCode::Right | KeyCode::Char('d') => app.move_camera(0.05, 0.0),
                    KeyCode::Up | KeyCode::Char('w') => app.move_camera(0.0, 0.05),
                    KeyCode::Down | KeyCode::Char('s') => app.move_camera(0.0, -0.05),
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= Duration::from_millis(50) {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }

    tui.exit()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    // Info
    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Hyperbolic Ants", Style::default().fg(Color::Green)),
            Span::raw(format!(
                " | Ants: {} | Path: {:?} | Pos: {:.2}, {:.2}",
                app.ants.len(),
                app.camera_path,
                app.camera_offset.re,
                app.camera_offset.im
            )),
        ]),
        Line::from(vec![Span::raw(
            "WASD/Arrows to move camera. Green=Pheromone, Red=Food.",
        )]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info, info_area);

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Poincaré Disk"),
        )
        .x_bounds([-1.05, 1.05])
        .y_bounds([-1.05, 1.05])
        .paint(|ctx| {
            // Boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            let view_transform = Mobius::inverse_translation(app.camera_offset);

            render::draw_dungeon(
                ctx,
                &app.dungeon,
                &app.camera_path,
                &view_transform,
                &app.tiling_consts,
                &app.ants,
            );

            // Draw Camera/Player indicator (center)
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 0.02,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, canvas_area);
}
