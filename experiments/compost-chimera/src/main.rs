mod compost;
mod decay;
mod simulation;

use crate::simulation::Simulation;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::Duration;
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Initialize Simulation
    let root = std::env::current_dir()?;
    let mut sim = Simulation::new(&root)?;

    let mut scroll_x = 0;
    let mut scroll_y = 0;

    loop {
        // Update
        sim.tick();

        // Draw
        tui.terminal.draw(|f| {
            draw_ui(f, &sim, scroll_x, scroll_y);
        })?;

        // Input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if scroll_y < sim.height.saturating_sub(10) {
                                scroll_y += 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            scroll_y = scroll_y.saturating_sub(1);
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            scroll_x += 1;
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            scroll_x = scroll_x.saturating_sub(1);
                        }
                        KeyCode::Tab => {
                            // Next file
                            let next = (sim.current_file_idx + 1) % sim.bin.files.len();
                            sim.load_file(next);
                            scroll_x = 0;
                            scroll_y = 0;
                        }
                        KeyCode::Char('r') => {
                            // Reload/Respawn
                            sim.spawn_agents(1);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn draw_ui(f: &mut Frame, sim: &Simulation, scroll_x: usize, scroll_y: usize) {
    let size = f.size();

    // Title Block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(" 🧬 Compost Chimera ");

    // Convert current content to Text with Overlay
    let mut lines = Vec::new();

    let view_height = (size.height as usize).saturating_sub(2);
    let view_width = (size.width as usize).saturating_sub(2);

    for y in scroll_y..std::cmp::min(scroll_y + view_height, sim.height) {
        let mut spans = Vec::new();
        if y < sim.current_content.len() {
            let row = &sim.current_content[y];

            // We need to construct the line char by char to handle colors/agents
            for x in scroll_x..std::cmp::min(scroll_x + view_width, sim.width) {
                // Check if agent is here
                let agents_here: Vec<_> =
                    sim.agents.iter().filter(|a| a.x == x && a.y == y).collect();

                if let Some(agent) = agents_here.first() {
                    spans.push(Span::styled(
                        "@",
                        Style::default().fg(Color::Red).bg(Color::Black),
                    ));
                } else {
                    let c = if x < row.len() { row[x] } else { ' ' };

                    // Color based on decay/glitch
                    let style = match c {
                        '░' | '▒' | '▓' | '█' => Style::default().fg(Color::DarkGray),
                        '?' | '!' | '@' | '#' => Style::default().fg(Color::Yellow),
                        ' ' => Style::default(),
                        _ => Style::default().fg(Color::Green),
                    };

                    spans.push(Span::styled(c.to_string(), style));
                }
            }
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).block(title_block).scroll((0, 0)); // We handled scrolling manually

    f.render_widget(paragraph, size);

    // Stats overlay
    let stats = format!(
        "File: {}/{} | Agents: {} | Scroll: {},{}",
        sim.current_file_idx + 1,
        sim.bin.files.len(),
        sim.agents.len(),
        scroll_x,
        scroll_y
    );
    let stats_widget =
        Paragraph::new(stats).style(Style::default().fg(Color::White).bg(Color::Blue));
    let stats_area = Rect::new(size.width.saturating_sub(50), 0, 48, 1);
    f.render_widget(stats_widget, stats_area);
}
