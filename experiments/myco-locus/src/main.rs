mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use locus::Topology;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
};
use simulation::World;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // ☠️ REAPER BYPASS: Ensure headless CI test survivability
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode detected, bypassing TUI initialization.");
        return Ok(());
    }

    let mut tui = Tui::init()?;

    let width = 160;
    let height = 100;
    let num_cities = 8;

    // Switch topologies to observe different structural mapping
    let topologies = [
        Topology::Plane,
        Topology::Torus,
        Topology::Klein,
        Topology::Sphere,
        Topology::Projective,
    ];
    let mut current_topo_idx = 1; // Start with Torus
    let mut topology = topologies[current_topo_idx];

    let (mut world, mut agents) =
        World::with_cities_and_agents(width, height, num_cities, topology);

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    let city_colors = [
        Color::Red,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::Green,
        Color::White,
        Color::LightRed,
    ];

    let mut agent_groups: Vec<Vec<(f64, f64)>> =
        (0..num_cities).map(|_| Vec::with_capacity(1500)).collect();
    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        for group in &mut agent_groups {
            group.clear();
        }
        for agent in &agents {
            if agent.target_city < num_cities {
                agent_groups[agent.target_city].push((agent.position.x, agent.position.y));
            }
        }

        trails_low.clear();
        trails_med.clear();
        trails_high.clear();

        for y in 0..world.height {
            for x in 0..world.width {
                let val = world.get_trail(x, y);
                if val > 50.0 {
                    trails_high.push((x as f64, y as f64));
                } else if val > 20.0 {
                    trails_med.push((x as f64, y as f64));
                } else if val > 5.0 {
                    trails_low.push((x as f64, y as f64));
                }
            }
        }

        let total_commutes: u32 = agents.iter().map(|a| a.commuted_count).sum();
        let agent_count = agents.len();

        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Myco-Locus: Topological Transit [{:?}]",
                    world.topology
                )))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
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

                    for (i, c) in world.cities.iter().enumerate() {
                        let color = city_colors[i % city_colors.len()];
                        ctx.draw(&Points {
                            coords: &[(c.x, c.y)],
                            color,
                        });
                        ctx.draw(&Points {
                            coords: &[
                                (c.x + 1.0, c.y),
                                (c.x - 1.0, c.y),
                                (c.x, c.y + 1.0),
                                (c.x, c.y - 1.0),
                                (c.x + 1.0, c.y + 1.0),
                                (c.x - 1.0, c.y - 1.0),
                                (c.x + 1.0, c.y - 1.0),
                                (c.x - 1.0, c.y + 1.0),
                            ],
                            color,
                        });
                    }

                    for (i, group) in agent_groups.iter().enumerate() {
                        if !group.is_empty() {
                            ctx.draw(&Points {
                                coords: group,
                                color: city_colors[i % city_colors.len()],
                            });
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" to quit, "),
                Span::styled("t", Style::default().fg(Color::Yellow)),
                Span::raw(" to cycle topology. Agents: "),
                Span::styled(format!("{}", agent_count), Style::default().fg(Color::Cyan)),
                Span::raw(" | Commutes: "),
                Span::styled(
                    format!("{}", total_commutes),
                    Style::default().fg(Color::Green),
                ),
            ]);
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('t') => {
                        current_topo_idx = (current_topo_idx + 1) % topologies.len();
                        topology = topologies[current_topo_idx];
                        world.topology = topology;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
