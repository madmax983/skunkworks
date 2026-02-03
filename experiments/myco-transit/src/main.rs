mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
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
    let mut tui = Tui::init()?;

    // Simulation parameters
    let width = 160;
    let height = 100;
    let num_cities = 8;
    let (mut world, mut agents) = World::with_cities_and_agents(width, height, num_cities);

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

    // Bolt Optimization: Hoist vector allocations out of the loop to reuse memory.
    // Reduces ~11 vector allocations/deallocations per frame (10,000+ items).
    let mut agent_groups: Vec<Vec<(f64, f64)>> =
        (0..num_cities).map(|_| Vec::with_capacity(1500)).collect();
    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        // Pre-process render data to avoid cloning inside closure or lifetime issues
        // 1. Agents grouped by target city (for color)
        for group in &mut agent_groups {
            group.clear();
        }
        for agent in &agents {
            if agent.target_city < num_cities {
                agent_groups[agent.target_city].push((agent.x, agent.y));
            }
        }

        // 2. Trails grouped by intensity (Low, Med, High)
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
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Myco-Transit: Slime Mold Urban Planning"),
                )
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
                        color: Color::White, // Strong paths are white
                    });

                    // Draw Cities
                    for (i, (cx, cy)) in world.cities.iter().enumerate() {
                        let color = city_colors[i % city_colors.len()];
                        // City Center
                        ctx.draw(&Points {
                            coords: &[(*cx, *cy)],
                            color,
                        });
                        // City Marker (Square)
                        ctx.draw(&Points {
                            coords: &[
                                (*cx + 1.0, *cy),
                                (*cx - 1.0, *cy),
                                (*cx, *cy + 1.0),
                                (*cx, *cy - 1.0),
                                (*cx + 1.0, *cy + 1.0),
                                (*cx - 1.0, *cy - 1.0),
                                (*cx + 1.0, *cy - 1.0),
                                (*cx - 1.0, *cy + 1.0),
                            ],
                            color,
                        });
                    }

                    // Draw Agents
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
                Span::raw(" to quit. Agents: "),
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
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update simulation
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
