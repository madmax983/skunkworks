use heap_auction::model::System;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
    Frame,
};

pub fn draw(f: &mut Frame, system: &System) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Memory Grid (Top)
    let mut spans = Vec::new();
    for block in &system.memory {
        let color = match block.owner {
            Some(id) => Color::Indexed(((id.0 % 6) + 10) as u8), // Simple color cycle
            None => Color::DarkGray,
        };
        // Price determines character intensity
        let char = if block.price > 20.0 {
            "█"
        } else if block.price > 10.0 {
            "▓"
        } else {
            "▒"
        };

        spans.push(Span::styled(char, Style::default().fg(color)));
    }

    let memory_paragraph = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Heap Memory Auction"),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(memory_paragraph, chunks[0]);

    // Bottom Split (Stats)
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Price History
    let prices: Vec<u64> = system
        .market
        .history
        .iter()
        .map(|tx| tx.price as u64)
        .collect();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Market Price History"),
        )
        .data(&prices)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, bottom_chunks[0]);

    // Agent List
    let mut agents = system.agents.clone();
    agents.sort_by(|a, b| b.balance.partial_cmp(&a.balance).unwrap());

    let items: Vec<ListItem> = agents
        .iter()
        .take(10)
        .map(|a| {
            ListItem::new(format!(
                "Agent {}: ${:.2} ({:?}) [{} blocks]",
                a.id.0,
                a.balance,
                a.strategy,
                a.owned_blocks.len()
            ))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Top Agents (Wealth)"),
    );
    f.render_widget(list, bottom_chunks[1]);
}
