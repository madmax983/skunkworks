pub mod model;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    widgets::canvas::{Canvas, Line, Circle},
    style::Color,
    Frame,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tui_shared::Tui;
use crate::model::*;

fn main() -> Result<()> {
    // Check for semantic flag (primitive check before proper arg parsing)
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--semantic".to_string()) {
        let network = Network::new(10);
        let json = serde_json::to_string_pretty(&network)?;
        println!("{}", json);
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut network = Network::new(10);

    // Arrange in a circle for better visibility
    let count = network.agents.len();
    for (i, agent) in network.agents.iter_mut().enumerate() {
        let angle = i as f64 * 2.0 * std::f64::consts::PI / count as f64;
        agent.x = angle.cos() * 80.0;
        agent.y = angle.sin() * 80.0;
    }

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    loop {
        tui.terminal.draw(|f| draw(f, &network))?;

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
            network.step();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn draw(f: &mut Frame, network: &Network) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Network"))
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Draw links
            for (u, v, success) in &network.active_links {
                if let (Some(ag_u), Some(ag_v)) = (network.agents.get(*u), network.agents.get(*v)) {
                    let color = if *success { Color::Green } else { Color::Red };
                    ctx.draw(&Line {
                        x1: ag_u.x,
                        y1: ag_u.y,
                        x2: ag_v.x,
                        y2: ag_v.y,
                        color,
                    });
                }
            }

            // Draw agents
            for agent in &network.agents {
                ctx.draw(&Circle {
                    x: agent.x,
                    y: agent.y,
                    radius: 3.0,
                    color: Color::Blue,
                });
                let label = format!("{}", agent.id);
                ctx.print(agent.x, agent.y, label);
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Stats
    let mut stats_text = String::new();
    stats_text.push_str(&format!("Epoch: {}\n\n", network.epoch));

    // Calculate dominant words
    let mut counts: HashMap<Meaning, HashMap<String, usize>> = HashMap::new();

    for agent in &network.agents {
        for (m, w) in &agent.lexicon.map {
            *counts.entry(*m).or_default().entry(w.clone()).or_default() += 1;
        }
    }

    for m in Meaning::iter() {
        stats_text.push_str(&format!("{:?}:\n", m));
        if let Some(word_counts) = counts.get(&m) {
            let mut sorted: Vec<_> = word_counts.iter().collect();
            sorted.sort_by_key(|(_, c)| std::cmp::Reverse(**c));
            for (w, c) in sorted.into_iter().take(3) {
                 stats_text.push_str(&format!("  {}: {}\n", w, c));
            }
        }
        stats_text.push('\n');
    }

    let p = Paragraph::new(stats_text).block(Block::default().title("Pidgin Stats").borders(Borders::ALL));
    f.render_widget(p, chunks[1]);
}
