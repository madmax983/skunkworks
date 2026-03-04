mod physics;
mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use glam::Vec2;
use physics::PendulumSystem;
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

    // In chaos-mycelium we don't need fixed cities for agents, maybe the pendulum tip is the only food source?
    // Let's create one city which is the pendulum tip.
    let num_cities = 1;
    let (mut world, mut agents) = World::with_cities_and_agents(width, height, num_cities);

    // Create Chaos Pendulum
    let mut pendulum = PendulumSystem::new();
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let root = pendulum.add_node(Vec2::new(cx, cy), 1.0, true, "root".to_string());
    let node1 = pendulum.add_node(Vec2::new(cx + 20.0, cy), 1.0, false, "joint".to_string());
    let node2 = pendulum.add_node(Vec2::new(cx + 40.0, cy), 1.0, false, "tip".to_string());

    pendulum.add_link(root, node1, 20.0);
    pendulum.add_link(node1, node2, 20.0);

    // Give it an initial kick to make it chaotic
    pendulum.nodes[node2].pos.y += 0.1;

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    let mut agent_groups: Vec<Vec<(f64, f64)>> =
        (0..num_cities).map(|_| Vec::with_capacity(1500)).collect();
    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        // Pre-process render data
        for group in &mut agent_groups {
            group.clear();
        }
        for agent in &agents {
            if agent.target_city < num_cities {
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

        let agent_count = agents.len();

        let node1_pos = pendulum.nodes[node1].pos;
        let node2_pos = pendulum.nodes[node2].pos;

        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Chaos-Mycelium: Slime Mold chasing Chaos"),
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
                        color: Color::White,
                    });

                    // Draw the pendulum links (simplistic using points)
                    // (Interpolating between points just for visual)
                    for i in 0..20 {
                        let t = i as f32 / 20.0;
                        let px = cx + (node1_pos.x - cx) * t;
                        let py = cy + (node1_pos.y - cy) * t;
                        ctx.draw(&Points {
                            coords: &[(px as f64, py as f64)],
                            color: Color::Red,
                        });

                        let px2 = node1_pos.x + (node2_pos.x - node1_pos.x) * t;
                        let py2 = node1_pos.y + (node2_pos.y - node1_pos.y) * t;
                        ctx.draw(&Points {
                            coords: &[(px2 as f64, py2 as f64)],
                            color: Color::Red,
                        });
                    }

                    // Draw Pendulum nodes
                    ctx.draw(&Points {
                        coords: &[
                            (cx as f64, cy as f64),
                            (node1_pos.x as f64, node1_pos.y as f64),
                            (node2_pos.x as f64, node2_pos.y as f64),
                        ],
                        color: Color::Yellow,
                    });

                    // Draw Agents
                    for group in agent_groups.iter() {
                        if !group.is_empty() {
                            ctx.draw(&Points {
                                coords: group,
                                color: Color::Cyan, // all agents cyan
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
            // Update pendulum
            pendulum.step(0.016);

            // Pendulum leaves massive trail of food
            let tip_x = pendulum.nodes[node2].pos.x.clamp(0.0, (width - 1) as f32) as usize;
            let tip_y = pendulum.nodes[node2].pos.y.clamp(0.0, (height - 1) as f32) as usize;

            // Set intense pheromone at tip and surrounding
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let px = tip_x as isize + dx;
                    let py = tip_y as isize + dy;
                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        world.trails[py as usize * width + px as usize] = 255.0;
                    }
                }
            }

            // Also move the "city" so agents re-target
            world.cities[0] = (tip_x as f64, tip_y as f64);

            // Update simulation
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
