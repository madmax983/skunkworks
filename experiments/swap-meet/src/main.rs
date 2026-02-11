use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use swap_meet::model::{Market, Agent};
use tui_shared::Tui;
use rand::Rng;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut market = Market::new(32, 32);

    // Seed initial agents
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        market.agents.push(Agent {
            id: i,
            wealth: rng.gen_range(50.0..500.0),
            color: Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
            desired_blocks: rng.gen_range(5..50),
            owned_blocks: Vec::new(),
        });
    }

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        // Event Handling
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        market = Market::new(32, 32);
                        for i in 0..10 {
                             market.agents.push(Agent {
                                id: i,
                                wealth: rng.gen_range(50.0..500.0),
                                color: Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
                                desired_blocks: rng.gen_range(5..50),
                                owned_blocks: Vec::new(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update
        if last_tick.elapsed() >= tick_rate {
            market.update();

            // Spawn new agents randomly if population drops
            if market.agents.len() < 5 {
                 let id = market.tick as usize + 100;
                 market.agents.push(Agent {
                    id,
                    wealth: rng.gen_range(50.0..200.0),
                    color: Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
                    desired_blocks: rng.gen_range(5..20),
                    owned_blocks: Vec::new(),
                });
            }

            last_tick = Instant::now();
        }

        // Draw
        tui.terminal.draw(|f| ui(f, &market))?;
    }

    Ok(())
}

fn ui(f: &mut Frame, market: &Market) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Render Grid
    let mut lines = Vec::new();
    for y in 0..market.height {
        let mut spans = Vec::new();
        for x in 0..market.width {
            let idx = y * market.width + x;
            let block = &market.grid[idx];

            let (char, style) = if let Some(owner_id) = block.owner {
                if let Some(agent) = market.agents.iter().find(|a| a.id == owner_id) {
                    ("██", Style::default().fg(agent.color))
                } else {
                    ("??", Style::default().fg(Color::Red)) // Should not happen
                }
            } else {
                // Heatmap for rent
                let intensity = (block.rent / 5.0 * 255.0).clamp(0.0, 255.0) as u8;
                ("..", Style::default().fg(Color::Rgb(intensity, intensity, intensity)))
            };

            spans.push(Span::styled(char, style));
        }
        lines.push(Line::from(spans));
    }

    let grid_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Memory Map (Rent Heatmap)"));
    f.render_widget(grid_widget, chunks[0]);

    // Render Stats
    let mut stats_text = Vec::new();
    stats_text.push(Line::from(vec![Span::raw(format!("Tick: {}", market.tick))]));
    stats_text.push(Line::from(vec![Span::raw(format!("Agents: {}", market.agents.len()))]));
    stats_text.push(Line::from(Span::raw("--- Top Agents ---")));

    // Sort agents by wealth for display (clone to sort)
    let mut sorted_agents = market.agents.clone();
    sorted_agents.sort_by(|a, b| b.wealth.partial_cmp(&a.wealth).unwrap_or(std::cmp::Ordering::Equal));

    for agent in sorted_agents.iter().take(15) {
        stats_text.push(Line::from(vec![
            Span::styled("█ ", Style::default().fg(agent.color)),
            Span::raw(format!("ID: {:<3} | ${:<6.1} | Blks: {}/{}",
                agent.id, agent.wealth, agent.owned_blocks.len(), agent.desired_blocks)),
        ]));
    }

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Market Data"));
    f.render_widget(stats_widget, chunks[1]);
}
