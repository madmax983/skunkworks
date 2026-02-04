mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points, Rectangle},
        Block, Borders,
    },
};
use simulation::World;
use std::time::{Duration, Instant};
use tui_shared::Tui;
use rand::Rng;
use rand::prelude::SliceRandom;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Simulation params
    let width = 120;
    let height = 60;
    let mut world = World::new(width, height);

    // Initial Setup: Random Nodes
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let x = rng.gen_range(5..width - 5);
        let y = rng.gen_range(5..height - 5);
        // Half capacity, half load
        world.add_node(x, y, 100.0, rng.gen_range(0.0..150.0));
    }

    let tick_rate = Duration::from_millis(33); // 30 FPS
    let mut last_tick = Instant::now();

    loop {
        // Render
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            // Prepare visualization data
            // 1. Trails (Thresholded for performance/clarity)
            let mut trails_low = Vec::new();
            let mut trails_med = Vec::new();
            let mut trails_high = Vec::new();

            for y in 0..world.grid.height {
                for x in 0..world.grid.width {
                    let v = world.grid.trail_pheromone[y * world.grid.width + x];
                    if v > 50.0 {
                        trails_high.push((x as f64, y as f64));
                    } else if v > 20.0 {
                        trails_med.push((x as f64, y as f64));
                    } else if v > 5.0 {
                        trails_low.push((x as f64, y as f64));
                    }
                }
            }

            // 2. Agents
            let agent_points: Vec<(f64, f64)> = world.agents.iter().map(|a| (a.x, a.y)).collect();

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Fungal Balancer: Load Distribution"))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    // Draw Trails
                    ctx.draw(&Points {
                        coords: &trails_low,
                        color: Color::DarkGray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_med,
                        color: Color::Gray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_high,
                        color: Color::White,
                    });

                    // Draw Agents
                    ctx.draw(&Points {
                        coords: &agent_points,
                        color: Color::Yellow,
                    });

                    // Draw Nodes
                    for node in &world.nodes {
                        // Color based on Load Ratio
                        let ratio = node.load / node.capacity;
                        let color = if ratio > 1.0 {
                            Color::Red
                        } else if ratio > 0.8 {
                            Color::Magenta
                        } else if ratio > 0.5 {
                            Color::Yellow
                        } else {
                            Color::Green
                        };

                        // Draw larger rect for node
                        ctx.draw(&Rectangle {
                            x: node.x as f64 - 1.0,
                            y: node.y as f64 - 1.0,
                            width: 3.0,
                            height: 3.0,
                            color,
                        });
                    }
                });

            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let total_load: f32 = world.nodes.iter().map(|n| n.load).sum();
            let total_agents = world.agents.len();
            let status = Line::from(vec![
                Span::raw(" [q] Quit | [r] Reset | [s] Spike Load | "),
                Span::styled(format!("Agents: {}", total_agents), Style::default().fg(Color::Cyan)),
                Span::raw(" | "),
                Span::styled(format!("Total Load: {:.0}", total_load), Style::default().fg(Color::Green)),
            ]);
            f.render_widget(status, chunks[1]);
        })?;

        // Input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => {
                             world = World::new(width, height);
                             for _ in 0..10 {
                                let x = rng.gen_range(5..width - 5);
                                let y = rng.gen_range(5..height - 5);
                                world.add_node(x, y, 100.0, rng.gen_range(0.0..150.0));
                             }
                        }
                        KeyCode::Char('s') => {
                            // Spike load on random node
                            if let Some(node) = world.nodes.choose_mut(&mut rng) {
                                node.load += 200.0;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update
        if last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
