pub mod dungeon;
pub mod render;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
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

use dungeon::Dungeon;
use poincare_disk::{neighbor_transform_a, mobius_add, Mobius, Point, TilingConsts};
use render::draw_dungeon;

struct App {
    dungeon: Dungeon,
    player_path: Vec<usize>,
    player_offset: Point, // Position relative to center of current tile
    tiling_consts: TilingConsts,
    last_tick: Instant,
    message: String,
}

impl App {
    fn new() -> Self {
        Self {
            dungeon: Dungeon::new(12345),
            player_path: Vec::new(),
            player_offset: Point::new(0.0, 0.0),
            tiling_consts: TilingConsts::new_4_5(),
            last_tick: Instant::now(),
            message: "Welcome to the Hyperbolic Dungeon!".to_string(),
        }
    }

    fn on_tick(&mut self) {
        // Continuous updates if needed
    }

    fn move_player(&mut self, dx: f64, dy: f64) {
        // Move player in the local frame
        // Just adding dx, dy is Euclidean approximation, valid for small steps near origin.
        // Better: Apply a small Mobius translation.
        // Let delta = dx + i*dy.
        // Transform: P_new = (P + delta)/(1 + conj(delta)P) ?
        // This is translation by delta.
        // Yes, if we consider input as "Move in direction delta".
        let delta = Point::new(dx, dy);
        // Limit speed to avoid jumping too far
        if delta.norm() > 0.2 {
            return;
        }

        let candidate_offset = mobius_add(self.player_offset, delta);

        // Check for tile transition
        // Find closest cell center among current (0) and neighbors.
        let mut best_neighbor = None;
        let mut best_dist_sq = candidate_offset.norm_sqr(); // Distance to current center (0)

        for i in 0..4 {
            let neighbor_pos = neighbor_transform_a(i, &self.tiling_consts);
            // Transform candidate to neighbor frame
            let t = Mobius::inverse_translation(neighbor_pos);
            let p_in_neighbor = t.apply(candidate_offset);

            let dist_sq = p_in_neighbor.norm_sqr();
            if dist_sq < best_dist_sq {
                best_dist_sq = dist_sq;
                best_neighbor = Some((i, p_in_neighbor));
            }
        }

        if let Some((idx, new_pos)) = best_neighbor {
            // Transition implied. Check if target is walkable.
            let next_path = Dungeon::canonicalize_step(self.player_path.clone(), idx);
            let tile = self.dungeon.get_tile(&next_path);

            if matches!(tile.tile_type, dungeon::TileType::Wall) {
                // Blocked!
                self.message = "Blocked by Wall!".to_string();
                // Do not update position
                return;
            }

            // Transition allowed
            self.player_path = next_path;
            self.player_offset = new_pos;

            // Update message
            let tile = self.dungeon.get_tile(&self.player_path);
            self.message = format!("Entered {:?} (Seed: {})", tile.tile_type, tile.color_seed);
            self.dungeon.mark_visited(&self.player_path);
        } else {
            // No transition, just move
            self.player_offset = candidate_offset;
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
                    KeyCode::Left | KeyCode::Char('a') => app.move_player(-0.05, 0.0),
                    KeyCode::Right | KeyCode::Char('d') => app.move_player(0.05, 0.0),
                    KeyCode::Up | KeyCode::Char('w') => app.move_player(0.0, 0.05),
                    KeyCode::Down | KeyCode::Char('s') => app.move_player(0.0, -0.05),
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= Duration::from_millis(16) {
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
            Span::styled("Hyperbolic Dungeon", Style::default().fg(Color::Cyan)),
            Span::raw(format!(
                " | Path: {:?} | Pos: {:.2}, {:.2}",
                app.player_path, app.player_offset.re, app.player_offset.im
            )),
        ]),
        Line::from(vec![Span::raw(format!("Message: {}", app.message))]),
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
            // Draw boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            // To render the view such that the player is at the center of the screen:
            // The player is at 'player_offset' in the 'current_tile' frame.
            // We want to map 'player_offset' to (0,0).
            // The transform T that does this is inverse_translation(player_offset).
            // This T maps "Current Tile Frame" to "Screen Frame".

            let view_transform = Mobius::inverse_translation(app.player_offset);

            draw_dungeon(
                ctx,
                &app.dungeon,
                &app.player_path,
                &view_transform,
                &app.tiling_consts,
            );

            // Draw Player (at center)
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 0.02,
                color: Color::Yellow,
            });
        });

    f.render_widget(canvas, canvas_area);
}
