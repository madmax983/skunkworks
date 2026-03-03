mod physics;
mod platter;

use crate::physics::{Body, Universe};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use quipu::{Cord, Knot};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct AppState {
    universe: Universe,
    playhead_y: f32,   // -100 to 100 (Physical Y)
    playhead_dir: f32, // -1 (Down)
    triggered_bodies: HashSet<usize>,
    last_update: Instant,
    cords: Vec<Cord>, // Keep the data structure
}

impl AppState {
    fn new() -> Self {
        Self {
            universe: Universe::new(),
            playhead_y: 80.0,
            playhead_dir: -20.0, // Units per second
            triggered_bodies: HashSet::new(),
            last_update: Instant::now(),
            cords: Vec::new(),
        }
    }

    fn regenerate(&mut self) {
        self.universe = Universe::new();
        self.triggered_bodies.clear();
        self.cords.clear();
        self.playhead_y = 80.0;

        let num_cords = 5;
        let spacing = 160.0 / (num_cords as f32 + 1.0);
        let start_x = -80.0 + spacing;

        for i in 0..num_cords {
            let x = start_x + (i as f32 * spacing);
            let anchor_y = 80.0;

            // Create a Cord
            let mut cord = Cord::new();
            let clusters = rand::random::<u8>() % 5 + 3; // 3 to 7 clusters
            for _ in 0..clusters {
                let k_type = rand::random::<u8>() % 3;
                let knots = match k_type {
                    0 => vec![Knot::Simple],
                    1 => vec![Knot::Long(rand::random::<u8>() % 8 + 2)],
                    _ => vec![Knot::FigureEight],
                };
                cord.clusters.push(knots);
            }
            self.cords.push(cord.clone());

            // Physics Body Generation

            // Anchor
            let anchor = Body::new(x, anchor_y, 1000.0, 2.0, Color::White).fixed();
            let mut prev_id = self.universe.add_body(anchor);

            // Knots
            // Iterate reverse (High powers/Top to Low powers/Bottom) to build chain downwards
            let mut depth = 1;
            for (_c_idx, cluster) in cord.clusters.iter().enumerate().rev() {
                if cluster.is_empty() {
                    continue;
                }

                // Determine properties from Knot type
                let k = &cluster[0];
                let (mass, radius, color) = match k {
                    Knot::Simple => (1.0, 1.5, Color::Red),
                    Knot::Long(v) => (1.0 + (*v as f32 * 0.2), 2.5, Color::Yellow),
                    Knot::FigureEight => (1.5, 2.0, Color::Blue),
                };

                let y = anchor_y - (depth as f32 * 15.0); // Initial position
                depth += 1;

                let jitter = (rand::random::<f32>() - 0.5) * 5.0;

                let body = Body::new(x + jitter, y, mass, radius, color);
                let id = self.universe.add_body(body);

                self.universe.add_edge(prev_id, id);
                prev_id = id;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut state = AppState::new();
    state.regenerate();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(state.last_update).as_secs_f32();
        state.last_update = now;

        // Physics Step
        state.universe.step(dt);

        // Playhead Logic
        let _old_y = state.playhead_y;
        state.playhead_y += state.playhead_dir * dt;

        if state.playhead_y < -80.0 {
            state.playhead_y = 80.0; // Loop
            state.triggered_bodies.clear();
        }

        // Check Triggers
        for (id, body) in state.universe.bodies.iter().enumerate() {
            if body.fixed {
                continue;
            }

            let dist = (body.pos.y - state.playhead_y).abs();
            if dist < 2.0 && !state.triggered_bodies.contains(&id) {
                state.triggered_bodies.insert(id);
            }
        }

        // Render
        tui.terminal.draw(|f| {
            ui(f, &state);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => state.regenerate(),
                    KeyCode::Up => state.playhead_dir -= 5.0,
                    KeyCode::Down => state.playhead_dir += 5.0,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ferrous Quipu | Magnetic Cords"),
        )
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Draw Platter (Magnetism)
            let step = 4;
            for y in (0..state.universe.platter.height()).step_by(step) {
                for x in (0..state.universe.platter.width()).step_by(step) {
                    let mag = state.universe.platter.get_magnetism(x, y);
                    if mag > 0.2 {
                        let px = x as f64 - 100.0;
                        let py = y as f64 - 100.0;
                        let color = if mag > 0.5 {
                            Color::Red
                        } else {
                            Color::DarkGray
                        };
                        ctx.print(
                            px,
                            py,
                            ratatui::text::Span::styled(".", Style::default().fg(color)),
                        );
                    }
                }
            }

            // Draw Edges
            for &(i, j) in &state.universe.edges {
                let p1 = state.universe.bodies[i].pos;
                let p2 = state.universe.bodies[j].pos;
                ctx.draw(&Line {
                    x1: p1.x as f64,
                    y1: p1.y as f64,
                    x2: p2.x as f64,
                    y2: p2.y as f64,
                    color: Color::Gray,
                });
            }

            // Draw Bodies
            for (id, body) in state.universe.bodies.iter().enumerate() {
                let color = if state.triggered_bodies.contains(&id) {
                    Color::White // Flash
                } else {
                    body.color
                };

                ctx.draw(&Rectangle {
                    x: body.pos.x as f64 - body.radius as f64,
                    y: body.pos.y as f64 - body.radius as f64,
                    width: body.radius as f64 * 2.0,
                    height: body.radius as f64 * 2.0,
                    color,
                });
            }

            // Draw Playhead
            ctx.draw(&Line {
                x1: -100.0,
                y1: state.playhead_y as f64,
                x2: 100.0,
                y2: state.playhead_y as f64,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let info = Paragraph::new(format!(
        "Cords: {} | Bodies: {} | Playhead Y: {:.1} | Speed: {:.1}",
        state.cords.len(),
        state.universe.bodies.len(),
        state.playhead_y,
        state.playhead_dir
    ))
    .style(Style::default().fg(Color::Gray))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
