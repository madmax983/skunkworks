mod physics;
mod platter;
mod genome;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
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
use quipu::{Cord, Knot};
use crate::physics::{Body, Universe};
use quipu_serializer::ser::to_quipu;

struct AppState {
    universe: Universe,
    playhead_y: f32, // -100 to 100 (Physical Y)
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

        // Generate Random DNA
        let dna = genome::generate_random_dna(5, 20); // 5 strands, up to 20 genes

        // Serialize to Quipu
        let quipu = match to_quipu(&dna) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                return;
            }
        };

        self.cords = quipu.cords;

        let num_cords = self.cords.len();
        let spacing = 160.0 / (num_cords as f32 + 1.0);
        let start_x = -80.0 + spacing;

        for (i, cord) in self.cords.iter().enumerate() {
            let x = start_x + (i as f32 * spacing);
            let anchor_y = 80.0;

            // Anchor
            let anchor = Body::new(x, anchor_y, 1000.0, 2.0, 0.0, Color::White).fixed();
            let mut prev_id = self.universe.add_body(anchor);

            // Knots
            // Iterate reverse (High powers/Top to Low powers/Bottom) to build chain downwards
            let mut depth = 1;
            for (_c_idx, cluster) in cord.clusters.iter().enumerate().rev() {
                if cluster.is_empty() { continue; }

                let mut total_mass = 0.0;
                let mut total_charge = 0.0;
                let mut knot_count = 0;

                for knot in cluster {
                    match knot {
                        Knot::Simple => {
                            total_mass += 1.0;
                            total_charge += 1.0; // Positive Charge
                        },
                        Knot::Long(v) => {
                            total_mass += *v as f32;
                            total_charge -= *v as f32; // Negative Charge
                        },
                        Knot::FigureEight => {
                            total_mass += 1.5;
                            // Neutral charge
                        }
                    }
                    knot_count += 1;
                }

                if knot_count == 0 { continue; }

                let y = anchor_y - (depth as f32 * 15.0); // Initial position
                depth += 1;

                let jitter = (rand::random::<f32>() - 0.5) * 5.0;

                // Map quipu::Color to ratatui::style::Color
                let color = match cord.color {
                    quipu::Color::Natural => Color::Gray,
                    quipu::Color::Red => Color::Red,
                    quipu::Color::Green => Color::Green,
                    quipu::Color::Blue => Color::Blue,
                    quipu::Color::Yellow => Color::Yellow,
                    quipu::Color::Black => Color::DarkGray,
                    quipu::Color::White => Color::White,
                };

                let radius = 2.0 + (total_mass * 0.2).min(5.0);

                let body = Body::new(x + jitter, y, total_mass, radius, total_charge, color);
                let id = self.universe.add_body(body);

                self.universe.add_edge(prev_id, id);
                prev_id = id;
            }

            // Subsidiaries (One level deep)
            for sub in &cord.subsidiaries {
                 let sub_prev_id = prev_id; // Attach to end of main cord
                 let mut sub_prev = sub_prev_id;
                 let mut sub_depth = 1;

                 for cluster in sub.clusters.iter().enumerate().rev().map(|(_, c)| c) {
                     if cluster.is_empty() { continue; }
                     let mut total_mass = 0.0;
                     let mut total_charge = 0.0;
                     for k in cluster {
                         match k {
                             Knot::Simple => { total_mass += 1.0; total_charge += 1.0; }
                             Knot::Long(v) => { total_mass += *v as f32; total_charge -= *v as f32; }
                             Knot::FigureEight => { total_mass += 1.5; }
                         }
                     }

                     let parent_pos = self.universe.bodies[sub_prev].pos;
                     let y = parent_pos.y - 15.0;
                     let sub_x = parent_pos.x + (rand::random::<f32>() - 0.5) * 2.0;

                     let color = Color::Cyan; // Differentiate subsidiaries
                     let body = Body::new(sub_x, y, total_mass, 2.0, total_charge, color);
                     let id = self.universe.add_body(body);
                     self.universe.add_edge(sub_prev, id);
                     sub_prev = id;
                     sub_depth += 1;
                 }
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
            if body.fixed { continue; }

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
        .block(Block::default().borders(Borders::ALL).title("Ferrous Weaver | Magnetic DNA Loom"))
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Draw Platter (Magnetism)
            let step = 4;
            for y in (0..state.universe.platter.height).step_by(step) {
                for x in (0..state.universe.platter.width).step_by(step) {
                    let mag = state.universe.platter.get_magnetism(x, y);
                    if mag > 0.2 {
                        let px = x as f64 - 100.0;
                        let py = y as f64 - 100.0;
                        let color = if mag > 0.5 { Color::Red } else { Color::DarkGray };
                         ctx.print(px, py, ratatui::text::Span::styled(".", Style::default().fg(color)));
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
                let mut color = if state.triggered_bodies.contains(&id) {
                    Color::White // Flash
                } else {
                    body.color
                };

                // Visualize Charge with symbol or color shift?
                // Just keep it simple for now.

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

    let info = Paragraph::new(format!("Cords: {} | Bodies: {} | Playhead Y: {:.1} | Speed: {:.1}",
        state.cords.len(), state.universe.bodies.len(), state.playhead_y, state.playhead_dir))
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
