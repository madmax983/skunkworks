mod git;
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
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Fetch Git Data
    let commits = git::get_commit_history()?;

    // 2. Determine "Cities" (Top 8 most modified files)
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    for commit in &commits {
        for file in &commit.files_changed {
            *file_counts.entry(file.clone()).or_insert(0) += 1;
        }
    }

    let mut sorted_files: Vec<_> = file_counts.into_iter().collect();
    sorted_files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending

    let top_files = sorted_files
        .into_iter()
        .take(8)
        .map(|(f, _)| f)
        .collect::<Vec<_>>();

    if top_files.len() < 2 {
        println!("Not enough git history to form a mycelial network. Need at least 2 highly modified files.");
        return Ok(());
    }

    let width = 160;
    let height = 100;

    // Arrange cities in a circle
    let mut city_positions = Vec::new();
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;
    let radius = (height as f64 * 0.4).min(width as f64 * 0.4);

    for (i, file) in top_files.iter().enumerate() {
        let angle = (i as f64 / top_files.len() as f64) * 2.0 * std::f64::consts::PI;
        let x = center_x + angle.cos() * radius;
        let y = center_y + angle.sin() * radius;
        city_positions.push((file.clone(), (x, y)));
    }

    // 3. Determine "Connections" (Files changed together in the same commit)
    let mut connections = Vec::new();
    for commit in &commits {
        // Only look at files that are in our "cities" list
        let mut indices = Vec::new();
        for file in &commit.files_changed {
            if let Some(pos) = top_files.iter().position(|f| f == file) {
                indices.push(pos);
            }
        }

        // Add bidirectional connections between all pairs in this commit
        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                connections.push((indices[i], indices[j]));
                connections.push((indices[j], indices[i]));
            }
        }
    }

    let mut tui = Tui::init()?;

    let (mut world, mut agents) =
        World::with_git_cities_and_agents(width, height, city_positions, connections);

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

    let mut agent_groups: Vec<Vec<(f64, f64)>> = (0..world.cities.len())
        .map(|_| Vec::with_capacity(1500))
        .collect();
    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        for group in &mut agent_groups {
            group.clear();
        }
        for agent in &agents {
            if agent.target_city < world.cities.len() {
                agent_groups[agent.target_city].push((agent.x, agent.y));
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
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Git Mycelium: Slime Mold Codebase Architecture"),
                )
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

                    for (i, (cx, cy)) in world.cities.iter().enumerate() {
                        let color = city_colors[i % city_colors.len()];
                        // City Center
                        ctx.draw(&Points {
                            coords: &[(*cx, *cy)],
                            color,
                        });
                        // Print city name label text
                        ctx.print(
                            *cx + 2.0,
                            *cy,
                            Line::from(world.city_names[i].clone())
                                .style(Style::default().fg(color)),
                        );

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
                Span::raw(" to quit. Commits parsed: "),
                Span::styled(
                    format!("{}", commits.len()),
                    Style::default().fg(Color::Magenta),
                ),
                Span::raw(" | Agents: "),
                Span::styled(format!("{}", agent_count), Style::default().fg(Color::Cyan)),
                Span::raw(" | Connections formed: "),
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
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
